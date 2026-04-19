# backend_structure_design.md

## 1. 目的

本ファイルは、Rust + axum + sqlx を用いたバックエンドのディレクトリ構成と、  
各ディレクトリの役割を整理することを目的とする。

---

## 2. 構成方針

バックエンドは **役割ごとにディレクトリを分ける構成** とする。  
目的は以下の通りである。

- どこに何を書くかをわかりやすくする
- 機能追加時に見通しを保つ
- API処理、DB処理、認証処理、エラー処理を分離する

---

## 3. ディレクトリ構成

```text
src/
  main.rs
  routes/
    mod.rs
    memo_routes.rs
    category_routes.rs
    auth_routes.rs
  handlers/
    mod.rs
    memo_handler.rs
    category_handler.rs
    auth_handler.rs
  models/
    mod.rs
    memo.rs
    category.rs
    user.rs
  db/
    mod.rs
    connection.rs
    memo_repository.rs
    category_repository.rs
    user_repository.rs
  auth/
    mod.rs
    google.rs
    session.rs
  errors/
    mod.rs
    api_error.rs
````

---

## 4. 各ディレクトリの役割

### 4.1 `main.rs`

#### 役割

* アプリケーション起動処理
* ルーター組み立て
* サーバ起動
* 環境変数読み込み
* DB接続初期化

#### 方針

* `main.rs` には処理本体を書きすぎない
* 起動処理の入口として利用する

---

### 4.2 `routes/`

#### 役割

* URLとhandlerの対応付けを記述する

#### 想定ファイル

* `memo_routes.rs`
* `category_routes.rs`
* `auth_routes.rs`

#### 方針

* ルーティングのみを記述する
* 業務処理やSQL処理は書かない

---

### 4.3 `handlers/`

#### 役割

* リクエストを受け取り、レスポンスを返すAPI処理を記述する

#### 想定ファイル

* `memo_handler.rs`
* `category_handler.rs`
* `auth_handler.rs`

#### 方針

* APIの入口となる処理を書く
* 必要に応じて `db/` を呼び出す
* SQLやDB処理を直接書きすぎないようにする

---

### 4.4 `models/`

#### 役割

* データ構造を表す構造体を定義する

#### 想定ファイル

* `memo.rs`
* `category.rs`
* `user.rs`

#### 内容

* DB取得結果用の struct
* APIリクエスト用の struct
* APIレスポンス用の struct

#### 方針

* データの形をここにまとめる
* handler と DB の間で使うデータ型を整理する

---

### 4.5 `db/`

#### 役割

* DB接続
* SQL実行
* クエリ処理

#### 想定ファイル

* `connection.rs`
* `memo_repository.rs`
* `category_repository.rs`
* `user_repository.rs`

#### 方針

* sqlx を利用したDB処理を記述する
* handler から DB処理を分離する
* 生SQLを扱う責務をこの層に集める

---

### 4.6 `auth/`

#### 役割

* Googleログイン関連処理
* セッション確認
* 認証処理

#### 想定ファイル

* `google.rs`
* `session.rs`

#### 方針

* Google認証とセッション管理を分けて記述する
* 認証に関する共通処理をまとめる

---

### 4.7 `errors/`

#### 役割

* エラー型定義
* APIエラーレスポンスへの変換

#### 想定ファイル

* `api_error.rs`

#### 方針

* エラー処理を共通化する
* handler 側でのエラー処理を簡潔にする

---

## 5. 役割分担の考え方

### `routes`

* どのURLがどの処理に対応するかを決める

### `handlers`

* 受け取ったリクエストを処理する

### `models`

* データの形を定義する

### `db`

* DBへの読み書きを行う

### `auth`

* ログイン、セッション、認証関連を扱う

### `errors`

* エラーを統一的に扱う

---

## 6. 設計上の意図

本構成は、最初から複雑なアーキテクチャを採用するのではなく、
**学習しながら実装しやすい構成** を意図している。

### 利点

* どこに何を書くかがわかりやすい
* メモアプリ規模に対して過剰に複雑でない
* API、DB、認証、エラーを分けて理解できる

### 注意点

* `handlers` と `db` の責務が混ざらないようにする
* `main.rs` に処理を書きすぎないようにする

---

## 7. 補足

* 本構成は現時点の設計方針であり、将来的に機能追加が増えた場合は再編成を検討する
* DB接続設定やDocker構成は `docker_env_design.md` を参照する
* 認証の詳細仕様は `docs/01_auth/auth_design.md` を参照する