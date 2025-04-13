use clap::Args;
use serde::{Serialize, Deserialize};
use crate::domain::defaults::*;
use anyhow::Result;
use crate::cli::commands::add::TaskData;
use std::{fs::{read_to_string, OpenOptions}, io::Write};

#[derive(Debug, Serialize, Deserialize, Clone, Args)]
pub struct UpdateTask {
    /// Task ID
    pub id: i64,

    /// Task description
    pub description: String,
}

impl UpdateTask {
    // Method to change the task description in the file
    pub fn change_description(&self, file_path: &str) -> Result<()> {
        // Read the file content and deserialize it into a vector of tasks
        let mut tasks: Vec<TaskData> = match read_to_string(file_path) {
            Ok(content) if !content.is_empty() => serde_json::from_str(&content).unwrap_or_default(),
            _ => Vec::new(), // Return an empty vector if the file is empty or can't be read
        };

        // Find the task by ID and update its description
        if let Some(task) = tasks.iter_mut().find(|task| task.id == Some(self.id)) {
            task.description = self.description.clone();
            task.updated_at = default_created_at(); // Update the timestamp
        } else {
            return Err(anyhow::anyhow!("Task with ID {} not found", self.id)); // Error if task not found
        }

        // Open the file for writing, truncate its contents, and write the updated tasks
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)?;

        file.write_all(serde_json::to_string_pretty(&tasks)?.as_bytes())?;

        Ok(())
    }
}