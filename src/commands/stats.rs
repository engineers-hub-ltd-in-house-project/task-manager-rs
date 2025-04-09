use crate::db::TaskRepository;
use crate::error::Result;
use crate::utils::format::format_stats;

/// タスクの統計情報を表示するコマンド
pub fn show_stats() -> Result<()> {
    let repo = TaskRepository::new()?;
    let stats = repo.get_stats()?;
    
    println!("{}", format_stats(&stats));
    
    Ok(())
} 
