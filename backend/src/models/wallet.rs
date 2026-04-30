use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Wallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub wallet_type: String,
    pub balance: Decimal,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct WalletInput {
    #[validate(length(min = 1, max = 100, message = "Name is required"))]
    pub name: String,
    pub wallet_type: String,
    #[validate(range(min = 0.0, message = "Balance cannot be negative"))]
    pub balance: f64,
    pub icon: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdjustBalanceInput {
    pub balance: f64,
}
