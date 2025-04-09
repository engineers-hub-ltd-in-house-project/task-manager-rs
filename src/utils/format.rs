use colored::*;
use crate::models::task::{Priority, Task};
use crate::utils::date::get_relative_time;

/// タスクの表示用文字列を作成
pub fn format_task(task: &Task, verbose: bool) -> String {
    let status = if task.completed {
        "[✓]".green()
    } else if task.is_overdue() {
        "[!]".red()
    } else if task.is_due_today() {
        "[⚑]".yellow()
    } else {
        "[ ]".normal()
    };

    let id = format!("{:>3}", task.id.unwrap_or(0)).blue();
    let title = if task.completed {
        task.title.strikethrough()
    } else {
        match task.priority {
            Priority::High => task.title.red().bold(),
            Priority::Medium => task.title.yellow(),
            Priority::Low => task.title.normal(),
        }
    };

    let mut result = format!("{} {} {}", status, id, title);

    // タグの表示
    if !task.tags.is_empty() {
        let tags = task.tags.iter()
            .map(|t| format!("#{}", t).cyan().to_string())
            .collect::<Vec<_>>()
            .join(" ");
        result = format!("{} {}", result, tags);
    }

    // 期限の表示
    if let Some(due_date) = task.due_date {
        let due_str = format!("期限: {}", get_relative_time(due_date));
        if task.is_overdue() {
            result = format!("{} {}", result, due_str.red());
        } else if task.is_due_today() {
            result = format!("{} {}", result, due_str.yellow());
        } else {
            result = format!("{} {}", result, due_str.normal());
        }
    }

    // 詳細表示
    if verbose {
        if let Some(desc) = &task.description {
            if !desc.is_empty() {
                result = format!("{}\n    {}", result, desc);
            }
        }

        let created_str = format!("作成: {}", get_relative_time(task.created_at));
        result = format!("{}\n    {}", result, created_str);

        if task.completed {
            if let Some(completed_at) = task.completed_at {
                let completed_str = format!("完了: {}", get_relative_time(completed_at));
                result = format!("{} | {}", result, completed_str.green());
            }
        }
    }

    result
}

/// 優先度に対応する色付きの文字列を作成
#[allow(dead_code)]
pub fn format_priority(priority: Priority) -> ColoredString {
    match priority {
        Priority::High => "高".red().bold(),
        Priority::Medium => "中".yellow(),
        Priority::Low => "低".normal(),
    }
}

/// 統計情報の表示用文字列を作成
pub fn format_stats(stats: &std::collections::HashMap<String, i64>) -> String {
    let total = stats.get("total").unwrap_or(&0);
    let completed = stats.get("completed").unwrap_or(&0);
    let active = stats.get("active").unwrap_or(&0);
    let overdue = stats.get("overdue").unwrap_or(&0);
    let due_today = stats.get("due_today").unwrap_or(&0);
    
    let priority_1 = stats.get("priority_1").unwrap_or(&0);
    let priority_2 = stats.get("priority_2").unwrap_or(&0);
    let priority_3 = stats.get("priority_3").unwrap_or(&0);
    
    let completion_rate = if *total > 0 {
        (*completed as f64 / *total as f64) * 100.0
    } else {
        0.0
    };
    
    format!(
        r#"📊 タスク統計

総タスク数: {}
完了済み: {} ({:.1}%)
未完了: {}
期限切れ: {}
今日が期限: {}

優先度:
  高: {}
  中: {}
  低: {}"#,
        total.to_string().bold(),
        completed.to_string().green(),
        completion_rate,
        active.to_string().blue(),
        overdue.to_string().red(),
        due_today.to_string().yellow(),
        priority_3.to_string().red(),
        priority_2.to_string().yellow(),
        priority_1.to_string().normal(),
    )
} 
