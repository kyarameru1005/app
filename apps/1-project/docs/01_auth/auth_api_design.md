# auth_api_design.md

## 1. 認証系API一覧

| No | API名 | メソッド | パス | 概要 |
|---|---|---|---|---|
| 1 | ログイン開始API | `POST` | `/api/auth/google/login` | Googleログイン処理を開始する |
| 2 | ログインコールバックAPI | `GET` | `/api/auth/google/callback` | Google認証結果を受け取り、アプリ内ログイン処理を行う |
| 3 | ログアウトAPI | `POST` | `/api/auth/logout` | ログイン中のセッションを終了する |
| 4 | ログインユーザー取得API | `GET` | `/api/users/me` | ログイン中ユーザー情報を取得する |

---

## 2. API詳細

### 2.1 ログイン開始API

| 項目 | 内容 |
|---|---|
| API名 | ログイン開始 |
| メソッド | `POST` |
| パス | `/api/auth/google/login` |
| 概要 | Googleログイン処理を開始する |

#### リクエスト
なし

#### レスポンス

| 項目名 | 型 | 内容 |
|---|---|---|
| `message` | string | ログイン開始メッセージ |

#### エラー
- `500 Internal Server Error`

---

### 2.2 ログインコールバックAPI

| 項目 | 内容 |
|---|---|
| API名 | ログインコールバック |
| メソッド | `GET` |
| パス | `/api/auth/google/callback` |
| 概要 | Google認証結果を受け取り、アプリ内ログイン処理を行う |

#### リクエスト
- Google認証結果を受け取る

#### レスポンス

| 項目名 | 型 | 内容 |
|---|---|---|
| `message` | string | ログイン成功メッセージ |

#### エラー
- `401 Unauthorized`
- `500 Internal Server Error`

---

### 2.3 ログアウトAPI

| 項目 | 内容 |
|---|---|
| API名 | ログアウト |
| メソッド | `POST` |
| パス | `/api/auth/logout` |
| 概要 | ログイン中のセッションを終了する |

#### リクエスト
なし

#### レスポンス

| 項目名 | 型 | 内容 |
|---|---|---|
| `message` | string | ログアウト完了メッセージ |

#### エラー
- `401 Unauthorized`
- `500 Internal Server Error`

---

### 2.4 ログインユーザー取得API

| 項目 | 内容 |
|---|---|
| API名 | ログインユーザー取得 |
| メソッド | `GET` |
| パス | `/api/users/me` |
| 概要 | ログイン中ユーザー情報を取得する |

#### リクエスト
なし

#### レスポンス

| 項目名 | 型 | 内容 |
|---|---|---|
| `user_id` | string(UUID) | アプリ内ユーザーID |
| `google_sub` | string | Googleユーザー識別子 |
| `user_name` | string | ユーザー名 |
| `email` | string | メールアドレス |

#### エラー
- `401 Unauthorized`
- `500 Internal Server Error`

---

## 3. ステータスコード方針

| ステータスコード | 意味 |
|---|---|
| `401 Unauthorized` | 未ログイン、または認証失敗 |
| `500 Internal Server Error` | サーバ内部エラー |

---

## 4. 補足

### 4.1 初回ログイン時
- Google認証成功後、`users` テーブルに対象ユーザーが存在しない場合は新規登録する

### 4.2 ユーザー識別
- Google 側の識別子として `google_sub` を利用する
- アプリ内のユーザー識別には `user_id` を利用する

### 4.3 セッション管理
- ログイン成功後、セッションに `user_id` を保持する
- API実行時はセッションから `user_id` を取得してログイン中ユーザーを特定する