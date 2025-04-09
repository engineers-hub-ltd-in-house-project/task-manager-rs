use crate::db::TaskRepository;
use crate::error::{Result, TaskError};
use serde_json;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// タスクデータをエクスポートするコマンド
pub fn export_tasks(file: String, format: String) -> Result<()> {
    let repo = TaskRepository::new()?;
    let tasks = repo.get_all_tasks(true)?;
    
    match format.to_lowercase().as_str() {
        "json" => export_as_json(&file, &tasks)?,
        "csv" => export_as_csv(&file, &tasks)?,
        _ => {
            return Err(TaskError::InvalidArgument(format!(
                "不正なフォーマット: {}. 'json' または 'csv' を指定してください", 
                format
            )));
        }
    }
    
    println!("{}件のタスクを {} にエクスポートしました", tasks.len(), file);
    Ok(())
}

/// JSONフォーマットでエクスポート
fn export_as_json(file: &str, tasks: &[crate::models::task::Task]) -> Result<()> {
    let json = serde_json::to_string_pretty(tasks)?;
    let mut file = File::create(Path::new(file))?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

/// CSVフォーマットでエクスポート
fn export_as_csv(file: &str, tasks: &[crate::models::task::Task]) -> Result<()> {
    let file = File::create(Path::new(file))?;
    let mut wtr = csv::Writer::from_writer(file);
    
    // ヘッダーの書き込み
    wtr.write_record(&[
        "ID", "Title", "Description", "Created At", "Due Date", 
        "Completed", "Completed At", "Priority", "Tags"
    ])?;
    
    // データの書き込み
    for task in tasks {
        wtr.write_record(&[
            task.id.map(|id| id.to_string()).unwrap_or_default(),
            task.title.clone(),
            task.description.clone().unwrap_or_default(),
            task.created_at.to_rfc3339(),
            task.due_date.map(|date| date.to_rfc3339()).unwrap_or_default(),
            task.completed.to_string(),
            task.completed_at.map(|date| date.to_rfc3339()).unwrap_or_default(),
            (task.priority as i32).to_string(),
            task.tags.join(","),
        ])?;
    }
    
    wtr.flush()?;
    Ok(())
} 
