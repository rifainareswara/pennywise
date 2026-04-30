use crate::models::wallet::Wallet;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn find_all(pool: &PgPool, user_id: Uuid) -> Result<Vec<Wallet>, sqlx::Error> {
    sqlx::query_as::<_, Wallet>(
        "SELECT * FROM wallets WHERE user_id = $1 ORDER BY created_at ASC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> Result<Option<Wallet>, sqlx::Error> {
    sqlx::query_as::<_, Wallet>(
        "SELECT * FROM wallets WHERE id = $1 AND user_id = $2",
    )
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn create(
    pool: &PgPool,
    user_id: Uuid,
    name: &str,
    wallet_type: &str,
    balance: Decimal,
    icon: Option<&str>,
    color: Option<&str>,
) -> Result<Wallet, sqlx::Error> {
    sqlx::query_as::<_, Wallet>(
        "INSERT INTO wallets (user_id, name, wallet_type, balance, icon, color)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *",
    )
    .bind(user_id)
    .bind(name)
    .bind(wallet_type)
    .bind(balance)
    .bind(icon)
    .bind(color)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    name: &str,
    wallet_type: &str,
    balance: Decimal,
    icon: Option<&str>,
    color: Option<&str>,
) -> Result<Wallet, sqlx::Error> {
    sqlx::query_as::<_, Wallet>(
        "UPDATE wallets
         SET name = $3, wallet_type = $4, balance = $5, icon = $6, color = $7, updated_at = NOW()
         WHERE id = $1 AND user_id = $2
         RETURNING *",
    )
    .bind(id)
    .bind(user_id)
    .bind(name)
    .bind(wallet_type)
    .bind(balance)
    .bind(icon)
    .bind(color)
    .fetch_one(pool)
    .await
}

pub async fn adjust_balance(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    balance: Decimal,
) -> Result<Wallet, sqlx::Error> {
    sqlx::query_as::<_, Wallet>(
        "UPDATE wallets SET balance = $3, updated_at = NOW() WHERE id = $1 AND user_id = $2 RETURNING *",
    )
    .bind(id)
    .bind(user_id)
    .bind(balance)
    .fetch_one(pool)
    .await
}

pub async fn get_total_balance(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<Decimal>, sqlx::Error> {
    let row: (Option<i64>, Option<Decimal>) = sqlx::query_as(
        "SELECT COUNT(*), SUM(balance) FROM wallets WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    // Return None if user has no wallets (so dashboard falls back to tx-based balance)
    if row.0.unwrap_or(0) == 0 {
        Ok(None)
    } else {
        Ok(Some(row.1.unwrap_or_default()))
    }
}

pub async fn apply_transaction_delta(
    pool: &PgPool,
    wallet_id: Uuid,
    user_id: Uuid,
    delta: Decimal,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE wallets SET balance = balance + $3, updated_at = NOW() \
         WHERE id = $1 AND user_id = $2",
    )
    .bind(wallet_id)
    .bind(user_id)
    .bind(delta)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM wallets WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
