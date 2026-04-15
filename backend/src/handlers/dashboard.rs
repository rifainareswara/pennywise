use axum::{extract::Extension, Json};
use serde::Serialize;
use sqlx::PgPool;
use rust_decimal::Decimal;

use crate::{
    errors::AppError,
    middleware::auth::AuthUser,
    models::transaction::Transaction,
    repositories::transaction as tx_repo,
};

#[derive(Debug, Serialize)]
pub struct DashboardSummary {
    pub total_balance: String,
    pub total_income: String,
    pub total_expenses: String,
    pub balance_change_percent: f64,
    pub weekly_activity: Vec<String>,
    pub recent_transactions: Vec<Transaction>,
}

pub async fn summary(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<DashboardSummary>, AppError> {
    let (income, expenses) = tx_repo::get_summary(&pool, auth.user_id).await?;
    let balance = income - expenses;

    let weekly = tx_repo::get_weekly_activity(&pool, auth.user_id).await?;
    let recent = tx_repo::get_recent(&pool, auth.user_id, 5).await?;

    // Simple change percent (placeholder — compare with previous month)
    let change_percent = if expenses > Decimal::ZERO {
        ((income - expenses).to_string().parse::<f64>().unwrap_or(0.0) /
            expenses.to_string().parse::<f64>().unwrap_or(1.0)) * 100.0
    } else {
        0.0
    };

    Ok(Json(DashboardSummary {
        total_balance: format!("{:.2}", balance),
        total_income: format!("{:.2}", income),
        total_expenses: format!("{:.2}", expenses),
        balance_change_percent: (change_percent * 10.0).round() / 10.0,
        weekly_activity: weekly.iter().map(|d| format!("{:.2}", d)).collect(),
        recent_transactions: recent,
    }))
}
