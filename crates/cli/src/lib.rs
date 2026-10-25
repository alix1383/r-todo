use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Comamnds,
}

#[derive(Subcommand, Debug)]
pub enum Comamnds {
    /// add new task
    Add { task_name: String },
    /// Remove a task
    Remove { task_name: String },
    /// toggle a task
    Toggle { task_name: String },
    /// Show task list
    List,
    /// Remove all tasks
    Reset,
}

pub fn cli() -> Cli {
    let args = Cli::parse();

    args
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
