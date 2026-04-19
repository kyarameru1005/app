# memo_api_design.md

## 1. メモ系API一覧

| No | API名 | メソッド | パス | 概要 |
|---|---|---|---|---|
| 1 | メモ一覧取得API | `GET` | `/api/memos` | メモ一覧を取得する |
| 2 | メモ詳細取得API | `GET` | `/api/memos/{memo_id}` | 指定したメモの詳細情報を取得する |
| 3 | メモ新規作成API | `POST` | `/api/memos` | 新しいメモを登録する |
| 4 | メモ更新API | `PUT` | `/api/memos/{memo_id}` | 指定したメモを更新する |
| 5 | メモ削除API | `DELETE` | `/api/memos/{memo_id}` | 指定したメモを削除する |

---

## 2. API詳細

### 2.1 メモ一覧取得API

| 項目 | 内容 |
|---|---|
| API名 | メモ一覧取得 |
| メソッド | `GET` |
| パス | `/api/memos` |
| 概要 | メモ一覧を取得する |

#### リクエストパラメータ

| 項目名 | 型 | 必須 | 内容 |
|---|---|---|---|
| `keyword` | string | 任意 | タイトル・本文検索キーワード |
| `category_id` | string(UUID) | 任意 | カテゴリID |

#### レスポンス

| 項目名 | 型 | 内容 |
|---|---|---|
| `memos` | array | メモ一覧 |

#### memos 配下

| 項目名 | 型 | 内容 |
|---|---|---|
| `memo_id` | string(UUID) | メモID |
| `title` | string | タイトル |
| `content` | string | 本文 |
| `category_id` | string(UUID) | カテゴリID |
| `category_name` | string | カテゴリ名 |
| `created_at` | string(datetime) | 作成日時 |
| `updated_at` | string(datetime) | 更新日時 |

#### 仕様
- 検索対象は `title` および `content` とする
- 並び順は `created_at` の降順とする
- 新しいメモから順に表示する
- ページングは実装しない
- 検索結果が0件の場合は空配列を返す

#### エラー
- `400 Bad Request`
- `401 Unauthorized`
- `500 Internal Server Error`

---

### 2.2 メモ詳細取得API

| 項目 | 内容 |
|---|---|
| API名 | メモ詳細取得 |
| メソッド | `GET` |
| パス | `/api/memos/{memo_id}` |
| 概要 | 指定したメモの詳細情報を取得する |

#### パスパラメータ

| 項目名 | 型 | 必須 | 内容 |
|---|---|---|---|
| `memo_id` | string(UUID) | 必須 | メモID |

#### レスポンス

| 項目名 | 型 | 内容 |
|---|---|---|
| `memo_id` | string(UUID) | メモID |
| `title` | string | タイトル |
| `content` | string | 本文 |
| `category_id` | string(UUID) | カテゴリID |
| `category_name` | string | カテゴリ名 |
| `created_at` | string(datetime) | 作成日時 |
| `updated_at` | string(datetime) | 更新日時 |

#### エラー
- `401 Unauthorized`
- `404 Not Found`
- `500 Internal Server Error`

---

### 2.3 メモ新規作成API

| 項目 | 内容 |
|---|---|
| API名 | メモ新規作成 |
| メソッド | `POST` |
| パス | `/api/memos` |
| 概要 | 新しいメモを登録する |

#### リクエストボディ

| 項目名 | 型 | 必須 | 内容 |
|---|---|---|---|
| `title` | string | 必須 | メモタイトル |
| `content` | string | 任意 | メモ本文 |
| `category_id` | string(UUID) | 任意 | カテゴリID |

#### 仕様
- `title` は必須とする
- `content` は任意とし、空文字を許可する
- `category_id` は任意とする
- `category_id` が指定されている場合のみ、UUID形式チェックおよび存在チェックを行う
- `user_id` はリクエストで受け取らず、セッションから取得する

#### レスポンス

| 項目名 | 型 | 内容 |
|---|---|---|
| `memo_id` | string(UUID) | 作成したメモID |
| `message` | string | 完了メッセージ |

#### エラー
- `400 Bad Request`
- `401 Unauthorized`
- `404 Not Found`
- `500 Internal Server Error`

---

### 2.4 メモ更新API

| 項目 | 内容 |
|---|---|
| API名 | メモ更新 |
| メソッド | `PUT` |
| パス | `/api/memos/{memo_id}` |
| 概要 | 指定したメモを更新する |

#### パスパラメータ

| 項目名 | 型 | 必須 | 内容 |
|---|---|---|---|
| `memo_id` | string(UUID) | 必須 | メモID |

#### リクエストボディ

| 項目名 | 型 | 必須 | 内容 |
|---|---|---|---|
| `title` | string | 必須 | メモタイトル |
| `content` | string | 任意 | メモ本文 |
| `category_id` | string(UUID) | 任意 | カテゴリID |

#### 仕様
- `title` は必須とする
- `content` は任意とし、空文字を許可する
- `category_id` は任意とする
- `category_id` が指定されている場合のみ、UUID形式チェックおよび存在チェックを行う
- `user_id` はリクエストで受け取らず、セッションから取得する
- 更新対象のメモが存在しない場合は `404 Not Found` とする

#### レスポンス

| 項目名 | 型 | 内容 |
|---|---|---|
| `memo_id` | string(UUID) | 更新したメモID |
| `message` | string | 完了メッセージ |

#### エラー
- `400 Bad Request`
- `401 Unauthorized`
- `404 Not Found`
- `500 Internal Server Error`

---

### 2.5 メモ削除API

| 項目 | 内容 |
|---|---|
| API名 | メモ削除 |
| メソッド | `DELETE` |
| パス | `/api/memos/{memo_id}` |
| 概要 | 指定したメモを削除する |

#### パスパラメータ

| 項目名 | 型 | 必須 | 内容 |
|---|---|---|---|
| `memo_id` | string(UUID) | 必須 | メモID |

#### 仕様
- 対象メモを指定して削除する
- `user_id` はリクエストで受け取らず、セッションから取得する
- ログイン中ユーザー本人のメモのみ削除可能とする

#### レスポンス

| 項目名 | 型 | 内容 |
|---|---|---|
| `memo_id` | string(UUID) | 削除したメモID |
| `message` | string | 完了メッセージ |

#### エラー
- `401 Unauthorized`
- `403 Forbidden`
- `404 Not Found`
- `500 Internal Server Error`

---

## 3. バリデーション方針

### 3.1 title
- 必須
- 最大200文字
- 空文字不可

### 3.2 content
- 任意
- 空文字可

### 3.3 category_id
- 任意
- 指定される場合のみ UUID形式チェックを行う
- 指定される場合のみ存在チェックを行う

---

## 4. エラーメッセージ例

| 種別 | メッセージ |
|---|---|
| タイトル未入力 | タイトルを入力してください |
| タイトル文字数超過 | タイトルは200文字以内で入力してください |
| 対象メモなし | 対象のメモが存在しません |
| 一覧取得失敗 | メモ一覧の取得に失敗しました |
| 詳細取得失敗 | メモの取得に失敗しました |
| 登録失敗 | メモの登録に失敗しました |
| 更新失敗 | メモの更新に失敗しました |
| 削除失敗 | メモの削除に失敗しました |
| 権限なし | この操作を行う権限がありません |