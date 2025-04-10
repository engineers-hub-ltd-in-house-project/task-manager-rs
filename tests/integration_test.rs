// 外部クレートとして明示的に宣言
extern crate task_manager_rs;
extern crate tempfile;

use task_manager_rs::models::task::{Task, Priority};
use task_manager_rs::db::TaskRepository;
use task_manager_rs::commands;
use std::env;
use tempfile::TempDir;
use std::path::Path;

// テスト用のヘルパー関数
fn setup_test_db() -> (TempDir, String) {
    // 一時ディレクトリを作成
    let temp_dir = tempfile::tempdir().expect("一時ディレクトリを作成できませんでした");
    let db_path = temp_dir.path().join("test_tasks.db");
    let db_path_str = db_path.to_str().unwrap().to_string();
    
    // 環境変数を設定してテスト用データベースを指定
    env::set_var("TASK_MANAGER_TEST_DB", &db_path_str);
    
    (temp_dir, db_path_str)
}

fn teardown() {
    env::remove_var("TASK_MANAGER_TEST_DB");
}

#[test]
fn test_task_model() {
    // タスクモデルのテスト
    let task = Task::new(
        "テストタスク".to_string(),
        Some("これはテストタスクです".to_string()),
        None,
        Priority::High,
        vec!["テスト".to_string(), "タスク".to_string()]
    );

    assert_eq!(task.title, "テストタスク");
    assert_eq!(task.description, Some("これはテストタスクです".to_string()));
    assert_eq!(task.priority, Priority::High);
    assert_eq!(task.completed, false);
    assert_eq!(task.completed_at, None);
    assert_eq!(task.tags, vec!["テスト".to_string(), "タスク".to_string()]);
    
    // 完了状態のテスト
    let mut task2 = Task::new(
        "完了テスト".to_string(),
        None,
        None,
        Priority::Medium,
        vec![]
    );
    
    task2.complete();
    assert!(task2.completed);
    assert!(task2.completed_at.is_some());
    
    task2.uncomplete();
    assert!(!task2.completed);
    assert!(task2.completed_at.is_none());
}

#[test]
fn test_repository_crud() {
    // テスト環境のセットアップ
    let (temp_dir, _) = setup_test_db();
    let mut repo = TaskRepository::new().expect("リポジトリを作成できませんでした");
    
    // テスト用タスクの作成
    let task = Task::new(
        "リポジトリテスト".to_string(),
        Some("リポジトリのテスト用タスク".to_string()),
        None,
        Priority::Medium,
        vec!["テスト".to_string()]
    );
    
    // タスクの追加テスト
    let task_id = repo.add_task(&task).expect("タスクを追加できませんでした");
    assert!(task_id > 0);
    
    // タスクの取得テスト
    let saved_task = repo.get_task(task_id).expect("タスクを取得できませんでした");
    assert_eq!(saved_task.title, "リポジトリテスト");
    assert_eq!(saved_task.priority, Priority::Medium);
    
    // タスクの更新テスト
    let mut updated_task = saved_task.clone();
    updated_task.title = "更新されたタスク".to_string();
    repo.update_task(&updated_task).expect("タスクを更新できませんでした");
    
    // タスクの完了テスト
    repo.complete_task(task_id).expect("タスクを完了できませんでした");
    let completed_task = repo.get_task(task_id).expect("完了済みタスクを取得できませんでした");
    assert!(completed_task.completed);
    
    // タスクの未完了化テスト
    repo.uncomplete_task(task_id).expect("タスクを未完了に戻せませんでした");
    
    // クリーンアップ
    teardown();
    drop(temp_dir);
}

#[test]
fn test_export_import() {
    // テスト環境のセットアップ
    let (temp_dir, _) = setup_test_db();
    let mut repo = TaskRepository::new().expect("リポジトリを作成できませんでした");
    
    // タスクを追加
    let task = Task::new(
        "エクスポートテスト".to_string(),
        Some("エクスポート用テストタスク".to_string()),
        None,
        Priority::High,
        vec!["テスト".to_string()]
    );
    
    repo.add_task(&task).expect("タスクを追加できませんでした");
    
    // エクスポート用の一時ファイル
    let export_path = temp_dir.path().join("export_test.json").to_str().unwrap().to_string();
    
    // エクスポートをテスト
    commands::export_tasks(
        export_path.clone(),
        "json".to_string()
    ).expect("エクスポートに失敗しました");
    
    // エクスポートされたファイルが存在することを確認
    assert!(Path::new(&export_path).exists());
    
    // 環境を一度クリーンアップ
    teardown();
    
    // 新しいテスト環境をセットアップ
    let (new_temp_dir, _) = setup_test_db();
    
    // JSONファイルからインポート
    commands::import_tasks(
        export_path
    ).expect("インポートに失敗しました");
    
    // インポート後、タスクが追加されていることを確認
    let new_repo = TaskRepository::new().unwrap();
    let tasks = new_repo.get_all_tasks(true).unwrap();
    assert!(!tasks.is_empty());
    
    // クリーンアップ
    teardown();
    drop(temp_dir);
    drop(new_temp_dir);
} 
