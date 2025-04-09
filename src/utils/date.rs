use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime};
use crate::error::{Result, TaskError};

/// 日付文字列からDateTime<Local>を作成
/// 形式: YYYY-MM-DD または YYYY-MM-DD HH:MM
pub fn parse_date(date_str: &str) -> Result<DateTime<Local>> {
    // 時間が指定されているかチェック
    let contains_time = date_str.contains(':');
    
    if contains_time {
        // YYYY-MM-DD HH:MM 形式
        match NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M") {
            Ok(dt) => Ok(DateTime::<Local>::from_naive_utc_and_offset(dt, *Local::now().offset())),
            Err(_) => Err(TaskError::InvalidDateFormat(format!(
                "不正な日付形式: {}. 正しい形式は YYYY-MM-DD HH:MM です", 
                date_str
            ))),
        }
    } else {
        // YYYY-MM-DD 形式 (時間は 00:00 とする)
        match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            Ok(date) => {
                let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
                let dt = NaiveDateTime::new(date, time);
                Ok(DateTime::<Local>::from_naive_utc_and_offset(dt, *Local::now().offset()))
            },
            Err(_) => Err(TaskError::InvalidDateFormat(format!(
                "不正な日付形式: {}. 正しい形式は YYYY-MM-DD です", 
                date_str
            ))),
        }
    }
}

/// 日時を人間が読みやすい形式に変換
pub fn format_datetime(date: DateTime<Local>) -> String {
    date.format("%Y-%m-%d %H:%M").to_string()
}

/// 相対的な日時表現を取得（例: "2時間前", "昨日", "3日後"）
pub fn get_relative_time(date: DateTime<Local>) -> String {
    let now = Local::now();
    let duration = now.signed_duration_since(date);
    
    if duration.num_seconds().abs() < 60 {
        return "たった今".to_string();
    }
    
    if duration.num_minutes().abs() < 60 {
        let minutes = duration.num_minutes();
        if minutes > 0 {
            return format!("{}分前", minutes);
        } else {
            return format!("{}分後", minutes.abs());
        }
    }
    
    if duration.num_hours().abs() < 24 {
        let hours = duration.num_hours();
        if hours > 0 {
            return format!("{}時間前", hours);
        } else {
            return format!("{}時間後", hours.abs());
        }
    }
    
    let days = duration.num_days();
    if days.abs() < 7 {
        if days == 1 {
            return "昨日".to_string();
        } else if days == -1 {
            return "明日".to_string();
        } else if days > 0 {
            return format!("{}日前", days);
        } else {
            return format!("{}日後", days.abs());
        }
    }
    
    if days.abs() < 30 {
        let weeks = days / 7;
        if weeks > 0 {
            return format!("{}週間前", weeks);
        } else {
            return format!("{}週間後", weeks.abs());
        }
    }
    
    // それ以外は正確な日付を返す
    format_datetime(date)
} 
