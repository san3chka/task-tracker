use std::{fs::{read_to_string, OpenOptions}, io::Write};

use clap::Args;
use anyhow::Result;

use super::add::TaskData;

/// Struct for marking a task as done
#[derive(Debug, Args)]
pub struct MarkDone {
    /// Task ID to be marked as done
    pub id: i64,
}

impl MarkDone {
    /// Updates the status of a task to the given `mark_status` (typically "done")
    pub fn mark_done(&self, file_path: &str, mark_status: &str) -> Result<()> {
        // Load tasks from the file
        let mut tasks: Vec<TaskData> = match read_to_string(file_path) {
            Ok(content) if !content.is_empty() => serde_json::from_str(&content)?,
            _ => Vec::new(),
        };

        // Find the task with the matching ID and update its status
        if let Some(task) = tasks.iter_mut().find(|task| task.id == Some(self.id)) {
            task.status = Some(mark_status.to_string());
        } else {
            return Err(anyhow::anyhow!("Task with ID {} not found", self.id));
        }

        // Overwrite the file with the updated task list
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)?;

        file.write_all(serde_json::to_string_pretty(&tasks)?.as_bytes())?;

        Ok(())
    }
}
