use crate::db::TaskRepository;
use crate::error::Result;
use crate::utils::format::format_task;

/// 特定のタスクの詳細を表示するコマンド
pub fn show_task(id: i64) -> Result<()> {
    let repo = TaskRepository::new()?;
    let task = repo.get_task(id)?;
    
    println!("{}", format_task(&task, true));
    
    Ok(())
} 
