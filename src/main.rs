use cli::args::{Cli, Commands};
use clap::Parser;
use dotenv::dotenv;
use std::env;

mod cli;
mod domain;

fn main() {
    dotenv().ok(); // Load environment variables from .env if available

    let args = Cli::parse();
    let file_path = env::var("FILE_PATH").expect("Failed to load FILE_PATH from environment");

    // Handle CLI command
    if let Err(e) = run_command(args.command, &file_path) {
        eprintln!("Error: {}", e);
    }
}

// Handles each CLI command and delegates to corresponding logic
fn run_command(command: Commands, file_path: &str) -> anyhow::Result<()> {

    match command {
        Commands::Add(task) => {
            let task_add_info = task.get_add_info();
            task_add_info.save_to_file(file_path)?;
            println!("Task added successfully (ID: {})", domain::defaults::get_latest_id());
        }

        Commands::Update(task) => {
            task.change_description(file_path)?;
            println!("Task updated successfully");
        }

        Commands::Delete(task) => {
            task.delete_task(file_path)?;
            println!("Task deleted successfully");
        }

        Commands::MarkInProgress(task) => {
            task.mark_in_progress(file_path, "in-progress")?;
            println!("Task marked as 'in-progress'");
        }

        Commands::MarkDone(task) => {
            task.mark_done(file_path, "done")?;
            println!("Task marked as 'done'");
        }

        Commands::List(list) => {
            list.read_list(file_path)?;
            println!("Task list displayed");
        }
    }

    Ok(())
}
