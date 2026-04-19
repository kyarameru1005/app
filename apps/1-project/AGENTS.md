# AGENTS.md

## 1. Project overview

このプロジェクトは、Googleアカウントでログインし、メモの作成・閲覧・編集・削除を行う Web アプリケーションである。  
メモにはカテゴリを設定でき、タイトルおよび本文を対象に検索できる。

### Tech stack
- Frontend: TypeScript, Next.js
- Backend: Rust, axum
- Database: PostgreSQL
- DB library: sqlx
- Authentication: Google Identity Services, OpenID Connect
- Development environment: Docker, Docker Compose

---

## 2. Rules for agents

- 原則として日本語で説明・報告する
- 勝手に機能追加や仕様変更をしない
- 不明な点を推測で埋めすぎない
- 実装や修正の前に、対象機能に対応する設計ファイルを確認する
- 詳細仕様は `docs/` 配下を参照する
- 正本は `docs/` 配下の設計ファイルとする

---

## 3. File map

### `docs/00_overview/system_overview.md`
- システム概要
- 技術スタック
- 機能一覧
- 画面一覧
- 全体構成

### `docs/01_auth/auth_design.md`
- 認証方式
- ユーザー識別
- セッション管理
- 未ログイン時制御
- 権限制御

### `docs/01_auth/login_screen_design.md`
- ログイン画面の詳細仕様

### `docs/01_auth/auth_api_design.md`
- 認証系API設計

### `docs/02_memo/memo_screen_design.md`
- メモ系画面設計

### `docs/02_memo/memo_api_design.md`
- メモ系API設計

### `docs/03_category/category_screen_design.md`
- カテゴリ系画面設計

### `docs/03_category/category_api_design.md`
- カテゴリ系API設計

### `docs/04_database/db_design.md`
- DB設計
- テーブル定義
- 制約
- バリデーション

### `docs/05_common/common_api_design.md`
- API共通設計
- ステータスコード方針
- API一覧

### `docs/05_common/error_message_design.md`
- エラーメッセージ設計

### `docs/06_infrastructure/docker_env_design.md`
- Docker / Compose / env設計

### `docs/06_infrastructure/backend_structure_design.md`
- Rustバックエンド構成設計

### `docs/07_non_functional/non_functional_requirements.md`
- 非機能要件

---

## 4. Which file to read first

### 画面修正をする場合
最初に確認する:
- `docs/02_memo/memo_screen_design.md`
- `docs/03_category/category_screen_design.md`
- `docs/01_auth/login_screen_design.md`

### API修正をする場合
最初に確認する:
- `docs/02_memo/memo_api_design.md`
- `docs/03_category/category_api_design.md`
- `docs/01_auth/auth_api_design.md`

### DB修正をする場合
最初に確認する:
- `docs/04_database/db_design.md`

### 認証修正をする場合
最初に確認する:
- `docs/01_auth/auth_design.md`
- `docs/01_auth/auth_api_design.md`
- `docs/01_auth/login_screen_design.md`

### Docker / 環境設定を修正する場合
最初に確認する:
- `docs/06_infrastructure/docker_env_design.md`
- `docs/06_infrastructure/backend_structure_design.md`

---

## 5. Source of truth

- 詳細仕様の正本は `docs/` 配下の設計ファイルとする
- `AGENTS.md` は概要と参照先を示す案内ファイルとする
- 仕様に関する判断は、まず `docs/` を参照すること