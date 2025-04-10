use chrono::Local;
use rusqlite::{params, Connection};
use std::collections::HashMap;

use crate::error::{Result, TaskError};
use crate::models::task::{Priority, Task};
use crate::models::tag::Tag;

use super::schema;

/// タスクリポジトリ - データベース操作を行うための構造体
pub struct TaskRepository {
    conn: Connection,
}

impl TaskRepository {
    /// 新しいリポジトリインスタンスを作成
    pub fn new() -> Result<Self> {
        let conn = schema::get_connection()?;
        Ok(Self { conn })
    }

    /// タスクを追加
    pub fn add_task(&mut self, task: &Task) -> Result<i64> {
        // トランザクション開始
        let tx = self.conn.transaction()?;

        // Unix タイムスタンプを取得（秒単位）
        let created_at = task.created_at.timestamp();
        let due_date = task.due_date.map(|date| date.timestamp());
        let completed_at = task.completed_at.map(|date| date.timestamp());

        // タスクをデータベースに挿入
        tx.execute(
            "INSERT INTO tasks (title, description, created_at, due_date, completed, completed_at, priority)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                task.title,
                task.description,
                created_at,
                due_date,
                task.completed,
                completed_at,
                task.priority as i32
            ],
        )?;

        // 挿入されたタスクのIDを取得
        let task_id = tx.last_insert_rowid();

        // タグを処理
        for tag_name in &task.tags {
            let tag_id = get_or_create_tag(&tx, tag_name)?;
            tx.execute(
                "INSERT OR IGNORE INTO task_tags (task_id, tag_id) VALUES (?1, ?2)",
                params![task_id, tag_id],
            )?;
        }

        // トランザクションコミット
        tx.commit()?;

