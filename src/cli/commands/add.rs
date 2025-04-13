use serde::{Serialize, Deserialize};
use anyhow::Result;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use clap::Args;

use crate::domain::defaults::*;

/// Struct representing a single task
#[derive(Debug, Serialize, Deserialize, Clone, Args)]
pub struct TaskData {
    /// Description of the task
    pub description: String,

    /// Task ID (automatically assigned)
    #[serde(default = "default_id")]
    #[clap(skip)]
    pub id: Option<i64>,

    /// Status of the task (e.g., todo, in-progress, done)
    #[serde(default = "default_status")]
    #[clap(skip)]
    pub status: Option<String>,

    /// Creation timestamp
    #[serde(default = "default_created_at")]
    #[clap(skip)]
    pub created_at: Option<String>,

    /// Last updated timestamp
    #[serde(default = "default_updated_at")]
    #[clap(skip)]
    pub updated_at: Option<String>,
}

impl TaskData {
    /// Fills in any missing fields (e.g., ID, timestamps) with default values
    pub fn get_add_info(mut self) -> Self {
        if self.id.is_none() {
            self.id = default_id();
        }
        if self.status.is_none() {
            self.status = default_status();
        }
        if self.created_at.is_none() {
            self.created_at = default_created_at();
        }
        if self.updated_at.is_none() {
            self.updated_at = default_updated_at();
        }
        self
    }

    /// Saves the current task to a file (appends to the existing task list)
    pub fn save_to_file(&self, file_path: &str) -> Result<()> {
        // Read existing tasks from file
        let tasks: Vec<TaskData> = match read_to_string(file_path) {
            Ok(content) if !content.is_empty() => {
                serde_json::from_str(&content).unwrap_or_default()
            },
            _ => Vec::new(),
        };

        // Append the new task
        let mut updated_tasks = tasks;
        updated_tasks.push(self.clone());

        // Overwrite the file with the updated task list
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path)?;

        file.write_all(serde_json::to_string_pretty(&updated_tasks)?.as_bytes())?;

        Ok(())
    }
}
