use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: Decimal,
    pub category: String,
    pub description: Option<String>,
    pub transaction_type: String,
    pub icon: Option<String>,
    pub date: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct TransactionInput {
    #[validate(range(min = 0.01, message = "Amount must be positive"))]
    pub amount: f64,
    #[validate(length(min = 1, message = "Category is required"))]
    pub category: String,
    pub description: Option<String>,
    #[validate(custom(function = "validate_transaction_type"))]
    pub transaction_type: String,
    pub icon: Option<String>,
    pub date: Option<String>,
}

fn validate_transaction_type(value: &str) -> Result<(), validator::ValidationError> {
    if value == "income" || value == "expense" {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid_type"))
    }
}

#[derive(Debug, Deserialize)]
pub struct TransactionQuery {
    pub search: Option<String>,
    pub r#type: Option<String>,
    pub month: Option<i32>,
    pub year: Option<i32>,
}
