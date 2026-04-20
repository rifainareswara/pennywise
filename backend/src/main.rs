mod config;
mod db;
mod errors;
mod handlers;
mod middleware;
mod models;
mod repositories;
mod services;

use axum::{
    middleware::from_fn,
    routing::{delete, get, post, put},
    Extension, Router,
};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config::Config::from_env();
    let pool = db::create_pool(&config.database_url).await;
    db::run_migrations(&pool).await;

    tracing::info!("Database connected and migrations applied");

    let cors = CorsLayer::new()
        .allow_origin(config.frontend_url.parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);

    // Public routes (no auth)
    let public_routes = Router::new()
        .route("/api/auth/register", post(handlers::auth::register))
        .route("/api/auth/login", post(handlers::auth::login));

    // Protected routes (auth required)
    let protected_routes = Router::new()
        .route("/api/auth/profile", get(handlers::auth::profile))
        .route("/api/auth/profile", put(handlers::auth::update_profile))
        .route("/api/auth/password", put(handlers::auth::update_password))
        .route("/api/transactions", get(handlers::transactions::list))
        .route("/api/transactions", post(handlers::transactions::create))
        .route("/api/transactions/{id}", get(handlers::transactions::get_one))
        .route("/api/transactions/{id}", put(handlers::transactions::update))
        .route("/api/transactions/{id}", delete(handlers::transactions::delete))
        .route("/api/budgets", get(handlers::budgets::list))
        .route("/api/budgets", post(handlers::budgets::create))
        .route("/api/budgets/{id}", delete(handlers::budgets::delete))
        .route("/api/dashboard/summary", get(handlers::dashboard::summary))
        .route("/api/tasks", get(handlers::tasks::list))
        .route("/api/tasks", post(handlers::tasks::create))
        .route("/api/tasks/{id}", get(handlers::tasks::get))
        .route("/api/tasks/{id}", put(handlers::tasks::update))
        .route("/api/tasks/{id}", delete(handlers::tasks::delete))
        .layer(from_fn(middleware::auth::auth_middleware));

    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(Extension(pool))
        .layer(Extension(config.clone()))
        .layer(cors);

    let addr = format!("{}:{}", config.server_host, config.server_port);
    tracing::info!("Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
