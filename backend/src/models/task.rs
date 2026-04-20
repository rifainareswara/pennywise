use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Task {
    pub id: Uuid,
    pub user_id: Uuid,
    pub task_type: String, // 'general', 'shopping', 'billing', 'debt'
    pub title: String,
    pub description: Option<String>,
    pub category: String, // defaults to 'general'. Users can set this to 'Programming', etc.
    pub priority: String, // 'low', 'medium', 'high', 'urgent'
    pub due_date: Option<DateTime<Utc>>,
    pub status: String, // 'pending', 'in_progress', 'completed'
    // Storing metadata as JSON value since it can dynamically fit shopping, etc.
    pub metadata: serde_json::Value,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskDto {
    pub task_type: String,
    pub title: String,
    pub description: Option<String>,
    #[serde(default = "default_category")]
    pub category: String,
    #[serde(default = "default_priority")]
    pub priority: String,
    pub due_date: Option<DateTime<Utc>>,
    #[serde(default = "default_status")]
    pub status: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskDto {
    pub task_type: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub priority: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub status: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

fn default_category() -> String {
    "general".to_string()
}

fn default_priority() -> String {
    "medium".to_string()
}

fn default_status() -> String {
    "pending".to_string()
}
