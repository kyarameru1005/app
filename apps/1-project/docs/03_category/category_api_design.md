# category_api_design.md

## 1. カテゴリ系API一覧

| No | API名 | メソッド | パス | 概要 |
|---|---|---|---|---|
| 1 | カテゴリ一覧取得API | `GET` | `/api/categories` | カテゴリ一覧を取得する |
| 2 | カテゴリ新規作成API | `POST` | `/api/categories` | 新しいカテゴリを登録する |
| 3 | カテゴリ更新API | `PUT` | `/api/categories/{category_id}` | 指定したカテゴリを更新する |

---

## 2. API詳細

### 2.1 カテゴリ一覧取得API

| 項目 | 内容 |
|---|---|
| API名 | カテゴリ一覧取得 |
| メソッド | `GET` |
| パス | `/api/categories` |
| 概要 | カテゴリ一覧を取得する |

#### リクエスト
なし

#### レスポンス

| 項目名 | 型 | 内容 |
|---|---|---|
| `categories` | array | カテゴリ一覧 |

#### categories 配下

| 項目名 | 型 | 内容 |
|---|---|---|
| `category_id` | string(UUID) | カテゴリID |
| `category_name` | string | カテゴリ名 |

#### 仕様
- カテゴリは全ユーザー共通とする
- カテゴリが0件の場合は空配列を返す

#### エラー
- `401 Unauthorized`
- `500 Internal Server Error`

---

### 2.2 カテゴリ新規作成API

| 項目 | 内容 |
|---|---|
| API名 | カテゴリ新規作成 |
| メソッド | `POST` |
| パス | `/api/categories` |
| 概要 | 新しいカテゴリを登録する |

#### リクエストボディ

| 項目名 | 型 | 必須 | 内容 |
|---|---|---|---|
| `category_name` | string | 必須 | カテゴリ名 |

#### 仕様
- `category_name` は必須とする
- `category_name` は最大100文字とする
- `category_name` は空文字不可とする
- `category_name` は重複不可とする

#### レスポンス

| 項目名 | 型 | 内容 |
|---|---|---|
| `category_id` | string(UUID) | 作成したカテゴリID |
| `message` | string | 完了メッセージ |

#### エラー
- `400 Bad Request`
- `401 Unauthorized`
- `409 Conflict`
- `500 Internal Server Error`

---

### 2.3 カテゴリ更新API

| 項目 | 内容 |
|---|---|
| API名 | カテゴリ更新 |
| メソッド | `PUT` |
| パス | `/api/categories/{category_id}` |
| 概要 | 指定したカテゴリを更新する |

#### パスパラメータ

| 項目名 | 型 | 必須 | 内容 |
|---|---|---|---|
| `category_id` | string(UUID) | 必須 | カテゴリID |

#### リクエストボディ

| 項目名 | 型 | 必須 | 内容 |
|---|---|---|---|
| `category_name` | string | 必須 | 更新後のカテゴリ名 |

#### 仕様
- `category_name` は必須とする
- `category_name` は最大100文字とする
- `category_name` は空文字不可とする
- `category_name` は重複不可とする
- 更新対象のカテゴリが存在しない場合は `404 Not Found` とする

#### レスポンス

| 項目名 | 型 | 内容 |
|---|---|---|
| `category_id` | string(UUID) | 更新したカテゴリID |
| `message` | string | 完了メッセージ |

#### エラー
- `400 Bad Request`
- `401 Unauthorized`
- `404 Not Found`
- `409 Conflict`
- `500 Internal Server Error`

---

## 3. バリデーション方針

### 3.1 category_name
- 必須
- 最大100文字
- 空文字不可
- 重複不可

---

## 4. エラーメッセージ例

| 種別 | メッセージ |
|---|---|
| カテゴリ名未入力 | カテゴリ名を入力してください |
| カテゴリ名文字数超過 | カテゴリ名は100文字以内で入力してください |
| カテゴリ名重複 | 同じカテゴリ名が既に存在します |
| 対象カテゴリなし | 対象のカテゴリが存在しません |
| 一覧取得失敗 | カテゴリ一覧の取得に失敗しました |
| 登録失敗 | カテゴリの登録に失敗しました |
| 更新失敗 | カテゴリの更新に失敗しました |