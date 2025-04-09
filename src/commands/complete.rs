use crate::db::TaskRepository;
use crate::error::Result;
use crate::utils::format::format_task;

/// タスクを完了状態に設定するコマンド
pub fn complete_task(id: i64) -> Result<()> {
    let mut repo = TaskRepository::new()?;
    repo.complete_task(id)?;
    
    let task = repo.get_task(id)?;
    println!("タスクを完了としてマークしました（ID: {}）", id);
    println!("{}", format_task(&task, false));
    
    Ok(())
}

/// タスクを未完了状態に設定するコマンド
pub fn uncomplete_task(id: i64) -> Result<()> {
    let mut repo = TaskRepository::new()?;
    repo.uncomplete_task(id)?;
    
    let task = repo.get_task(id)?;
    println!("タスクを未完了としてマークしました（ID: {}）", id);
    println!("{}", format_task(&task, false));
    
    Ok(())
} 
