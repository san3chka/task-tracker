use std::{fs::{read_to_string, OpenOptions}, io::Write};

use clap::Args;
use anyhow::Result;

use super::add::TaskData;

/// Struct to mark a task as "in progress"
#[derive(Debug, Args)]
pub struct MarkInProgress {
    /// ID of the task to update
    pub id: i64,
}

impl MarkInProgress {
    /// Updates the status of the task to "in-progress"
    pub fn mark_in_progress(&self, file_path: &str, mark_status: &str) -> Result<()> {
        // Load tasks from file
        let mut tasks: Vec<TaskData> = match read_to_string(file_path) {
            Ok(content) if !content.is_empty() => serde_json::from_str(&content)?,
            _ => Vec::new(),
        };

        // Find the task by ID and update its status
        if let Some(task) = tasks.iter_mut().find(|task| task.id == Some(self.id)) {
            task.status = Some(mark_status.to_string());
        } else {
            return Err(anyhow::anyhow!("Task with ID {} not found", self.id));
        }

        // Write updated task list back to the file
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)?;

        file.write_all(serde_json::to_string_pretty(&tasks)?.as_bytes())?;

        Ok(())
    }
}
