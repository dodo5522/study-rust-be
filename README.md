# Poem + SeaORM sample (local with Docker Compose)

このプロジェクトは Poem (web framework) と SeaORM (ORM) を使ったプロジェクトです。
PostgreSQL は Docker Compose で立ち上げ、起動時に [docker/db/*.*](docker/db) の内容でDBを初期化します。

# 動作確認手順

1. Docker Compose で PostgreSQL を起動
   - docker-compose.yml があるディレクトリで:
     ```
     docker compose up -d
     ```
   - 起動と初期化が終わるまで少し待ちます（healthcheck で確認できます）。

2. .env を確認
   - デフォルトでは .env に以下が入っています:
     DATABASE_URL=postgres://app:pass@127.0.0.1:5432/app_db
   - 必要なら変更してください（ただし docker-compose.yml の設定と一致させてください）。

3. サーバ起動
   - Rust がインストールされている環境で:
     ```
     cargo run
     ```
   - サーバは http://127.0.0.1:3000 で待ち受けます。

4. API サンプル
   - 健康チェック:
     GET http://127.0.0.1:3000/health
   - ユーザー一覧取得:
     GET http://127.0.0.1:3000/users
   - ユーザー作成:
     POST http://127.0.0.1:3000/users
     Content-Type: application/json
     Body:
     {
       "name": "Alice",
       "email": "alice@example.com"
     }

# 実装ノート & 次の拡張案（今後やれること）

- 認証（JWT / OAuth）、パスワード保存（argon2）を追加する
- OpenAPI ドキュメント（poem-openapi）を追加して FastAPI のような自動ドキュメントを出す
