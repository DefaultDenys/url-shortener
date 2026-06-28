# URL Shortener

Rust URL shortener with Axum + Tokio. Web UI, JSON API, and redirects. In-memory store for now.

## Stack

Axum, Tokio, Serde, tracing, tower-http, dotenvy

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

- Database (SQLite / Postgres)
- Caching
- Click statistics