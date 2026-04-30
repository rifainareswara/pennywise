use crate::models::transaction::{Transaction, TransactionQuery};
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn find_all(
    pool: &PgPool,
    user_id: Uuid,
    query: &TransactionQuery,
) -> Result<Vec<Transaction>, sqlx::Error> {
    let mut sql = String::from(
        "SELECT * FROM transactions WHERE user_id = $1",
    );
    let mut param_idx = 2u32;

    if query.search.is_some() {
        sql.push_str(&format!(
            " AND (description ILIKE ${0} OR category ILIKE ${0})",
            param_idx
        ));
        param_idx += 1;
    }
    if query.r#type.is_some() {
        sql.push_str(&format!(" AND transaction_type = ${}", param_idx));
        param_idx += 1;
    }
    if query.month.is_some() {
        sql.push_str(&format!(" AND EXTRACT(MONTH FROM date) = ${}", param_idx));
        param_idx += 1;
    }
    if query.year.is_some() {
        sql.push_str(&format!(" AND EXTRACT(YEAR FROM date) = ${}", param_idx));
        // param_idx += 1;
    }

    sql.push_str(" ORDER BY date DESC LIMIT 100");

    let mut q = sqlx::query_as::<_, Transaction>(&sql).bind(user_id);

    if let Some(ref search) = query.search {
        q = q.bind(format!("%{}%", search));
    }
    if let Some(ref t) = query.r#type {
        q = q.bind(t);
    }
    if let Some(month) = query.month {
        q = q.bind(month as f64);
    }
    if let Some(year) = query.year {
        q = q.bind(year as f64);
    }

    q.fetch_all(pool).await
}

pub async fn find_by_id(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> Result<Option<Transaction>, sqlx::Error> {
    sqlx::query_as::<_, Transaction>(
        "SELECT * FROM transactions WHERE id = $1 AND user_id = $2",
    )
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn create(
    pool: &PgPool,
    user_id: Uuid,
    amount: Decimal,
    category: &str,
    description: Option<&str>,
    transaction_type: &str,
    icon: Option<&str>,
    date: Option<chrono::DateTime<chrono::Utc>>,
    wallet_id: Option<Uuid>,
) -> Result<Transaction, sqlx::Error> {
    sqlx::query_as::<_, Transaction>(
        "INSERT INTO transactions (user_id, amount, category, description, transaction_type, icon, date, wallet_id)
         VALUES ($1, $2, $3, $4, $5, $6, COALESCE($7, NOW()), $8)
         RETURNING *",
    )
    .bind(user_id)
    .bind(amount)
    .bind(category)
    .bind(description)
    .bind(transaction_type)
    .bind(icon)
    .bind(date)
    .bind(wallet_id)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    amount: Decimal,
    category: &str,
    description: Option<&str>,
    transaction_type: &str,
    icon: Option<&str>,
    date: Option<chrono::DateTime<chrono::Utc>>,
    wallet_id: Option<Uuid>,
) -> Result<Transaction, sqlx::Error> {
    sqlx::query_as::<_, Transaction>(
        "UPDATE transactions SET amount = $3, category = $4, description = $5,
         transaction_type = $6, icon = $7, date = COALESCE($8, date), wallet_id = $9, updated_at = NOW()
         WHERE id = $1 AND user_id = $2 RETURNING *",
    )
    .bind(id)
    .bind(user_id)
    .bind(amount)
    .bind(category)
    .bind(description)
    .bind(transaction_type)
    .bind(icon)
    .bind(date)
    .bind(wallet_id)
    .fetch_one(pool)
    .await
}

pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM transactions WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_summary(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<(Decimal, Decimal), sqlx::Error> {
    let row: (Option<Decimal>, Option<Decimal>) = sqlx::query_as(
        "SELECT 
            COALESCE(SUM(CASE WHEN transaction_type = 'income' THEN amount ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN transaction_type = 'expense' THEN amount ELSE 0 END), 0)
         FROM transactions WHERE user_id = $1
         AND EXTRACT(MONTH FROM date) = EXTRACT(MONTH FROM NOW())
         AND EXTRACT(YEAR FROM date) = EXTRACT(YEAR FROM NOW())",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    Ok((row.0.unwrap_or_default(), row.1.unwrap_or_default()))
}

pub async fn get_weekly_activity(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<Decimal>, sqlx::Error> {
    let rows: Vec<(Option<Decimal>,)> = sqlx::query_as(
        "WITH days AS (
            SELECT generate_series(
                date_trunc('week', NOW()),
                date_trunc('week', NOW()) + interval '6 days',
                interval '1 day'
            ) AS day
        )
        SELECT COALESCE(SUM(t.amount), 0) AS total
        FROM days d
        LEFT JOIN transactions t ON date_trunc('day', t.date) = d.day
            AND t.user_id = $1 AND t.transaction_type = 'expense'
        GROUP BY d.day ORDER BY d.day",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| r.0.unwrap_or_default()).collect())
}

pub async fn get_weekly_income(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<Decimal>, sqlx::Error> {
    let rows: Vec<(Option<Decimal>,)> = sqlx::query_as(
        "WITH days AS (
            SELECT generate_series(
                date_trunc('week', NOW()),
                date_trunc('week', NOW()) + interval '6 days',
                interval '1 day'
            ) AS day
        )
        SELECT COALESCE(SUM(t.amount), 0) AS total
        FROM days d
        LEFT JOIN transactions t ON date_trunc('day', t.date) = d.day
            AND t.user_id = $1 AND t.transaction_type = 'income'
        GROUP BY d.day ORDER BY d.day",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| r.0.unwrap_or_default()).collect())
}

pub async fn get_recent(
    pool: &PgPool,
    user_id: Uuid,
    limit: i64,
) -> Result<Vec<Transaction>, sqlx::Error> {
    sqlx::query_as::<_, Transaction>(
        "SELECT * FROM transactions WHERE user_id = $1 ORDER BY date DESC LIMIT $2",
    )
    .bind(user_id)
    .bind(limit)
    .fetch_all(pool)
    .await
}
