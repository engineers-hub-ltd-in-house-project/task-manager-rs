use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("データベースエラー: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("IOエラー: {0}")]
    IoError(#[from] std::io::Error),

    #[error("不正な日付形式: {0}")]
    InvalidDateFormat(String),

    #[error("不正な優先度: {0}. 優先度は 1, 2, 3 のいずれかである必要があります")]
    InvalidPriority(i32),

    #[error("タスクが見つかりません: ID {0}")]
    TaskNotFound(i64),

    #[error("CSVエラー: {0}")]
    CsvError(#[from] csv::Error),

    #[error("JSONエラー: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("不正な引数: {0}")]
    InvalidArgument(String),

    #[error("タグが見つかりません: {0}")]
    #[allow(dead_code)]
    TagNotFound(String),

    #[error("不明なエラー: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, TaskError>; 
