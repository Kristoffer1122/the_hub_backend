# The Hub - Backend

Rust (Axum + Diesel) API server for The Hub application.

## Running (Recommended)

Run everything from the monorepo using Docker Compose:

```bash
git clone --recurse-submodules https://github.com/Kristoffer1122/the_hub.git
cd the_hub
docker compose up --build
```

See the [monorepo README](https://github.com/Kristoffer1122/the_hub) for full instructions.

## Prerequisites (local dev only)

- Rust toolchain
- `diesel_cli` (`cargo install diesel_cli --no-default-features --features mysql`)
- A running MariaDB instance

## Local Development

Copy `.env` and configure your database connection:

```bash
cp .env .env.local
# edit DB_HOST, DB_USER, DB_PASSWORD, DB_NAME, DB_PORT
```

Run migrations and start the server:

```bash
diesel migration run
cargo run
```

Server listens on `http://localhost:7878`.

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DB_HOST` | Database host | `localhost` |
| `DB_PORT` | Database port | `3306` |
| `DB_USER` | Database user | - |
| `DB_PASSWORD` | Database password | - |
| `DB_NAME` | Database name | `the_hub` |
| `AZURE_OPENAI_ENDPOINT` | Azure AI Foundry endpoint | - |
| `AZURE_OPENAI_DEPLOYMENT_NAME` | AI agent name | `scheduler` |

### This does not save any of your Personal data
https://www.shera.no/privacy-policy