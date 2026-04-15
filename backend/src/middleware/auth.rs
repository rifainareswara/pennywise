use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::services::auth::verify_token;

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub user_id: Uuid,
}

pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "pennywise-dev-secret-change-in-production".into());

    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "));

    let token = match token {
        Some(t) => t,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": "Missing authorization token" })),
            ));
        }
    };

    let claims = match verify_token(token, &jwt_secret) {
        Ok(c) => c,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": "Invalid or expired token" })),
            ));
        }
    };

    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": "Invalid token payload" })),
            ));
        }
    };

    req.extensions_mut().insert(AuthUser { user_id });
    Ok(next.run(req).await)
}
