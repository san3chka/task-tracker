use clap::{Parser, Subcommand};

use crate::cli::commands::{
    add::TaskData,
    delete::DeleteTask,
    mark_done::MarkDone,
    mark_in_progress::MarkInProgress,
    update::UpdateTask,
};
use super::commands::list::List;

/// CLI entry point with available commands
#[derive(Debug, Parser)]
pub struct Cli {
    /// Choose a command to execute
    #[clap(subcommand)]
    pub command: Commands,
}

/// Enum representing all available CLI commands
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add a new task
    Add(TaskData),

    /// Update an existing task
    Update(UpdateTask),
    
    /// Delete a task by ID
    Delete(DeleteTask),

    /// Mark a task as 'in progress'
    MarkInProgress(MarkInProgress),

    /// Mark a task as 'done'
    MarkDone(MarkDone),

    /// List all tasks, optionally filtered by status
    List(List),
}
