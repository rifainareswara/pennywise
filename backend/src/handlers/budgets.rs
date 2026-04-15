use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    Json,
};
use rust_decimal::Decimal;
use sqlx::PgPool;
use validator::Validate;

use crate::{
    errors::AppError,
    middleware::auth::AuthUser,
    models::budget::{BudgetInput, BudgetQuery},
    repositories::budget as budget_repo,
};

pub async fn list(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
    Query(query): Query<BudgetQuery>,
) -> Result<Json<Vec<crate::models::budget::Budget>>, AppError> {
    let budgets = budget_repo::find_all(&pool, auth.user_id, query.month, query.year).await?;
    Ok(Json(budgets))
}

pub async fn create(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
    Json(input): Json<BudgetInput>,
) -> Result<(StatusCode, Json<crate::models::budget::Budget>), AppError> {
    input.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;

    let limit_amount = Decimal::try_from(input.limit_amount)
        .map_err(|_| AppError::BadRequest("Invalid amount".into()))?;

    let budget = budget_repo::upsert(
        &pool,
        auth.user_id,
        &input.category,
        limit_amount,
        input.icon.as_deref(),
        input.month,
        input.year,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(budget)))
}

pub async fn delete(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> Result<StatusCode, AppError> {
    budget_repo::find_by_id(&pool, id, auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Budget not found".into()))?;

    budget_repo::delete(&pool, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
