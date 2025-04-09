use crate::db::TaskRepository;
use crate::error::{Result, TaskError};
use crate::utils::format::format_task;

/// タスクを削除するコマンド
pub fn delete_task(id: Option<i64>, completed: bool) -> Result<()> {
    // いずれかのオプションが必要
    if id.is_none() && !completed {
        return Err(TaskError::InvalidArgument(
            "タスクIDまたは --completed オプションを指定してください".to_string(),
        ));
    }
    
    let mut repo = TaskRepository::new()?;
    
    // 特定のタスクを削除
    if let Some(task_id) = id {
        // 削除前にタスク情報を取得して表示
        let task = repo.get_task(task_id)?;
        println!("以下のタスクを削除します:");
        println!("{}", format_task(&task, false));
        
        repo.delete_task(task_id)?;
        println!("タスクを削除しました（ID: {}）", task_id);
    } 
    // 完了済みタスクをすべて削除
    else if completed {
        let count = repo.delete_completed_tasks()?;
        println!("{}件の完了済みタスクを削除しました", count);
    }
    
    Ok(())
} 
