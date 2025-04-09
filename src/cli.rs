use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "task")]
#[command(about = "タスク管理ツール", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// 新しいタスクを追加
    #[command(visible_alias = "a")]
    Add {
        /// タスクのタイトル
        title: String,
        
        /// タスクの説明
        #[arg(short, long)]
        description: Option<String>,
        
        /// タスクの期限日時（YYYY-MM-DD または YYYY-MM-DD HH:MM 形式）
        #[arg(short = 't', long)]
        due: Option<String>,
        
        /// タスクの優先度（1: 低, 2: 中, 3: 高）
        #[arg(short, long, default_value = "2")]
        priority: i32,
        
        /// タスクのタグ（カンマ区切り）
        #[arg(short = 'g', long)]
        tags: Option<String>,
    },
    
    /// タスクの一覧を表示
    #[command(visible_alias = "ls")]
    List {
        /// 完了済みタスクも含めて表示
        #[arg(short, long)]
        all: bool,
        
        /// 優先度でフィルタリング（1: 低, 2: 中, 3: 高）
        #[arg(short, long)]
        priority: Option<i32>,
        
        /// 今日が期限のタスクだけを表示
        #[arg(long)]
        due_today: bool,
        
        /// タグでフィルタリング
        #[arg(short, long)]
        tags: Option<String>,
    },
    
    /// 特定のタスクの詳細を表示
    #[command(visible_alias = "s")]
    Show {
        /// タスクのID
        id: i64,
    },
    
    /// タスクを更新
    #[command(visible_alias = "u")]
    Update {
        /// 更新するタスクのID
        id: i64,
        
        /// 新しいタイトル
        #[arg(short, long)]
        title: Option<String>,
        
        /// 新しい説明
        #[arg(short, long)]
        description: Option<String>,
        
        /// 新しい期限日時（YYYY-MM-DD または YYYY-MM-DD HH:MM 形式）
        #[arg(short = 't', long)]
        due: Option<String>,
        
        /// 期限を削除
        #[arg(long)]
        remove_due: bool,
        
        /// 新しい優先度（1: 低, 2: 中, 3: 高）
        #[arg(short, long)]
        priority: Option<i32>,
        
        /// 新しいタグ（カンマ区切り）
        #[arg(short = 'g', long)]
        tags: Option<String>,
    },
    
    /// タスクを完了状態に設定
    #[command(visible_alias = "c")]
    Complete {
        /// 完了するタスクのID
        id: i64,
    },
    
    /// タスクを未完了状態に設定
    #[command(visible_alias = "uc")]
    Uncomplete {
        /// 未完了に戻すタスクのID
        id: i64,
    },
    
    /// タスクを削除
    #[command(visible_alias = "rm")]
    Delete {
        /// 削除するタスクのID（指定しない場合は --completed が必要）
        id: Option<i64>,
        
        /// 完了済みタスクをすべて削除
        #[arg(short, long)]
        completed: bool,
    },
    
    /// タスクの統計情報を表示
    #[command(visible_alias = "st")]
    Stats,
    
    /// タスクデータをエクスポート
    Export {
        /// 出力ファイル名
        file: String,
        
        /// 出力フォーマット (json, csv)
        #[arg(short, long, default_value = "json")]
        format: String,
    },
    
    /// タスクデータをインポート
    Import {
        /// 入力ファイル名
        file: String,
    },
} 
