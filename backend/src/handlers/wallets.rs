use axum::{
    extract::{Extension, Path},
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
    models::wallet::{AdjustBalanceInput, WalletInput},
    repositories::wallet as wallet_repo,
};

pub async fn list(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<crate::models::wallet::Wallet>>, AppError> {
    let wallets = wallet_repo::find_all(&pool, auth.user_id).await?;
    Ok(Json(wallets))
}

pub async fn create(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
    Json(input): Json<WalletInput>,
) -> Result<(StatusCode, Json<crate::models::wallet::Wallet>), AppError> {
    input.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;

    let balance = Decimal::try_from(input.balance)
        .map_err(|_| AppError::BadRequest("Invalid balance".into()))?;

    let wallet = wallet_repo::create(
        &pool,
        auth.user_id,
        &input.name,
        &input.wallet_type,
        balance,
        input.icon.as_deref(),
        input.color.as_deref(),
    )
    .await?;

    Ok((StatusCode::CREATED, Json(wallet)))
}

pub async fn update(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(input): Json<WalletInput>,
) -> Result<Json<crate::models::wallet::Wallet>, AppError> {
    input.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;

    wallet_repo::find_by_id(&pool, id, auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Wallet not found".into()))?;

    let balance = Decimal::try_from(input.balance)
        .map_err(|_| AppError::BadRequest("Invalid balance".into()))?;

    let wallet = wallet_repo::update(
        &pool,
        id,
        auth.user_id,
        &input.name,
        &input.wallet_type,
        balance,
        input.icon.as_deref(),
        input.color.as_deref(),
    )
    .await?;

    Ok(Json(wallet))
}

pub async fn adjust_balance(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(input): Json<AdjustBalanceInput>,
) -> Result<Json<crate::models::wallet::Wallet>, AppError> {
    wallet_repo::find_by_id(&pool, id, auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Wallet not found".into()))?;

    if input.balance < 0.0 {
        return Err(AppError::BadRequest("Balance cannot be negative".into()));
    }

    let balance = Decimal::try_from(input.balance)
        .map_err(|_| AppError::BadRequest("Invalid balance".into()))?;

    let wallet = wallet_repo::adjust_balance(&pool, id, auth.user_id, balance).await?;
    Ok(Json(wallet))
}

pub async fn delete(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    wallet_repo::find_by_id(&pool, id, auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Wallet not found".into()))?;

    wallet_repo::delete(&pool, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
