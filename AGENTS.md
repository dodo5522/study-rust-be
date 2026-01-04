# リポジトリガイドライン

## 使用言語
- ドキュメントには日本語を使用し、コード内のコメントも日本語で記述。
- コード内の変数名、関数名、構造体名などは英語で記述。

## プロジェクト構成とモジュール構成
- `src/` はアプリのエントリポイント。
- `layers/` はワークスペースのクレート群:
  - `layers/presentation/` HTTP ルーティングとリクエスト/レスポンスモデル。
  - `layers/use-case/` アプリケーション/サービス層。
  - `layers/domain/` ドメインとリポジトリのインターフェース。
  - `layers/infra-db/` SeaORM のエンティティ、リポジトリ、DB 接続補助。
  - `layers/infra-db-migration/` SeaORM のマイグレーション CLI とファイル群。
- `docker/` は開発用コンテナと本番用コンテナ
  - `backend` は本番用バックエンド。
  - `backend-dev` は開発用バックエンド。
- `init` は初期化スクリプト群。

## ビルド・テスト・開発コマンド
- `docker compose up -d` で PostgreSQL を起動し、マイグレーション、APIサーバを起動。
- `cargo run` で API サーバを起動（`BIND_ADDR:BIND_PORT` にバインド）。
- `cargo test` でワークスペース全体のテストを実行（存在する場合）。
- `cargo run -p infra-db-migration -- status` でマイグレーションの状態確認。
- `cargo run -p infra-db-migration -- up` で未適用マイグレーションを適用。

## コーディングスタイルと命名規則
- 標準の Rust フォーマットに従い、`cargo fmt` を実行（`rust-toolchain.toml` に rustfmt あり）。
- 命名は Rust の慣例に従う: 関数/モジュールは `snake_case`、型は `CamelCase`。
- 説明や解説は日本語で記述し、コード上の変数名・関数名・構造体名などは英語で記述する。
- レイヤ構成（presentation/use-case/domain/infra）に沿って責務と境界を保つ。

## テスト指針
- ルートで `cargo test` を実行。
- テストは対象コードの近くに配置（例: `layers/*/src/`）。
- DB 依存テストを追加する場合は、必要な環境変数と手順を記載。

## コミットとプルリクエストの指針
- コミットメッセージはスコープ付きプレフィックス形式（例: `[infra] Add migration files`）。
- 1 コミット 1 変更（または 1 レイヤ）を意識。
- PR には要約、実行したコマンド（`cargo test`、マイグレーション）、DB スキーマ変更の有無を記載。

## 設定と環境
- 想定する環境変数: `BIND_ADDR`, `BIND_PORT`, `DB_HOST`, `DB_PORT`, `DB_NAME`, `DB_OPERATOR_NAME`, `DB_OPERATOR_PASSWORD`。
- 秘密情報はローカルの `.env` であり、リポジトリにコミット済みの `.env` は開発用のサンプル。
