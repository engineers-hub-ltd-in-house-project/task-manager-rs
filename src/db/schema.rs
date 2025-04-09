use rusqlite::{Connection, Result};
use crate::error::TaskError;

/// データベーススキーマの初期化
pub fn initialize_db(conn: &mut Connection) -> Result<()> {
    // トランザクション開始
    let tx = conn.transaction()?;

    // タスクテーブル作成
    tx.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT,
            created_at INTEGER NOT NULL,
            due_date INTEGER,
            completed BOOLEAN NOT NULL DEFAULT 0,
            completed_at INTEGER,
            priority INTEGER NOT NULL DEFAULT 2
        )",
        [],
    )?;

    // タグテーブル作成
    tx.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    // タスクとタグの関連テーブル作成
    tx.execute(
        "CREATE TABLE IF NOT EXISTS task_tags (
            task_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (task_id, tag_id),
            FOREIGN KEY (task_id) REFERENCES tasks (id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
        )",
        [],
    )?;

    // インデックス作成
    tx.execute(
        "CREATE INDEX IF NOT EXISTS idx_tasks_completed ON tasks (completed)",
        [],
    )?;
    tx.execute(
        "CREATE INDEX IF NOT EXISTS idx_tasks_due_date ON tasks (due_date)",
        [],
    )?;
    tx.execute(
        "CREATE INDEX IF NOT EXISTS idx_tasks_priority ON tasks (priority)",
        [],
    )?;
    tx.execute(
        "CREATE INDEX IF NOT EXISTS idx_tags_name ON tags (name)",
        [],
    )?;

    // トランザクションコミット
    tx.commit()?;

    Ok(())
}

/// データベース接続の取得
pub fn get_connection() -> crate::error::Result<Connection> {
    let home_dir = home::home_dir().ok_or_else(|| TaskError::Unknown("ホームディレクトリを特定できません".to_string()))?;
    let db_dir = home_dir.join(".task-manager-rs");
    std::fs::create_dir_all(&db_dir).map_err(|e| TaskError::IoError(e))?;
    
    let db_path = db_dir.join("tasks.db");
    let mut conn = Connection::open(db_path)?;
    
    // 外部キー制約を有効化
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    
    // データベーススキーマの初期化
    initialize_db(&mut conn)?;
    
    Ok(conn)
} 
