use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    Json,
};
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use chrono::Datelike;

use crate::{
    errors::AppError,
    middleware::auth::AuthUser,
    models::transaction::{TransactionInput, TransactionQuery},
    repositories::{budget as budget_repo, transaction as tx_repo, wallet as wallet_repo},
};

pub async fn list(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
    Query(query): Query<TransactionQuery>,
) -> Result<Json<Vec<crate::models::transaction::Transaction>>, AppError> {
    let transactions = tx_repo::find_all(&pool, auth.user_id, &query).await?;
    Ok(Json(transactions))
}

pub async fn get_one(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::models::transaction::Transaction>, AppError> {
    let tx = tx_repo::find_by_id(&pool, id, auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Transaction not found".into()))?;
    Ok(Json(tx))
}

pub async fn create(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
    Json(input): Json<TransactionInput>,
) -> Result<(StatusCode, Json<crate::models::transaction::Transaction>), AppError> {
    input.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;

    let amount = Decimal::try_from(input.amount)
        .map_err(|_| AppError::BadRequest("Invalid amount".into()))?;

    let date = input.date.as_ref().and_then(|d| {
        chrono::DateTime::parse_from_rfc3339(d).ok().map(|dt| dt.with_timezone(&chrono::Utc))
    });

    let wallet_id = input.wallet_id.as_ref().and_then(|id| Uuid::parse_str(id).ok());

    let tx = tx_repo::create(
        &pool,
        auth.user_id,
        amount,
        &input.category,
        input.description.as_deref(),
        &input.transaction_type,
        input.icon.as_deref(),
        date,
        wallet_id,
    )
    .await?;

    // Update budget spent_amount
    let tx_date = tx.date.unwrap_or_else(chrono::Utc::now);
    let _ = budget_repo::recalculate_spent(
        &pool, auth.user_id, &tx.category, tx_date.month() as i32, tx_date.year(),
    ).await;

    // Update wallet balance
    if let Some(wid) = tx.wallet_id {
        let delta = if tx.transaction_type == "income" { amount } else { -amount };
        let _ = wallet_repo::apply_transaction_delta(&pool, wid, auth.user_id, delta).await;
    }

    Ok((StatusCode::CREATED, Json(tx)))
}

pub async fn update(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(input): Json<TransactionInput>,
) -> Result<Json<crate::models::transaction::Transaction>, AppError> {
    input.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Fetch old transaction for reversal
    let old_tx = tx_repo::find_by_id(&pool, id, auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Transaction not found".into()))?;

    let amount = Decimal::try_from(input.amount)
        .map_err(|_| AppError::BadRequest("Invalid amount".into()))?;

    let date = input.date.as_ref().and_then(|d| {
        chrono::DateTime::parse_from_rfc3339(d).ok().map(|dt| dt.with_timezone(&chrono::Utc))
    });

    let wallet_id = input.wallet_id.as_ref().and_then(|id| Uuid::parse_str(id).ok());

    let tx = tx_repo::update(
        &pool,
        id,
        auth.user_id,
        amount,
        &input.category,
        input.description.as_deref(),
        &input.transaction_type,
        input.icon.as_deref(),
        date,
        wallet_id,
    )
    .await?;

    // Recalculate budget for old category/date
    let old_date = old_tx.date.unwrap_or_else(chrono::Utc::now);
    let _ = budget_repo::recalculate_spent(
        &pool, auth.user_id, &old_tx.category, old_date.month() as i32, old_date.year(),
    ).await;

    // Recalculate budget for new category/date if changed
    let new_date = tx.date.unwrap_or_else(chrono::Utc::now);
    if tx.category != old_tx.category
        || new_date.month() != old_date.month()
        || new_date.year() != old_date.year()
    {
        let _ = budget_repo::recalculate_spent(
            &pool, auth.user_id, &tx.category, new_date.month() as i32, new_date.year(),
        ).await;
    }

    // Reverse old wallet effect
    if let Some(old_wid) = old_tx.wallet_id {
        let old_amount = old_tx.amount;
        let reverse = if old_tx.transaction_type == "income" { -old_amount } else { old_amount };
        let _ = wallet_repo::apply_transaction_delta(&pool, old_wid, auth.user_id, reverse).await;
    }

    // Apply new wallet effect
    if let Some(new_wid) = tx.wallet_id {
        let delta = if tx.transaction_type == "income" { amount } else { -amount };
        let _ = wallet_repo::apply_transaction_delta(&pool, new_wid, auth.user_id, delta).await;
    }

    Ok(Json(tx))
}

pub async fn delete(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let old_tx = tx_repo::find_by_id(&pool, id, auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Transaction not found".into()))?;

    tx_repo::delete(&pool, id, auth.user_id).await?;

    // Recalculate budget after delete
    let old_date = old_tx.date.unwrap_or_else(chrono::Utc::now);
    let _ = budget_repo::recalculate_spent(
        &pool, auth.user_id, &old_tx.category, old_date.month() as i32, old_date.year(),
    ).await;

    // Reverse wallet effect
    if let Some(wid) = old_tx.wallet_id {
        let old_amount = old_tx.amount;
        let reverse = if old_tx.transaction_type == "income" { -old_amount } else { old_amount };
        let _ = wallet_repo::apply_transaction_delta(&pool, wid, auth.user_id, reverse).await;
    }

    Ok(StatusCode::NO_CONTENT)
}
