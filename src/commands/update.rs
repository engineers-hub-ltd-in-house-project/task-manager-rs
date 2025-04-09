use crate::db::TaskRepository;
use crate::error::{Result, TaskError};
use crate::models::task::Priority;
use crate::utils::date::parse_date;
use crate::utils::format::format_task;

/// タスクを更新するコマンド
pub fn update_task(
    id: i64,
    title: Option<String>,
    description: Option<String>,
    due: Option<String>,
    remove_due: bool,
    priority: Option<i32>,
    tags: Option<String>,
) -> Result<()> {
    let mut repo = TaskRepository::new()?;
    let mut task = repo.get_task(id)?;
    
    // タイトルの更新
    if let Some(new_title) = title {
        task.title = new_title;
    }
    
    // 説明の更新
    if let Some(new_desc) = description {
        task.description = Some(new_desc);
    }
    
    // 期限の更新
    if remove_due {
        task.due_date = None;
    } else if let Some(due_str) = due {
        task.due_date = Some(parse_date(&due_str)?);
    }
    
    // 優先度の更新
    if let Some(p) = priority {
        task.priority = Priority::from_i32(p).map_err(|_| TaskError::InvalidPriority(p))?;
    }
    
    // タグの更新
    if let Some(tags_str) = tags {
        task.tags = tags_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
    }
    
    // タスクの保存
    repo.update_task(&task)?;
    
    println!("タスクを更新しました（ID: {}）", id);
    println!("{}", format_task(&task, true));
    
    Ok(())
} 
