use crate::db::TaskRepository;
use crate::error::{Result, TaskError};
use crate::models::task::{Priority, Task};
use crate::utils::date::parse_date;

/// タスクを追加するコマンド
pub fn add_task(
    title: String,
    description: Option<String>,
    due: Option<String>,
    priority: i32,
    tags: Option<String>,
) -> Result<()> {
    // 優先度の検証
    let priority = Priority::from_i32(priority).map_err(|_| TaskError::InvalidPriority(priority))?;

    // 期限の変換
    let due_date = match due {
        Some(due_str) => Some(parse_date(&due_str)?),
        None => None,
    };

    // タグのパース
    let tags_vec = tags
        .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_else(Vec::new);

    // タスクの作成
    let task = Task::new(title, description, due_date, priority, tags_vec);

    // タスクの保存
    let mut repo = TaskRepository::new()?;
    let task_id = repo.add_task(&task)?;

    println!("タスクを追加しました（ID: {}）", task_id);
    Ok(())
} 
