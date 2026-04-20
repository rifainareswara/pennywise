use crate::models::task::{CreateTaskDto, Task, UpdateTaskDto};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn find_all_by_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<Task>, sqlx::Error> {
    sqlx::query_as::<_, Task>(
        "SELECT * FROM tasks WHERE user_id = $1 AND (status != 'completed' OR (status = 'completed' AND updated_at >= NOW() - INTERVAL '30 days')) ORDER BY COALESCE(due_date, created_at) ASC"
    )
        .bind(user_id)
        .fetch_all(pool)
        .await
}

pub async fn find_by_id(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<Option<Task>, sqlx::Error> {
    sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user_id)
        .fetch_optional(pool)
        .await
}

pub async fn create(pool: &PgPool, user_id: Uuid, dto: &CreateTaskDto) -> Result<Task, sqlx::Error> {
    let id = Uuid::new_v4();
    let metadata = dto.metadata.clone().unwrap_or_else(|| serde_json::json!({}));

    sqlx::query_as::<_, Task>(
        r#"
        INSERT INTO tasks (id, user_id, task_type, title, description, category, priority, due_date, status, metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(user_id)
    .bind(&dto.task_type)
    .bind(&dto.title)
    .bind(&dto.description)
    .bind(&dto.category)
    .bind(&dto.priority)
    .bind(dto.due_date)
    .bind(&dto.status)
    .bind(metadata)
    .fetch_one(pool)
    .await
}

pub async fn update(pool: &PgPool, id: Uuid, user_id: Uuid, dto: &UpdateTaskDto) -> Result<Option<Task>, sqlx::Error> {
    sqlx::query_as::<_, Task>(
        r#"
        UPDATE tasks 
        SET 
            task_type = COALESCE($1, task_type),
            title = COALESCE($2, title),
            description = COALESCE($3, description),
            category = COALESCE($4, category),
            priority = COALESCE($5, priority),
            due_date = COALESCE($6, due_date),
            status = COALESCE($7, status),
            metadata = COALESCE($8, metadata),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $9 AND user_id = $10
        RETURNING *
        "#,
    )
    .bind(&dto.task_type)
    .bind(&dto.title)
    .bind(&dto.description)
    .bind(&dto.category)
    .bind(&dto.priority)
    .bind(dto.due_date)
    .bind(&dto.status)
    .bind(&dto.metadata)
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM tasks WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
