use crate::models::budget::Budget;
use chrono::Datelike;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn find_all(
    pool: &PgPool,
    user_id: Uuid,
    month: Option<i32>,
    year: Option<i32>,
) -> Result<Vec<Budget>, sqlx::Error> {
    let now = chrono::Utc::now();
    let m = month.unwrap_or(now.month() as i32);
    let y = year.unwrap_or(now.year());

    sqlx::query_as::<_, Budget>(
        "SELECT * FROM budgets WHERE user_id = $1 AND month = $2 AND year = $3 ORDER BY category",
    )
    .bind(user_id)
    .bind(m)
    .bind(y)
    .fetch_all(pool)
    .await
}

pub async fn upsert(
    pool: &PgPool,
    user_id: Uuid,
    category: &str,
    limit_amount: Decimal,
    icon: Option<&str>,
    month: i32,
    year: i32,
) -> Result<Budget, sqlx::Error> {
    sqlx::query_as::<_, Budget>(
        "INSERT INTO budgets (user_id, category, limit_amount, icon, month, year)
         VALUES ($1, $2, $3, $4, $5, $6)
         ON CONFLICT (user_id, category, month, year) 
         DO UPDATE SET limit_amount = $3, icon = COALESCE($4, budgets.icon), updated_at = NOW()
         RETURNING *",
    )
    .bind(user_id)
    .bind(category)
    .bind(limit_amount)
    .bind(icon)
    .bind(month)
    .bind(year)
    .fetch_one(pool)
    .await
}

pub async fn find_by_id(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> Result<Option<Budget>, sqlx::Error> {
    sqlx::query_as::<_, Budget>("SELECT * FROM budgets WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user_id)
        .fetch_optional(pool)
        .await
}

pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM budgets WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

pub async fn recalculate_spent(
    pool: &PgPool,
    user_id: Uuid,
    category: &str,
    month: i32,
    year: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE budgets
         SET spent_amount = COALESCE((
             SELECT SUM(amount) FROM transactions
             WHERE user_id = $1
               AND LOWER(category) = LOWER($2)
               AND transaction_type = 'expense'
               AND EXTRACT(MONTH FROM date) = $5
               AND EXTRACT(YEAR FROM date) = $6
         ), 0),
         updated_at = NOW()
         WHERE user_id = $1 AND LOWER(category) = LOWER($2) AND month = $3 AND year = $4",
    )
    .bind(user_id)
    .bind(category)
    .bind(month)
    .bind(year)
    .bind(month as f64)
    .bind(year as f64)
    .execute(pool)
    .await?;
    Ok(())
}
