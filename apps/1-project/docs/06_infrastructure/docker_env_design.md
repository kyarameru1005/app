# docker_env_design.md

## 1. 目的

本ファイルは、開発環境における Docker / Docker Compose 構成および  
envファイルによる設定値管理方針を整理することを目的とする。

---

## 2. 構成方針

- Docker Compose を利用して開発環境を起動する
- コンテナは以下の3つで構成する
  - `frontend`
  - `backend`
  - `db`
- 構成情報は Docker Compose に記載する
- 環境ごとに変わる設定値は envファイルで管理する

---

## 3. コンテナ構成

### 3.1 frontend コンテナ

#### 役割
- Next.js アプリケーションの実行
- 画面表示
- ユーザー操作受付
- backend API 呼び出し

#### 使用技術
- TypeScript
- Next.js
- Node.js

#### 公開ポート
- `3000`

#### 接続先
- backend API

#### 設定管理
- envファイルで管理する

#### 開発環境のAPI接続先
- `NEXT_PUBLIC_API_BASE_URL=http://localhost:8080`

---

### 3.2 backend コンテナ

#### 役割
- Rust + axum による API サーバの実行
- 認証処理
- メモAPI処理
- カテゴリAPI処理
- DBアクセス

#### 使用技術
- Rust
- axum
- sqlx

#### 公開ポート
- `8080`

#### 接続先
- PostgreSQL

#### 設定管理
- envファイルで管理する

#### DB接続設定
- `DB_HOST`
- `DB_PORT`
- `DB_NAME`
- `DB_USER`
- `DB_PASSWORD`

---

### 3.3 db コンテナ

#### 役割
- PostgreSQL によるデータ保存

#### 使用技術
- PostgreSQL

#### 公開ポート
- `5432`

#### 保存対象
- `users`
- `notes`
- `categories`

#### 設定管理
- envファイルで管理する

#### 永続化
- あり

---

## 4. コンテナ間の関係

```text
ブラウザ
  ↓
frontend (3000)
  ↓ API呼び出し
backend (8080)
  ↓ DB接続
db (5432)
````

### 通信関係

* ブラウザは `frontend` にアクセスする
* `frontend` は `backend` API を呼び出す
* `backend` は `db` に接続してデータを読み書きする

---

## 5. Docker Compose に記載する内容

Docker Compose には、以下の構成情報を記載する。

* サービス名

  * `frontend`
  * `backend`
  * `db`
* 各コンテナの役割
* 公開ポート
* 依存関係
* 永続化設定
* envファイルを読み込む設定

### Compose の役割

* コンテナ構成を定義する
* 複数コンテナをまとめて起動する
* 開発環境の再現を容易にする

---

## 6. envファイルに記載する内容

envファイルには、環境ごとに変化する設定値を記載する。

### 6.1 frontend 用

* `NEXT_PUBLIC_API_BASE_URL`

### 6.2 backend 用

* `DB_HOST`
* `DB_PORT`
* `DB_NAME`
* `DB_USER`
* `DB_PASSWORD`

### 6.3 db 用

* `POSTGRES_DB`
* `POSTGRES_USER`
* `POSTGRES_PASSWORD`

### envファイルの役割

* コードに設定値を直書きしない
* 環境差分を吸収する
* 設定変更を容易にする

---

## 7. frontend 設定方針

* frontend は Next.js アプリケーションを実行する
* 開発環境では `3000` 番ポートで起動する
* backend API の接続先は envファイルで管理する
* 開発環境では以下を利用する

```env
NEXT_PUBLIC_API_BASE_URL=http://localhost:8080
```

### 補足

* フロントエンドから利用する環境変数は `NEXT_PUBLIC_` プレフィックスを付与する

---

## 8. backend 設定方針

* backend は Rust + axum + sqlx を利用して API サーバを実行する
* 開発環境では `8080` 番ポートで起動する
* PostgreSQL 接続設定は分割して管理する
* 設定値は envファイルで持つ

### backend の DB接続設定項目

* `DB_HOST`
* `DB_PORT`
* `DB_NAME`
* `DB_USER`
* `DB_PASSWORD`

### 開発環境の例

```env
DB_HOST=db
DB_PORT=5432
DB_NAME=memo_app
DB_USER=postgres
DB_PASSWORD=postgres
```

### 方針

* 接続情報は `DATABASE_URL` のような1本管理ではなく、分割して管理する
* 理由は、学習時に内容を理解しやすくするためである

---

## 9. db 設定方針

* db コンテナでは PostgreSQL を実行する
* 開発環境では `5432` 番ポートを利用する
* DBデータは永続化する
* コンテナ再作成時もデータが保持されることを前提とする

### db の初期設定値

* `POSTGRES_DB`
* `POSTGRES_USER`
* `POSTGRES_PASSWORD`

### 開発環境の例

```env
POSTGRES_DB=memo_app
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
```

---

## 10. 永続化方針

* db コンテナのデータは永続化する
* コンテナ再作成時も以下のデータが保持されること

  * ユーザーデータ
  * メモデータ
  * カテゴリデータ

---

## 11. 運用方針

* Docker Compose により開発環境を起動できること
* 設定値は envファイルで管理すること
* frontend / backend / db の役割を明確に分離すること

---

## 12. 補足

* 本ファイルは Docker / env に関する構成の正本とする
* Rustバックエンド内部のディレクトリ構成は `backend_structure_design.md` を参照する
