# Task List

このファイルは、このアプリの実装タスクを管理する正本。
`docs/implementation-plan.md` を実装可能な単位に分解し、完了した項目を順にチェックしていく。

## 運用ルール

- 新規タスク追加時にこの一覧へ記録する。
- 未着手は `- [ ]`、完了は `- [x]` を使う。
- 進行中は行末に `(doing)` を付ける。
- 優先度は `high` / `medium` / `low` を使う。
- 実装完了時は、対応タスクを完了に更新し、検証結果も実装報告に残す。

## タスク一覧

- [x] `T-001` [high] `frontend` / `backend` / `db` の基本ディレクトリ構成と Docker Compose の雛形を作成する
- [x] `T-002` [high] frontend / backend / db の env ファイル雛形と設定読み込み方針を実装する
- [x] `T-003` [high] PostgreSQL の初期スキーマとして `users` / `categories` / `notes` テーブルを作成する
- [x] `T-004` [high] Rust backend の共通骨格として `main.rs`、router、handler、model、repository、error 基盤を作成する
- [x] `T-005` [high] backend の DB 接続処理と共通エラーレスポンス変換を実装する
- [ ] `T-006` [high] Google ログイン開始 / コールバック / ログアウト / `GET /api/users/me` の認証 API とセッション管理を実装する
- [ ] `T-007` [medium] Next.js のログイン画面と未ログイン時リダイレクト制御を実装する
- [ ] `T-008` [high] カテゴリ一覧取得 / 作成 / 更新 API とカテゴリ repository を実装する
- [ ] `T-009` [medium] カテゴリ一覧 / 作成 / 編集画面と API 連携を実装する
- [ ] `T-010` [high] メモ一覧 / 詳細 / 作成 / 更新 / 削除 API とメモ repository を実装する
- [ ] `T-011` [medium] メモ一覧画面に検索・0件表示・カテゴリ管理遷移を実装する
- [ ] `T-012` [medium] メモ詳細 / 新規作成 / 編集 / 削除確認画面と API 連携を実装する
- [ ] `T-013` [medium] エラーメッセージ表示を `docs/05_common/error_message_design.md` に合わせて統一する
- [x] `T-014` [medium] README.md にアプリ概要、起動手順、主要コマンドを追加する
- [ ] `T-015` [high] Docker 起動確認、API 動作確認、画面遷移確認を行い、未完了項目を洗い出す

## 実装順の目安

- 第1段階: `T-001` 〜 `T-005`
- 第2段階: `T-006` 〜 `T-009`
- 第3段階: `T-010` 〜 `T-013`
- 第4段階: `T-014` 〜 `T-015`
