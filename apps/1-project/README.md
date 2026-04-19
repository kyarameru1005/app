# 1-project (Memo Web App)

Google アカウントでログインし、メモの作成・閲覧・編集・削除を行う Web アプリの実装プロジェクト。

## Tech Stack

- frontend: Next.js (TypeScript)
- backend: Rust + axum + sqlx
- database: PostgreSQL
- environment: Docker Compose

## Setup

1. `apps/1-project` へ移動する
2. 以下のコマンドで開発環境を起動する

```bash
docker compose up --build
```

## Ports

- frontend: `http://localhost:3000`
- backend: `http://localhost:8080`
- db: `localhost:5432`

## Useful Commands

```bash
# 起動
docker compose up --build

# 停止
docker compose down

# ボリュームも削除して停止
docker compose down -v

# Compose 構成確認
docker compose config
```

## Notes

- 設計書は `docs/` 配下を正本とする。
- 実装進捗は `docs/task.md` で管理する。

## CI / Toolchain Policy

- backend の Rust ツールチェーンは `backend/rust-toolchain.toml` で固定する。
- 依存解決の再現性確保のため `backend/Cargo.lock` をコミット対象にする。
- CI と Docker ビルドは lockfile を前提に同一依存を使う（`Cargo.lock` を無視しない）。
