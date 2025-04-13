use clap::Args;
use anyhow::Result;
use std::{
    fs::{read_to_string, OpenOptions},
    io::Write,
};

use super::add::TaskData;

/// Struct for deleting a task by ID
#[derive(Debug, Clone, Args)]
pub struct DeleteTask {
    /// ID of the task to delete
    pub id: i64,
}

impl DeleteTask {
    /// Deletes a task by its ID from the file
    pub fn delete_task(&self, file_path: &str) -> Result<()> {
        // Read and parse existing tasks
        let mut tasks: Vec<TaskData> = match read_to_string(file_path) {
            Ok(content) if !content.is_empty() => serde_json::from_str(&content)?,
            _ => Vec::new(),
        };

        let initial_len = tasks.len();

        // Retain all tasks except the one with the matching ID
        tasks.retain(|task| task.id != Some(self.id));

        // If no task was removed, return an error
        if tasks.len() == initial_len {
            return Err(anyhow::anyhow!("Task with ID {:?} not found", self.id));
        }

        // Write the updated task list back to the file
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)?;

        file.write_all(serde_json::to_string_pretty(&tasks)?.as_bytes())?;

        Ok(())
    }
}
