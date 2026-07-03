# URL Shortener

Rust URL shortener with Axum + Tokio. Web UI, JSON API, and redirects. PostgreSQL persistence via SeaORM.

**Live:** [https://url-shortener-rust.up.railway.app/](https://url-shortener-rust.up.railway.app/)

## Stack

Axum, Tokio, SeaORM, PostgreSQL, Serde, tracing, tower-http, dotenvy

## Database

Requires PostgreSQL. Migrations run automatically on startup.

**Local dev — Postgres in Docker, app with Cargo:**

```bash
docker compose up -d
cargo run
```

`docker-compose.yml` only runs the database. App: [http://127.0.0.1:3000](http://127.0.0.1:3000)

On Railway: add a **PostgreSQL** service, link it to the app (sets `DATABASE_URL`), and deploy with the included `Dockerfile`.

## Run

```bash
cp .env.example .env
cargo run
```

Server: [http://127.0.0.1:3000](http://127.0.0.1:3000)

## Routes

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/` | Web UI |
| `GET` | `/health` | Health check |
| `POST` | `/shorten` | Create short URL |
| `GET` | `/{url_short}` | Redirect |

## API

```bash
curl -X POST http://127.0.0.1:3000/shorten \
  -H "Content-Type: application/json" \
  -d "{\"url_original\": \"https://www.rust-lang.org\"}"
```

```bash
curl -L http://127.0.0.1:3000/<url_short>
```

## Logging

Set in `.env`:

```env
RUST_LOG=url_shortener=info,tower_http=info
```

## Todo

- Caching
- Click statistics