        Ok(task_id)
    }

    /// 全タスクを取得
    pub fn get_all_tasks(&self, include_completed: bool) -> Result<Vec<Task>> {
        let sql = if include_completed {
            "SELECT t.id, t.title, t.description, t.created_at, t.due_date, t.completed, t.completed_at, t.priority, 
                    GROUP_CONCAT(tags.name, ',') as tags
             FROM tasks t
             LEFT JOIN task_tags ON t.id = task_tags.task_id
             LEFT JOIN tags ON task_tags.tag_id = tags.id
             GROUP BY t.id
             ORDER BY t.created_at DESC"
        } else {
            "SELECT t.id, t.title, t.description, t.created_at, t.due_date, t.completed, t.completed_at, t.priority, 
                    GROUP_CONCAT(tags.name, ',') as tags
             FROM tasks t
             LEFT JOIN task_tags ON t.id = task_tags.task_id
             LEFT JOIN tags ON task_tags.tag_id = tags.id
             WHERE t.completed = 0
             GROUP BY t.id
             ORDER BY t.created_at DESC"
        };

        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map([], |row| {
            Ok(Task::from_row(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
                row.get(8)?,
            ).map_err(|_e| rusqlite::Error::ExecuteReturnedResults))
        })?;

        let mut tasks = Vec::new();
        for task_result in rows {
            match task_result? {
                Ok(task) => tasks.push(task),
                Err(e) => return Err(TaskError::Unknown(e.to_string())),
            }
        }

        Ok(tasks)
    }

    /// 優先度でフィルタリングしたタスクを取得
    pub fn get_tasks_by_priority(&self, priority: Priority) -> Result<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.id, t.title, t.description, t.created_at, t.due_date, t.completed, t.completed_at, t.priority, 
                    GROUP_CONCAT(tags.name, ',') as tags
             FROM tasks t
             LEFT JOIN task_tags ON t.id = task_tags.task_id
             LEFT JOIN tags ON task_tags.tag_id = tags.id
             WHERE t.priority = ?1 AND t.completed = 0
             GROUP BY t.id
             ORDER BY t.created_at DESC"
        )?;

        let rows = stmt.query_map(params![priority as i32], |row| {
            Ok(Task::from_row(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
                row.get(8)?,
            ).map_err(|_e| rusqlite::Error::ExecuteReturnedResults))
        })?;

        let mut tasks = Vec::new();
        for task_result in rows {
            match task_result? {
                Ok(task) => tasks.push(task),
                Err(e) => return Err(TaskError::Unknown(e.to_string())),
            }
        }

        Ok(tasks)
    }

    /// 今日が期限のタスクを取得
    pub fn get_tasks_due_today(&self) -> Result<Vec<Task>> {
        let today = Local::now();
        let start_of_day = today.date_naive().and_hms_opt(0, 0, 0).unwrap();
        let end_of_day = today.date_naive().and_hms_opt(23, 59, 59).unwrap();
        
        let start_ts = start_of_day.and_local_timezone(Local).unwrap().timestamp();
        let end_ts = end_of_day.and_local_timezone(Local).unwrap().timestamp();

        let mut stmt = self.conn.prepare(
            "SELECT t.id, t.title, t.description, t.created_at, t.due_date, t.completed, t.completed_at, t.priority, 
                    GROUP_CONCAT(tags.name, ',') as tags
             FROM tasks t
             LEFT JOIN task_tags ON t.id = task_tags.task_id
             LEFT JOIN tags ON task_tags.tag_id = tags.id
             WHERE t.due_date BETWEEN ?1 AND ?2 AND t.completed = 0
             GROUP BY t.id
             ORDER BY t.due_date ASC"
        )?;

        let rows = stmt.query_map(params![start_ts, end_ts], |row| {
            Ok(Task::from_row(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
                row.get(8)?,
            ).map_err(|_e| rusqlite::Error::ExecuteReturnedResults))
        })?;

        let mut tasks = Vec::new();
        for task_result in rows {
            match task_result? {
                Ok(task) => tasks.push(task),
                Err(e) => return Err(TaskError::Unknown(e.to_string())),
            }
        }

        Ok(tasks)
    }

    /// タグでフィルタリングしたタスクを取得
    pub fn get_tasks_by_tag(&self, tag: &str) -> Result<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.id, t.title, t.description, t.created_at, t.due_date, t.completed, t.completed_at, t.priority, 
                    GROUP_CONCAT(tags.name, ',') as tags
             FROM tasks t
             JOIN task_tags ON t.id = task_tags.task_id
             JOIN tags ON task_tags.tag_id = tags.id
             WHERE tags.name = ?1 AND t.completed = 0
             GROUP BY t.id
             ORDER BY t.created_at DESC"
        )?;

        let rows = stmt.query_map(params![tag], |row| {
            Ok(Task::from_row(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
                row.get(8)?,
            ).map_err(|_e| rusqlite::Error::ExecuteReturnedResults))
        })?;

        let mut tasks = Vec::new();
        for task_result in rows {
            match task_result? {
                Ok(task) => tasks.push(task),
                Err(e) => return Err(TaskError::Unknown(e.to_string())),
            }
        }

        Ok(tasks)
    }

    /// 特定のタスクを取得
    pub fn get_task(&self, id: i64) -> Result<Task> {
        let mut stmt = self.conn.prepare(
            "SELECT t.id, t.title, t.description, t.created_at, t.due_date, t.completed, t.completed_at, t.priority, 
                    GROUP_CONCAT(tags.name, ',') as tags
             FROM tasks t
             LEFT JOIN task_tags ON t.id = task_tags.task_id
             LEFT JOIN tags ON task_tags.tag_id = tags.id
             WHERE t.id = ?1
             GROUP BY t.id"
        )?;

        let task_result = stmt.query_row(params![id], |row| {
            Ok(Task::from_row(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
                row.get(8)?,
            ).map_err(|_e| rusqlite::Error::ExecuteReturnedResults))
        });

        match task_result {
            Ok(Ok(task)) => Ok(task),
            Ok(Err(e)) => Err(TaskError::Unknown(e.to_string())),
            Err(_) => Err(TaskError::TaskNotFound(id)),
        }
    }

    /// タスクを更新
    pub fn update_task(&mut self, task: &Task) -> Result<()> {
        let id = task.id.ok_or_else(|| TaskError::InvalidArgument("タスクIDが指定されていません".into()))?;

        // トランザクション開始
        let tx = self.conn.transaction()?;

        let due_date = task.due_date.map(|date| date.timestamp());
        let completed_at = task.completed_at.map(|date| date.timestamp());

        tx.execute(
            "UPDATE tasks SET 
                title = ?1, 
                description = ?2, 
                due_date = ?3, 
                completed = ?4, 
                completed_at = ?5, 
                priority = ?6
             WHERE id = ?7",
            params![
                task.title,
                task.description,
                due_date,
                task.completed,
                completed_at,
                task.priority as i32,
                id
            ],
        )?;

        // 既存のタグ関連付けを削除
        tx.execute("DELETE FROM task_tags WHERE task_id = ?1", params![id])?;

        // 新しいタグを関連付け
        for tag_name in &task.tags {
            let tag_id = get_or_create_tag(&tx, tag_name)?;
            tx.execute(
                "INSERT INTO task_tags (task_id, tag_id) VALUES (?1, ?2)",
                params![id, tag_id],
            )?;
        }

        // トランザクションコミット
        tx.commit()?;

        Ok(())
    }

    /// タスクを完了状態に設定
    pub fn complete_task(&mut self, id: i64) -> Result<()> {
        // タスクを取得して存在を確認
        let task = self.get_task(id)?;
        
        // タスクを複製し、変更を加える
        let mut task_clone = task.clone();
        task_clone.complete();
        
        // 変更を保存
        self.update_task(&task_clone)
    }

    /// タスクを未完了状態に設定
    pub fn uncomplete_task(&mut self, id: i64) -> Result<()> {
        // タスクを取得して存在を確認
        let task = self.get_task(id)?;
        
        // タスクを複製し、変更を加える
        let mut task_clone = task.clone();
        task_clone.uncomplete();
        
        // 変更を保存
        self.update_task(&task_clone)
    }

    /// タスクを削除
    pub fn delete_task(&mut self, id: i64) -> Result<()> {
        let rows_affected = self.conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
        
        if rows_affected == 0 {
            return Err(TaskError::TaskNotFound(id));
        }
        
        Ok(())
    }

    /// 完了済みタスクをすべて削除
    pub fn delete_completed_tasks(&mut self) -> Result<usize> {
        let rows_affected = self.conn.execute("DELETE FROM tasks WHERE completed = 1", [])?;
        Ok(rows_affected)
    }

    /// 全タグを取得
    #[allow(dead_code)]
    pub fn get_all_tags(&self) -> Result<Vec<Tag>> {
        let mut stmt = self.conn.prepare("SELECT id, name FROM tags ORDER BY name")?;
        let rows = stmt.query_map([], |row| {
            Ok(Tag {
                id: Some(row.get(0)?),
                name: row.get(1)?,
            })
        })?;

        let mut tags = Vec::new();
        for tag_result in rows {
            tags.push(tag_result?);
        }

        Ok(tags)
    }

    /// 統計情報を取得
    pub fn get_stats(&self) -> Result<HashMap<String, i64>> {
        let mut stats = HashMap::new();
        
        // 総タスク数
        let total_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM tasks", 
            [], 
            |row| row.get(0)
        )?;
        stats.insert("total".to_string(), total_count);
        
        // 完了済みタスク数
        let completed_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM tasks WHERE completed = 1", 
            [], 
            |row| row.get(0)
        )?;
        stats.insert("completed".to_string(), completed_count);
        
        // 未完了タスク数
        let active_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM tasks WHERE completed = 0", 
            [], 
            |row| row.get(0)
        )?;
        stats.insert("active".to_string(), active_count);
        
        // 優先度ごとの数
        for priority in 1..=3 {
            let priority_count: i64 = self.conn.query_row(
                "SELECT COUNT(*) FROM tasks WHERE priority = ?1 AND completed = 0", 
                params![priority], 
                |row| row.get(0)
            )?;
            stats.insert(format!("priority_{}", priority), priority_count);
        }
        
        // 期限切れのタスク数
        let now = Local::now().timestamp();
        let overdue_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM tasks WHERE due_date < ?1 AND due_date IS NOT NULL AND completed = 0", 
            params![now], 
            |row| row.get(0)
        )?;
        stats.insert("overdue".to_string(), overdue_count);
        
        // 今日が期限のタスク数
        let today = Local::now();
        let start_of_day = today.date_naive().and_hms_opt(0, 0, 0).unwrap();
        let end_of_day = today.date_naive().and_hms_opt(23, 59, 59).unwrap();
        
        let start_ts = start_of_day.and_local_timezone(Local).unwrap().timestamp();
        let end_ts = end_of_day.and_local_timezone(Local).unwrap().timestamp();
        
        let due_today_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM tasks WHERE due_date BETWEEN ?1 AND ?2 AND completed = 0", 
            params![start_ts, end_ts], 
            |row| row.get(0)
        )?;
        stats.insert("due_today".to_string(), due_today_count);
        
        Ok(stats)
    }
}

/// タグの取得または作成 - トランザクション内で完結するためのヘルパー関数
fn get_or_create_tag(tx: &rusqlite::Transaction, tag_name: &str) -> Result<i64> {
    // タグが存在するか確認
    let mut stmt = tx.prepare("SELECT id FROM tags WHERE name = ?1")?;
    let tag_id = stmt.query_row(params![tag_name], |row| row.get(0));

    match tag_id {
        Ok(id) => Ok(id),
        Err(_) => {
            // タグが存在しない場合は作成
            tx.execute("INSERT INTO tags (name) VALUES (?1)", params![tag_name])?;
            Ok(tx.last_insert_rowid())
        }
    }
} 
