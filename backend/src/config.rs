use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub server_host: String,
    pub server_port: u16,
    pub frontend_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://pennywise:pennywise@localhost:5432/pennywise".into()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "pennywise-dev-secret-change-in-production".into()),
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".into())
                .parse()
                .unwrap_or(3000),
            frontend_url: env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:5173".into()),
        }
    }
}
