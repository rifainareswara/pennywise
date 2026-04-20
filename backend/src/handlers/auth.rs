use axum::{extract::Extension, Json};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    config::Config,
    errors::AppError,
    models::user::{AuthResponse, LoginInput, RegisterInput, UpdateProfileInput, UserResponse},
    repositories::user as user_repo,
    services::auth::{create_token, hash_password, verify_password},
    middleware::auth::AuthUser,
};

pub async fn register(
    Extension(pool): Extension<PgPool>,
    Extension(config): Extension<Config>,
    Json(input): Json<RegisterInput>,
) -> Result<Json<AuthResponse>, AppError> {
    input.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Check if user already exists
    if user_repo::find_by_email(&pool, &input.email).await?.is_some() {
        return Err(AppError::Conflict("Email already registered".into()));
    }

    let password_hash = hash_password(&input.password)?;
    let user = user_repo::create(&pool, &input.email, &input.name, &password_hash).await?;
    let token = create_token(user.id, &config.jwt_secret)?;

    Ok(Json(AuthResponse {
        token,
        user: UserResponse::from(user),
    }))
}

pub async fn login(
    Extension(pool): Extension<PgPool>,
    Extension(config): Extension<Config>,
    Json(input): Json<LoginInput>,
) -> Result<Json<AuthResponse>, AppError> {
    input.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;

    let user = user_repo::find_by_email(&pool, &input.email)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid email or password".into()))?;

    let is_valid = verify_password(&input.password, &user.password_hash)?;
    if !is_valid {
        return Err(AppError::Unauthorized("Invalid email or password".into()));
    }

    let token = create_token(user.id, &config.jwt_secret)?;

    Ok(Json(AuthResponse {
        token,
        user: UserResponse::from(user),
    }))
}

pub async fn profile(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<UserResponse>, AppError> {
    let user = user_repo::find_by_id(&pool, auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    Ok(Json(UserResponse::from(user)))
}

pub async fn update_profile(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthUser>,
    Json(input): Json<UpdateProfileInput>,
) -> Result<Json<UserResponse>, AppError> {
    input.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Make sure user exists
    let _ = user_repo::find_by_id(&pool, auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    let updated_user = user_repo::update_name(&pool, auth.user_id, &input.name)
        .await
        .map_err(|_| AppError::Internal("Database error".into()))?;

    Ok(Json(UserResponse::from(updated_user)))
}
