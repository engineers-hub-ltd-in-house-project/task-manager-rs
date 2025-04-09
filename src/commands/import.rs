use crate::db::TaskRepository;
use crate::error::{Result, TaskError};
use crate::models::task::Task;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// タスクデータをインポートするコマンド
pub fn import_tasks(file: String) -> Result<()> {
    let path = Path::new(&file);
    
    // ファイル拡張子の確認
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| TaskError::InvalidArgument("ファイル拡張子が認識できません".to_string()))?;
    
    let tasks = match extension.to_lowercase().as_str() {
        "json" => import_from_json(&file)?,
        "csv" => import_from_csv(&file)?,
        _ => {
            return Err(TaskError::InvalidArgument(format!(
                "不正なファイル形式: {}. .json または .csv ファイルを指定してください", 
                extension
            )));
        }
    };
    
    // データベースへのインポート
    let mut repo = TaskRepository::new()?;
    let mut success_count = 0;
    
    for task in tasks {
        match repo.add_task(&task) {
            Ok(_) => success_count += 1,
            Err(e) => eprintln!("タスク '{}' のインポートに失敗しました: {}", task.title, e),
        }
    }
    
    println!("{}件のタスクをインポートしました", success_count);
    Ok(())
}

/// JSONファイルからインポート
fn import_from_json(file: &str) -> Result<Vec<Task>> {
    let mut file = File::open(Path::new(file))?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    let tasks: Vec<Task> = serde_json::from_str(&content)?;
    Ok(tasks)
}

/// CSVファイルからインポート
fn import_from_csv(file: &str) -> Result<Vec<Task>> {
    use chrono::DateTime;
    use crate::models::task::Priority;
    
    let file = File::open(Path::new(file))?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut tasks = Vec::new();
    
    for result in rdr.records() {
        let record = result?;
        
        // レコードが十分なフィールドを持っているか確認
        if record.len() < 9 {
            eprintln!("警告: 一部のフィールドが不足しているレコードをスキップします");
            continue;
        }
        
        // 各フィールドの解析
        let title = record[1].to_string();
        let description = if record[2].is_empty() { None } else { Some(record[2].to_string()) };
        
        let created_at = match DateTime::parse_from_rfc3339(&record[3]) {
            Ok(dt) => dt.with_timezone(&chrono::Local),
            Err(_) => {
                eprintln!("警告: 作成日時の解析に失敗しました: {}", &record[3]);
                continue;
            }
        };
        
        let due_date = if record[4].is_empty() {
            None
        } else {
            match DateTime::parse_from_rfc3339(&record[4]) {
                Ok(dt) => Some(dt.with_timezone(&chrono::Local)),
                Err(_) => {
                    eprintln!("警告: 期限日時の解析に失敗しました: {}", &record[4]);
                    None
                }
            }
        };
        
        let completed = record[5].parse::<bool>().unwrap_or(false);
        
        let completed_at = if record[6].is_empty() {
            None
        } else {
            match DateTime::parse_from_rfc3339(&record[6]) {
                Ok(dt) => Some(dt.with_timezone(&chrono::Local)),
                Err(_) => {
                    eprintln!("警告: 完了日時の解析に失敗しました: {}", &record[6]);
                    None
                }
            }
        };
        
        let priority = match record[7].parse::<i32>() {
            Ok(p) => match Priority::from_i32(p) {
                Ok(priority) => priority,
                Err(_) => {
                    eprintln!("警告: 不正な優先度: {}, デフォルト値を使用します", &record[7]);
                    Priority::Medium
                }
            },
            Err(_) => {
                eprintln!("警告: 優先度の解析に失敗しました: {}, デフォルト値を使用します", &record[7]);
                Priority::Medium
            }
        };
        
        let tags = if record[8].is_empty() {
            Vec::new()
        } else {
            record[8].split(',').map(|s| s.to_string()).collect()
        };
        
        // タスクの作成
        let mut task = Task::new(title, description, None, priority, tags);
        
        // 各フィールドを設定
        task.created_at = created_at;
        task.due_date = due_date;
        task.completed = completed;
        task.completed_at = completed_at;
        
        tasks.push(task);
    }
    
    Ok(tasks)
} 
