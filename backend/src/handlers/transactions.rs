use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    Json,
};
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    errors::AppError,
    middleware::auth::AuthUser,
    models::transaction::{TransactionInput, TransactionQuery},
    repositories::transaction as tx_repo,
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

    let tx = tx_repo::create(
        &pool,
        auth.user_id,
        amount,
        &input.category,
        input.description.as_deref(),
        &input.transaction_type,
        input.icon.as_deref(),
        date,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(tx)))
}

pub async fn update(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(input): Json<TransactionInput>,
) -> Result<Json<crate::models::transaction::Transaction>, AppError> {
    input.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Check ownership
    tx_repo::find_by_id(&pool, id, auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Transaction not found".into()))?;

    let amount = Decimal::try_from(input.amount)
        .map_err(|_| AppError::BadRequest("Invalid amount".into()))?;

    let date = input.date.as_ref().and_then(|d| {
        chrono::DateTime::parse_from_rfc3339(d).ok().map(|dt| dt.with_timezone(&chrono::Utc))
    });

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
    )
    .await?;

    Ok(Json(tx))
}

pub async fn delete(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    tx_repo::find_by_id(&pool, id, auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Transaction not found".into()))?;

    tx_repo::delete(&pool, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
