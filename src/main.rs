mod cli;
mod commands;
mod db;
mod error;
mod models;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;

fn main() {
    // コマンドライン引数の解析
    let cli = Cli::parse();
    
    // コマンドの実行
    let result = match cli.command {
        Commands::Add { title, description, due, priority, tags } => {
            commands::add_task(title, description, due, priority, tags)
        },
        Commands::List { all, priority, due_today, tags } => {
            commands::list_tasks(all, priority, due_today, tags)
        },
        Commands::Show { id } => {
            commands::show_task(id)
        },
        Commands::Update { id, title, description, due, remove_due, priority, tags } => {
            commands::update_task(id, title, description, due, remove_due, priority, tags)
        },
        Commands::Complete { id } => {
            commands::complete_task(id)
        },
        Commands::Uncomplete { id } => {
            commands::uncomplete_task(id)
        },
        Commands::Delete { id, completed } => {
            commands::delete_task(id, completed)
        },
        Commands::Stats => {
            commands::show_stats()
        },
        Commands::Export { file, format } => {
            commands::export_tasks(file, format)
        },
        Commands::Import { file } => {
            commands::import_tasks(file)
        },
    };
    
    // エラーハンドリング
    if let Err(e) = result {
        eprintln!("{} {}", "エラー:".red().bold(), e);
        std::process::exit(1);
    }
}
