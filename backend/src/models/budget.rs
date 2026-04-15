use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Budget {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category: String,
    pub limit_amount: Decimal,
    pub spent_amount: Decimal,
    pub icon: Option<String>,
    pub month: i32,
    pub year: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct BudgetInput {
    #[validate(length(min = 1, message = "Category is required"))]
    pub category: String,
    #[validate(range(min = 0.01, message = "Limit must be positive"))]
    pub limit_amount: f64,
    pub icon: Option<String>,
    #[validate(range(min = 1, max = 12))]
    pub month: i32,
    #[validate(range(min = 2020, max = 2100))]
    pub year: i32,
}

#[derive(Debug, Deserialize)]
pub struct BudgetQuery {
    pub month: Option<i32>,
    pub year: Option<i32>,
}
