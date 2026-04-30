use axum::{extract::Extension, Json};
use serde::Serialize;
use sqlx::PgPool;
use rust_decimal::Decimal;

use crate::{
    errors::AppError,
    middleware::auth::AuthUser,
    models::transaction::Transaction,
    repositories::{transaction as tx_repo, wallet as wallet_repo},
};

#[derive(Debug, Serialize)]
pub struct DashboardSummary {
    pub total_balance: String,
    pub total_income: String,
    pub total_expenses: String,
    pub balance_change_percent: f64,
    pub weekly_income: Vec<f64>,
    pub weekly_expenses: Vec<f64>,
    pub avg_daily_income: f64,
    pub avg_daily_expense: f64,
    pub recent_transactions: Vec<Transaction>,
}

fn to_f64_vec(data: &[Decimal]) -> Vec<f64> {
    data.iter()
        .map(|d| d.to_string().parse::<f64>().unwrap_or(0.0))
        .collect()
}

fn daily_average(data: &[Decimal]) -> f64 {
    let active = data.iter().filter(|&&v| v > Decimal::ZERO).count();
    if active == 0 {
        return 0.0;
    }
    let sum: Decimal = data.iter().sum();
    (sum / Decimal::from(active as i64))
        .to_string()
        .parse()
        .unwrap_or(0.0)
}

pub async fn summary(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<DashboardSummary>, AppError> {
    let (income, expenses) = tx_repo::get_summary(&pool, auth.user_id).await?;

    // Use wallet total as balance if user has wallets, otherwise fall back to income - expenses
    let wallet_total = wallet_repo::get_total_balance(&pool, auth.user_id).await?;
    let balance = wallet_total.unwrap_or(income - expenses);

    let weekly_expense_data = tx_repo::get_weekly_activity(&pool, auth.user_id).await?;
    let weekly_income_data = tx_repo::get_weekly_income(&pool, auth.user_id).await?;
    let recent = tx_repo::get_recent(&pool, auth.user_id, 5).await?;

    let change_percent = if expenses > Decimal::ZERO {
        ((income - expenses).to_string().parse::<f64>().unwrap_or(0.0)
            / expenses.to_string().parse::<f64>().unwrap_or(1.0))
            * 100.0
    } else {
        0.0
    };

    Ok(Json(DashboardSummary {
        total_balance: format!("{:.2}", balance),
        total_income: format!("{:.2}", income),
        total_expenses: format!("{:.2}", expenses),
        balance_change_percent: (change_percent * 10.0).round() / 10.0,
        weekly_income: to_f64_vec(&weekly_income_data),
        weekly_expenses: to_f64_vec(&weekly_expense_data),
        avg_daily_income: daily_average(&weekly_income_data),
        avg_daily_expense: daily_average(&weekly_expense_data),
        recent_transactions: recent,
    }))
}
