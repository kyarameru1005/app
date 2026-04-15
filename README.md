# work

継続的に Web アプリを作り、実装力を高めながら友人との情報交換にも活用するためのリポジトリです。

## 目的

- さまざまな Web アプリを継続的に作る
- 実装、設計、検証の基礎体力を高める
- 作成した内容を共有し、フィードバックを得る

## 現在の状態

現時点ではプロジェクトの初期構成と運用ドキュメントを管理しています。

## ファイル構造

```text
.
|-- AGENTS.md
|-- README.md
|-- .gitignore
|-- docs/
|   `-- task-list.md
|-- scripts/
|   `-- install.py
`-- work/
    |-- task.md
    `-- work.md
```

`tests/` は今後テストコードを配置する想定のディレクトリです。

## 主要ファイル

- `README.md`: リポジトリの目的、構成、主要コマンド
- `AGENTS.md`: このリポジトリで作業する際の運用ルール
- `.gitignore`: Git 管理対象から除外するファイルやディレクトリを定義
- `docs/`: 生成してきたドキュメントを配置するスペース
- `scripts/`: 起動スクリプトや運用スクリプトをまとめて配置するスペース
- `tests/`: テストコードを配置する想定のスペース
- `work/work.md`: 作業メモと依頼内容の整理
- `work/task.md`: 作業メモ用の補助ファイル
- `docs/task-list.md`: このリポジトリの作業台帳
- `scripts/install.py`: `AGENTS.md` を `~/.codex/AGENTS.md` に反映するスクリプト

## 主要コマンド

```bash
python3 scripts/install.py update
```

`AGENTS.md` を `~/.codex/AGENTS.md` に反映し、既存ファイルがあれば `AGENTS.md.bak.<timestamp>` としてバックアップします。

## 運用メモ

- 新しい作業を始める前に `docs/task-list.md` を更新する
- ドキュメント運用ルールの詳細は `AGENTS.md` を参照する
