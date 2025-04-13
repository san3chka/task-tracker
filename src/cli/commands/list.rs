use std::fs::read_to_string;

use clap::Args;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::add::TaskData;

/// Struct for listing tasks, optionally filtered by status
#[derive(Debug, Args, Serialize, Deserialize)]
pub struct List {
    /// Optional status filter (e.g., "done", "todo", "in-progress")
    pub status: Option<String>,
}

impl List {
    /// Reads tasks from the file and prints them, optionally filtered by status
    pub fn read_list(&self, file_path: &str) -> Result<()> {
        // Read the task file and deserialize JSON into TaskData vector
        let tasks: Vec<TaskData> = match read_to_string(file_path) {
            Ok(content) if !content.is_empty() => serde_json::from_str(&content).unwrap_or_default(),
            _ => Vec::new(),
        };

        // Filter tasks if a status is provided
        let filtered_tasks: Vec<TaskData> = match &self.status {
            Some(status_filter) => tasks
                .into_iter()
                .filter(|task| {
                    task.status
                        .as_ref()
                        .map(|s| s.eq_ignore_ascii_case(status_filter))
                        .unwrap_or(false)
                })
                .collect(),
            None => tasks,
        };

        // Print task list or indicate it's empty
        if filtered_tasks.is_empty() {
            println!("Task list is empty.");
        } else {
            for task in filtered_tasks {
                println!("Task #{}", task.id.unwrap_or_default());
                println!("Description: {}", task.description);
                println!("Status: {}", task.status.clone().unwrap_or("unknown".into()));
                println!("Created at: {}", task.created_at.clone().unwrap_or("unknown".into()));
                println!("Updated at: {}", task.updated_at.clone().unwrap_or("unknown".into()));
                println!("-------------------------------");
            }
        }

        Ok(())
    }
}
