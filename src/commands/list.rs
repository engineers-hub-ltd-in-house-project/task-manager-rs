use crate::db::TaskRepository;
use crate::error::{Result, TaskError};
use crate::models::task::Priority;
use crate::utils::format::format_task;

/// タスクの一覧を表示するコマンド
pub fn list_tasks(
    all: bool,
    priority: Option<i32>,
    due_today: bool,
    tags: Option<String>,
) -> Result<()> {
    let repo = TaskRepository::new()?;
    
    // 取得方法の選択（優先度、期限日、タグによるフィルタリング）
    let tasks = if let Some(p) = priority {
        let priority = Priority::from_i32(p).map_err(|_| TaskError::InvalidPriority(p))?;
        repo.get_tasks_by_priority(priority)?
    } else if due_today {
        repo.get_tasks_due_today()?
    } else if let Some(tag) = tags {
        repo.get_tasks_by_tag(&tag)?
    } else {
        repo.get_all_tasks(all)?
    };
    
    // タスクがない場合
    if tasks.is_empty() {
        println!("タスクが見つかりませんでした");
        return Ok(());
    }
    
    // タスクの表示
    println!("全{}件のタスク:", tasks.len());
    for task in tasks {
        println!("{}", format_task(&task, false));
    }
    
    Ok(())
} 
