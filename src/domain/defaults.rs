use chrono::Utc;
use dotenv::dotenv;
use std::env;
use std::fs::read_to_string;

use crate::cli::commands::add::TaskData;

/// Returns the current UTC timestamp as creation time
pub fn default_created_at() -> Option<String> {
    Some(Utc::now().to_string())
}

/// Returns the current UTC timestamp as update time
pub fn default_updated_at() -> Option<String> {
    Some(Utc::now().to_string())
}

/// Default task status: "todo"
pub fn default_status() -> Option<String> {
    Some("todo".to_string())
}

/// Returns a new unique task ID based on the highest existing ID
pub fn default_id() -> Option<i64> {
    Some(get_latest_id() + 1)
}

/// Scans the file for the highest task ID and returns it
/// If no tasks exist, returns 0
pub fn get_latest_id() -> i64 {
    dotenv().ok(); // Load .env to get FILE_PATH
    let file_path = env::var("FILE_PATH").expect("Failed to load FILE_PATH from environment");

    let tasks: Vec<TaskData> = match read_to_string(file_path) {
        Ok(content) if !content.trim().is_empty() => serde_json::from_str(&content).unwrap_or_default(),
        _ => Vec::new(),
    };

    // Extract max ID or default to 0
    tasks
        .iter()
        .filter_map(|task| task.id)
        .max()
        .unwrap_or(0)
}
