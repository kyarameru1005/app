# system_overview.md

## 1. システム概要

### 1.1 システム名
メモWebアプリ

### 1.2 システム概要
Googleアカウントでログインし、メモの作成・閲覧・編集・削除を行えるWebアプリケーションである。  
メモにはカテゴリを設定することができ、タイトルおよび本文を対象に検索できる。

### 1.3 目的
ユーザーがメモを簡単に記録・管理できるようにすることを目的とする。  
また、カテゴリ分類と検索機能により、必要なメモを見つけやすくすることを目的とする。

### 1.4 利用者
本システムの利用者は、Googleアカウントを保有するユーザーとする。

---

## 2. 技術スタック

### 2.1 フロントエンド
- TypeScript
- Next.js

### 2.2 バックエンド
- Rust
- axum

### 2.3 データベース
- PostgreSQL

### 2.4 DBライブラリ
- sqlx

### 2.5 認証
- Googleアカウントログイン
- Google Identity Services
- OpenID Connect

### 2.6 開発 / 実行環境
- Docker
- Docker Compose

---

## 3. 機能一覧

- メモ一覧表示
- メモ詳細表示
- メモ新規作成
- メモ編集
- メモ削除
- カテゴリ分類
- タイトル・本文検索

---

## 4. 画面一覧

- ログイン画面
- メモ一覧画面
- メモ詳細画面
- メモ新規作成画面
- メモ編集画面
- メモ削除確認画面
- カテゴリ一覧画面
- カテゴリ作成画面
- カテゴリ編集画面

---

## 5. 全体構成

### 5.1 システム構成
- フロントエンド: Next.js
- バックエンド: Rust + axum
- データベース: PostgreSQL

### 5.2 インフラ構成
- Docker Compose により以下の3コンテナを起動する
  - frontend
  - backend
  - db

### 5.3 コンテナ間の関係
- ブラウザは frontend にアクセスする
- frontend は backend API を呼び出す
- backend は db に接続してデータを読み書きする

---

## 6. 設計書構成

本プロジェクトでは、設計書を用途ごとに分割して管理する。

- `docs/00_overview/system_overview.md`
  - システム概要、技術スタック、機能一覧、画面一覧、全体構成
- `docs/01_auth/auth_design.md`
  - 認証設計
- `docs/01_auth/login_screen_design.md`
  - ログイン画面設計
- `docs/01_auth/auth_api_design.md`
  - 認証系API設計
- `docs/02_memo/memo_screen_design.md`
  - メモ系画面設計
- `docs/02_memo/memo_api_design.md`
  - メモ系API設計
- `docs/03_category/category_screen_design.md`
  - カテゴリ系画面設計
- `docs/03_category/category_api_design.md`
  - カテゴリ系API設計
- `docs/04_database/db_design.md`
  - DB設計
- `docs/05_common/common_api_design.md`
  - API共通設計
- `docs/05_common/error_message_design.md`
  - エラーメッセージ設計
- `docs/06_infrastructure/docker_env_design.md`
  - Docker / env 設計
- `docs/06_infrastructure/backend_structure_design.md`
  - バックエンド構成設計
- `docs/07_non_functional/non_functional_requirements.md`
  - 非機能要件