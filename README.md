# Poem + SeaORM sample (local with Docker Compose)

このプロジェクトは Poem (web framework) と SeaORM (ORM) を使ったプロジェクトです。
PostgreSQL は Docker Compose で立ち上げ、起動時に [docker/db/*.*](docker/db) の内容でDBを初期化します。

# 動作確認手順

1. Docker Compose で PostgreSQL , バックエンドサーバを起動
   - docker-compose.yml があるディレクトリで:
     ```
     docker compose up -d
     ```
   - 起動と初期化が終わるまで少し待ちます（healthcheck で確認できます）。
   - バックエンドサーバは http://127.0.0.1:8000 で待ち受けます。

2. API サンプル
   - healthcheck:
     GET http://127.0.0.1:3000/health

# 実装ノート & 次の拡張案（今後やること）

- 認証（JWT / OAuth）、パスワード保存（argon2）を追加する
- OpenAPI ドキュメント（poem-openapi）を追加して FastAPI のような自動ドキュメントを出す
