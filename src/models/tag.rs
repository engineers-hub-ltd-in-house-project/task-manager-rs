use serde::{Deserialize, Serialize};

/// タグを表す構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<i64>,
    pub name: String,
}

impl Tag {
    #[allow(dead_code)]
    pub fn new(name: String) -> Self {
        Self { id: None, name }
    }
} 
