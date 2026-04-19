# メモWebアプリ実装計画書

## Summary

`docs/` 配下の設計を正本として、このリポジトリでは「Docker で起動できる最小構成の Next.js + Rust/axum + PostgreSQL アプリ」を段階的に実装する。  
優先順は、`開発基盤` → `DB/バックエンド共通基盤` → `認証` → `カテゴリ` → `メモ` → `画面統合/品質確認` とし、各段階で API と画面を接続して動作確認できる状態を作る。

## Key Changes

- 開発基盤を先に整備する。
  - `frontend` / `backend` / `db` の 3 コンテナを Docker Compose で起動可能にする。
  - env 管理は `docs/06_infrastructure/docker_env_design.md` の分割方針に合わせる。
  - `README.md` には概要・起動手順・主要コマンドを追加する。
- DB とバックエンド骨格を作る。
  - `users` / `notes` / `categories` を `docs/04_database/db_design.md` の定義どおりに作成する。
  - backend は `docs/06_infrastructure/backend_structure_design.md` の責務分割で `routes` / `handlers` / `models` / `db` / `auth` / `errors` を用意する。
  - API の共通方針は `docs/05_common/common_api_design.md` に合わせ、JSON 入出力・HTTP ステータス・セッション由来の `user_id` 取得を統一する。
- 認証機能を実装する。
  - `POST /api/auth/google/login`
  - `GET /api/auth/google/callback`
  - `POST /api/auth/logout`
  - `GET /api/users/me`
  - Google 認証成功後は `google_sub` で `users` を upsert し、セッションに `user_id` を保存する。
  - 未ログイン時は保護画面をログイン画面へ遷移、保護 API は `401` を返す。
- カテゴリ機能を実装する。
  - API: `GET /api/categories`, `POST /api/categories`, `PUT /api/categories/{category_id}`
  - 画面: カテゴリ一覧 / 作成 / 編集
  - `category_name` は必須・100文字以内・空文字不可・重複不可で、重複は `409` を返す。
  - カテゴリは全ユーザー共通データとして扱う。
- メモ機能を実装する。
  - API: `GET /api/memos`, `GET /api/memos/{memo_id}`, `POST /api/memos`, `PUT /api/memos/{memo_id}`, `DELETE /api/memos/{memo_id}`
  - 画面: 一覧 / 詳細 / 新規作成 / 編集 / 削除確認
  - 一覧は `created_at` 降順、検索対象は `title` と `content`、ページングなし。
  - `title` は必須・200文字以内、`content` は任意、`category_id` は指定時のみ UUID と存在チェック。
  - メモはログイン中ユーザー本人のものだけ取得・更新・削除でき、他人のデータ操作は `403` または `404` 方針に従って防ぐ。
- 画面統合と UX を仕上げる。
  - ログイン画面からメモ一覧へ遷移する導線を構成する。
  - メモ画面とカテゴリ画面の遷移を `docs/02_memo/memo_screen_design.md` と `docs/03_category/category_screen_design.md` に合わせる。
  - エラーメッセージは `docs/05_common/error_message_design.md` の文言を使い回し、入力エラー・一覧取得失敗・認証失敗の表示位置も統一する。

## Public APIs / Interfaces

- 認証系
  - `POST /api/auth/google/login`
  - `GET /api/auth/google/callback`
  - `POST /api/auth/logout`
  - `GET /api/users/me`
- カテゴリ系
  - `GET /api/categories`
  - `POST /api/categories`
  - `PUT /api/categories/{category_id}`
- メモ系
  - `GET /api/memos`
  - `GET /api/memos/{memo_id}`
  - `POST /api/memos`
  - `PUT /api/memos/{memo_id}`
  - `DELETE /api/memos/{memo_id}`
- 主要データ型
  - `users(user_id, google_sub, user_name, email, created_at, updated_at)`
  - `categories(category_id, category_name)`
  - `notes(memo_id, title, content, user_id, category_id, created_at, updated_at)`

## Test Plan

- 環境起動確認
  - Docker Compose で `frontend` / `backend` / `db` が起動し、`3000/8080/5432` が利用可能であること。
- API 正常系
  - 認証後に `GET /api/users/me` がログインユーザーを返すこと。
  - カテゴリ作成・更新・一覧取得が設計どおり動くこと。
  - メモ作成・一覧・詳細・更新・削除・検索が設計どおり動くこと。
- API 異常系
  - 未ログインで保護 API にアクセスすると `401`。
  - 不正入力で `400`、重複カテゴリで `409`、存在しない対象で `404`。
  - 他ユーザーのメモ操作を拒否できること。
- 画面確認
  - ログイン済み/未ログインで画面遷移制御が正しいこと。
  - 0件表示、入力エラー、取得失敗、登録/更新/削除失敗時の文言が設計どおりであること。
  - カテゴリ未設定メモで「未分類」、本文空で「本文なし」が表示されること。

## Assumptions

- 今回の計画対象は「この `docs/` 一式に基づく初期実装全体」であり、単一機能だけの部分計画ではない。
- 現時点では実装コードが未配置のため、新規構築前提でフェーズ分割している。
- セッション実装方式の具体的ライブラリ名、Google OAuth の細部、Next.js の pages/app router 選定までは設計書にないため、実装時は既存規約がなければ学習コストの低い標準構成を採用する。
- タスク台帳はリポジトリ規約上必要だが、現状 `docs/task.md` は存在しないため、実装着手時に台帳ファイルの新設または運用ルールの補正が必要。
