use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    middleware::auth::AuthUser,
    models::task::{CreateTaskDto, Task, UpdateTaskDto},
    repositories::task as task_repo,
};

pub async fn list(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<Task>>, AppError> {
    let tasks = task_repo::find_all_by_user(&pool, auth.user_id).await?;
    Ok(Json(tasks))
}

pub async fn get(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<Task>, AppError> {
    let task = task_repo::find_by_id(&pool, id, auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Task not found".into()))?;
    Ok(Json(task))
}

pub async fn create(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
    Json(input): Json<CreateTaskDto>,
) -> Result<(StatusCode, Json<Task>), AppError> {
    let task = task_repo::create(&pool, auth.user_id, &input).await?;
    Ok((StatusCode::CREATED, Json(task)))
}

pub async fn update(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateTaskDto>,
) -> Result<Json<Task>, AppError> {
    // Check if exists
    task_repo::find_by_id(&pool, id, auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Task not found".into()))?;

    let task = task_repo::update(&pool, id, auth.user_id, &input)
        .await?
        .expect("Task should exist due to prior check");

    Ok(Json(task))
}

pub async fn delete(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    task_repo::find_by_id(&pool, id, auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Task not found".into()))?;

    task_repo::delete(&pool, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
