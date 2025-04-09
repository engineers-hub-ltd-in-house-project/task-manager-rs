use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt;

/// タスクの優先度を表す列挙型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    Low = 1,
    Medium = 2,
    High = 3,
}

impl Priority {
    pub fn from_i32(value: i32) -> Result<Self, String> {
        match value {
            1 => Ok(Priority::Low),
            2 => Ok(Priority::Medium),
            3 => Ok(Priority::High),
            _ => Err(format!("不正な優先度: {}. 1, 2, 3 のいずれかを指定してください", value)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Priority::Low => "低".to_string(),
            Priority::Medium => "中".to_string(),
            Priority::High => "高".to_string(),
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// タスクを表す構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Local>,
    pub due_date: Option<DateTime<Local>>,
    pub completed: bool,
    pub completed_at: Option<DateTime<Local>>,
    pub priority: Priority,
    pub tags: Vec<String>,
}

impl Task {
    pub fn new(
        title: String,
        description: Option<String>,
        due_date: Option<DateTime<Local>>,
        priority: Priority,
        tags: Vec<String>,
    ) -> Self {
        Self {
            id: None,
            title,
            description,
            created_at: Local::now(),
            due_date,
            completed: false,
            completed_at: None,
            priority,
            tags,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
        self.completed_at = Some(Local::now());
    }

    pub fn uncomplete(&mut self) {
        self.completed = false;
        self.completed_at = None;
    }

    pub fn is_due_today(&self) -> bool {
        if let Some(due) = self.due_date {
            let today = Local::now().date_naive();
            due.date_naive() == today
        } else {
            false
        }
    }

    pub fn is_overdue(&self) -> bool {
        if self.completed {
            return false;
        }

        if let Some(due) = self.due_date {
            due < Local::now()
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    #[allow(dead_code)]
    pub fn format_due_date(&self) -> String {
        match self.due_date {
            Some(date) => date.format("%Y-%m-%d %H:%M").to_string(),
            None => "なし".to_string(),
        }
    }

    pub fn from_row(
        id: i64,
        title: String,
        description: Option<String>,
        created_at: i64,
        due_date: Option<i64>,
        completed: bool,
        completed_at: Option<i64>,
        priority: i32,
        tags_str: Option<String>,
    ) -> Result<Self, String> {
        // Unix タイムスタンプから DateTime<Local> に変換
        let created_at_dt = DateTime::from_timestamp(created_at, 0)
            .ok_or_else(|| format!("不正な作成日時: {}", created_at))?;
        let created_at_local = created_at_dt.with_timezone(&Local);

        // due_date と completed_at も同様に変換
        let due_date_local = due_date.map(|ts| {
            let dt = DateTime::from_timestamp(ts, 0).unwrap();
            dt.with_timezone(&Local)
        });

        let completed_at_local = completed_at.map(|ts| {
            let dt = DateTime::from_timestamp(ts, 0).unwrap();
            dt.with_timezone(&Local)
        });

        // Priority の変換
        let priority = Priority::from_i32(priority)?;

        // タグの変換
        let tags = tags_str
            .map(|s| s.split(',').map(|t| t.trim().to_string()).collect())
            .unwrap_or_else(Vec::new);

        Ok(Self {
            id: Some(id),
            title,
            description,
            created_at: created_at_local,
            due_date: due_date_local,
            completed,
            completed_at: completed_at_local,
            priority,
            tags,
        })
    }
} 
