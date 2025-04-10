# task-manager-rs

Rust で実装されたコマンドラインタスク管理アプリケーションです。SQLite データベースを使用してタスクを管理し、コマンドラインから簡単にタスクの作成、表示、更新、削除（CRUD操作）を行うことができます。

## 機能

- タスクの追加（タイトル、説明、期限、優先度、タグの設定）
- タスクの一覧表示
  - すべてのタスク表示
  - 優先度、期限、タグによるフィルタリング
- タスクの詳細表示
- タスクの更新（タイトル、説明、期限、優先度、タグの変更）
- タスクの完了/未完了の切り替え
- タスクの削除（個別削除または完了済みタスクの一括削除）
- タグ付け機能
- 統計情報の表示
- データのエクスポート/インポート（JSON, CSV形式）

## インストール

### 前提条件

- Rust と Cargo がインストールされていること（[rustup.rs](https://rustup.rs/)からインストール可能）

### ビルド方法

```bash
git clone https://github.com/your-username/task-manager-rs.git
cd task-manager-rs
cargo build --release
```

ビルド後、実行ファイルは `target/release/task-manager-rs` に生成されます。

### インストール（オプション）

システムのパスに追加するには以下のコマンドを実行します：

```bash
cargo install --path .
```

## 使用方法

### 基本的なコマンド構造

アプリケーションは `cargo run -- [コマンド] [オプション]` または、インストール後は `task-manager-rs [コマンド] [オプション]` として実行できます。

### タスクの追加

```bash
# 基本的なタスク追加
cargo run -- add "買い物に行く"

# 詳細情報付きでタスクを追加
cargo run -- add "レポートを書く" --description "プロジェクトの月次レポート" --priority 3 --due "2023-12-31 17:00" --tags "仕事,重要"

# エイリアスを使った短い書き方
cargo run -- a "牛乳を買う" -p 1 -t "2023-12-25" -g "食料品,買い物"
```

### タスクの一覧表示

```bash
# 未完了タスクの一覧
cargo run -- list

# すべてのタスク（完了済み含む）を表示
cargo run -- list --all

# 優先度でフィルタリング（1: 低, 2: 中, 3: 高）
cargo run -- list --priority 3

# 今日が期限のタスクを表示
cargo run -- list --due-today

# タグでフィルタリング
cargo run -- list --tags "仕事"

# エイリアスを使った短い書き方
cargo run -- ls
```

### タスクの詳細表示

```bash
# IDを指定してタスクの詳細を表示
cargo run -- show 1

# エイリアスを使った短い書き方
cargo run -- s 1
```

### タスクの更新

```bash
# タイトルを更新
cargo run -- update 1 --title "新しいタイトル"

# 説明を更新
cargo run -- update 1 --description "新しい説明"

# 期限を更新
cargo run -- update 1 --due "2024-01-15"

# 期限を削除
cargo run -- update 1 --remove-due

# 優先度を更新
cargo run -- update 1 --priority 2

# タグを更新
cargo run -- update 1 --tags "個人,買い物"
cargo run -- update 1 -g "個人,買い物"

# エイリアスを使った短い書き方
cargo run -- u 1 -t "2024-01-01" -g "個人,優先"
```

### タスクの完了/未完了

```bash
# タスクを完了状態に設定
cargo run -- complete 1

# タスクを未完了状態に戻す
cargo run -- uncomplete 1

# エイリアスを使った短い書き方
cargo run -- c 1
cargo run -- uc 1
```

### タスクの削除

```bash
# 特定のタスクを削除
cargo run -- delete 1

# 完了済みタスクをすべて削除
cargo run -- delete --completed

# エイリアスを使った短い書き方
cargo run -- rm 1
```

### 統計情報の表示

```bash
# タスクの統計情報を表示
cargo run -- stats

# エイリアスを使った短い書き方
cargo run -- st
```

### データのエクスポート・インポート

```bash
# JSONフォーマットでエクスポート
cargo run -- export tasks.json

# CSVフォーマットでエクスポート
cargo run -- export tasks.csv --format csv

# データのインポート
cargo run -- import tasks.json
cargo run -- import tasks.csv
```

## デモ

以下は簡単な使用例です：

```bash
# タスクを追加
cargo run -- add "ミーティングの準備" -p 3 -t "2023-12-20 14:00" -g "仕事,会議"
# 出力: タスクを追加しました（ID: 1）

# タスク一覧を確認
cargo run -- list
# 出力例:
# 全1件のタスク:
# [!]   1 ミーティングの準備 #仕事 #会議 期限: 2日後

# タグでフィルタリング
cargo run -- list --tags "仕事"
# 出力例:
# 全1件のタスク:
# [!]   1 ミーティングの準備 #仕事 #会議 期限: 2日後

# タスクの詳細を表示
cargo run -- show 1
# 出力例:
# [!]   1 ミーティングの準備 #仕事 #会議 期限: 2日後
#     作成: 5分前

# タスクを完了
cargo run -- complete 1
# 出力例:
# タスクを完了としてマークしました（ID: 1）
# [✓]   1 ミーティングの準備 #仕事 #会議

# 統計を確認
cargo run -- stats
# 出力例:
# 📊 タスク統計
#
# 総タスク数: 1
# 完了済み: 1 (100.0%)
# 未完了: 0
# 期限切れ: 0
# 今日が期限: 0
#
# 優先度:
#   高: 0
#   中: 0
#   低: 0
```

## 実行結果のサンプル

実際のアプリケーション実行例：

```
# タスクの追加
$ cargo run -- add "買い物に行く" -g "買い物,急ぎ"
タスクを追加しました（ID: 1）

# タスク一覧の表示
$ cargo run -- list
全1件のタスク:
[ ]   1 買い物に行く #買い物 #急ぎ

# 優先度の高いタスクを追加
$ cargo run -- add "重要な会議" -p 3 -t "2023-12-20 14:00" -g "仕事,会議"
タスクを追加しました（ID: 2）

# 全タスク表示
$ cargo run -- list
全2件のタスク:
[!]   2 重要な会議 #仕事 #会議 期限: 2023-12-20 14:00
[ ]   1 買い物に行く #買い物 #急ぎ

# タスクを完了
$ cargo run -- complete 1
タスクを完了としてマークしました（ID: 1）
[✓]   1 買い物に行く #買い物 #急ぎ

# 完了済みも含めてすべてのタスクを表示
$ cargo run -- list --all
全2件のタスク:
[!]   2 重要な会議 #仕事 #会議 期限: 2023-12-20 14:00
[✓]   1 買い物に行く #買い物 #急ぎ
```

## テストの実行

アプリケーションのテストを実行するには以下のコマンドを使用します：

```bash
cargo test
```

テスト実行結果の例：

```
❯ cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running unittests src/lib.rs (target/debug/deps/task_manager_rs-e7bc24962e2b2a21)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/task_manager_rs-7b454abf278cdbcc)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-9e544cbdb63509ea)

running 3 tests
test test_task_model ... ok
test test_repository_crud ... ok
test test_export_import ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s

   Doc-tests task_manager_rs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

テストでは以下の項目が検証されています：

1. `test_task_model` - タスクモデルの基本的な機能をテスト
2. `test_repository_crud` - データベースリポジトリのCRUD操作をテスト
3. `test_export_import` - タスクのエクスポートとインポート機能をテスト

各テストは一時的なデータベースを使用して実行されるため、実際のアプリケーションデータに影響を与えることはありません。

## データの保存場所

タスクデータは SQLite データベースに保存され、以下のディレクトリに格納されます：
- Linux/macOS: `~/.task-manager-rs/tasks.db`
- Windows: `C:\Users\<username>\.task-manager-rs\tasks.db`

## 継続的インテグレーション

このプロジェクトはGitHub Actionsを使用して継続的インテグレーション（CI）を実施しています。

### ビルド・テストワークフロー

`rust-ci.yml` - プッシュやプルリクエスト時に自動的に実行される基本的なCIパイプライン
- コードのビルド
- テストの実行（インテグレーションテストを含む）

このシンプルなワークフローにより、コードの変更が既存の機能を壊していないことを継続的に確認できます。

## 使用技術

- [clap](https://crates.io/crates/clap): コマンドライン引数の解析
- [rusqlite](https://crates.io/crates/rusqlite): SQLiteデータベース連携
- [chrono](https://crates.io/crates/chrono): 日付・時間の取り扱い
- [colored](https://crates.io/crates/colored): ターミナル出力の色付け
- [serde](https://crates.io/crates/serde): データシリアライズ/デシリアライズ
- [thiserror](https://crates.io/crates/thiserror): エラー管理
- [csv](https://crates.io/crates/csv): CSVファイル操作
- [home](https://crates.io/crates/home): ホームディレクトリ検出

## プロジェクト構成

```
src/
├── cli.rs           # コマンドライン引数定義
├── commands/        # コマンド実装
│   ├── add.rs       # タスク追加
│   ├── complete.rs  # タスク完了/未完了
│   ├── delete.rs    # タスク削除
│   ├── export.rs    # データエクスポート
│   ├── import.rs    # データインポート
│   ├── list.rs      # タスク一覧
│   ├── mod.rs       # モジュール定義
│   ├── show.rs      # タスク詳細表示
│   ├── stats.rs     # 統計情報
│   └── update.rs    # タスク更新
├── db/              # データベース連携
│   ├── mod.rs       # モジュール定義
│   ├── repository.rs # リポジトリ実装
│   └── schema.rs    # スキーマ定義
├── error.rs         # エラー定義
├── main.rs          # エントリーポイント
├── models/          # データモデル
│   ├── mod.rs       # モジュール定義
│   ├── tag.rs       # タグモデル
│   └── task.rs      # タスクモデル
└── utils/           # ユーティリティ
    ├── date.rs      # 日付処理
    ├── format.rs    # 表示フォーマット
    └── mod.rs       # モジュール定義
tests/               # テスト
└── integration_test.rs # 統合テスト
.github/             # GitHub関連の設定
└── workflows/       # GitHub Actions設定
    └── rust-ci.yml     # ビルド・テストワークフロー
```

## ライセンス

MIT

## 貢献

バグ報告や機能追加のリクエストは歓迎します！プルリクエストを送る前に、まずはイシューを作成してください。

## 作者

Yusuke Sato（Engineers Hub Co. ,Ltd.）

---

このプロジェクトは、Rustでのコマンドラインアプリケーション開発の学習を目的として作成されました。
