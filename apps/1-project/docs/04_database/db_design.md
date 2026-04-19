# db_design.md

## 1. DB概要

本システムでは、データベースに **PostgreSQL** を利用する。  
保存対象は次の通りとする。

- ユーザーデータ
- メモデータ
- カテゴリデータ

IDはすべて **UUID** を利用する。

---

## 2. テーブル一覧

| テーブル名 | 内容 |
|---|---|
| `users` | ユーザー情報を管理する |
| `notes` | メモ情報を管理する |
| `categories` | カテゴリ情報を管理する |

---

## 3. テーブル関連

- `notes.user_id` → `users.user_id`
- `notes.category_id` → `categories.category_id`

### 関係
- 1ユーザーは複数のメモを持つ
- 1カテゴリは複数のメモに設定できる
- カテゴリは全ユーザー共通で利用する

---

## 4. 主キー / 外部キー

### 4.1 主キー
- `users.user_id`
- `notes.memo_id`
- `categories.category_id`

### 4.2 外部キー
- `notes.user_id` → `users.user_id`
- `notes.category_id` → `categories.category_id`

---

## 5. テーブル定義

### 5.1 users

| カラム名 | 型 | 必須 | キー / 制約 | 内容 |
|---|---|---|---|---|
| `user_id` | `UUID` | 必須 | 主キー | アプリ内ユーザー識別子 |
| `google_sub` | `VARCHAR(255)` | 必須 | `UNIQUE` | Googleユーザー識別子 |
| `user_name` | `VARCHAR(100)` | 必須 |  | ユーザー名 |
| `email` | `VARCHAR(255)` | 必須 | `UNIQUE` | メールアドレス |
| `created_at` | `TIMESTAMP` | 必須 |  | 作成日時 |
| `updated_at` | `TIMESTAMP` | 必須 |  | 更新日時 |

---

### 5.2 categories

| カラム名 | 型 | 必須 | キー / 制約 | 内容 |
|---|---|---|---|---|
| `category_id` | `UUID` | 必須 | 主キー | カテゴリ識別子 |
| `category_name` | `VARCHAR(100)` | 必須 | `UNIQUE` | カテゴリ名 |

---

### 5.3 notes

| カラム名 | 型 | 必須 | キー / 制約 | 内容 |
|---|---|---|---|---|
| `memo_id` | `UUID` | 必須 | 主キー | メモ識別子 |
| `title` | `VARCHAR(200)` | 必須 |  | タイトル |
| `content` | `TEXT` | 任意 |  | 本文 |
| `user_id` | `UUID` | 必須 | 外部キー | 作成ユーザーID |
| `category_id` | `UUID` | 任意 | 外部キー | カテゴリID |
| `created_at` | `TIMESTAMP` | 必須 |  | 作成日時 |
| `updated_at` | `TIMESTAMP` | 必須 |  | 更新日時 |

---

## 6. 制約

### 6.1 users
- `user_id` は主キー
- `google_sub` は必須、一意
- `user_name` は必須、最大100文字
- `email` は必須、一意、最大255文字
- `created_at` は必須
- `updated_at` は必須

### 6.2 categories
- `category_id` は主キー
- `category_name` は必須、一意、最大100文字

### 6.3 notes
- `memo_id` は主キー
- `title` は必須、最大200文字
- `content` は任意、空文字可
- `user_id` は必須
- `category_id` は任意
- `created_at` は必須
- `updated_at` は必須

---

## 7. バリデーション方針

### 7.1 users

| 項目名 | ルール |
|---|---|
| `user_name` | 必須、最大100文字、空文字不可 |
| `email` | 必須、最大255文字、メールアドレス形式、重複不可 |
| `google_sub` | 必須、空文字不可、重複不可 |

---

### 7.2 categories

| 項目名 | ルール |
|---|---|
| `category_name` | 必須、最大100文字、空文字不可、重複不可 |

---

### 7.3 notes

| 項目名 | ルール |
|---|---|
| `title` | 必須、最大200文字、空文字不可 |
| `content` | 任意、空文字可 |
| `category_id` | 任意、値がある場合のみUUID形式チェック、存在するカテゴリであること |

---

## 8. 設計上の補足

### 8.1 認証との関係
- Googleログイン時の識別には `google_sub` を利用する
- アプリ内では `user_id` を利用してメモと関連付ける

### 8.2 カテゴリの扱い
- カテゴリはユーザー個別ではなく、全ユーザー共通とする
- メモにはカテゴリを設定しなくてもよい

### 8.3 メモ本文の扱い
- `content` は空文字を許可する
- タイトルのみのメモも登録可能とする