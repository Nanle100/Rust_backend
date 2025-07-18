use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "TodoApp", about = "A simple CLI Todo App")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new task
    Add {
        title: String,
        description: String,
        time: String,
    },

    /// List all tasks
    List,

    /// Update an existing task
    Update {
        id: u32,
        #[arg(short = 't', long)]
        title: Option<String>,
        #[arg(short = 'd', long)]
        description: Option<String>,
        #[arg(short = 'm', long)]
        time: Option<String>,
    },

    /// Delete a task
    Delete {
        id: u32,
    },
    
    /// Export tasks to a readable text or CSV file
    Export {
        #[arg(short = 'f', long, default_value = "exported_tasks.txt")]
        file: String,
    },

}
