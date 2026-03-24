# Next.js Frontend

This backend serves the API for The Hub frontend application.

## Repository

- **Repo:** [Kristoffer1122/the_hub_frontend](https://github.com/Kristoffer1122/the_hub_frontend)
- **Language:** TypeScript (Next.js + Tailwind CSS)
- **Default Port:** `3000`

## Running Together

The easiest way to run both services is from the monorepo root:

```bash
git clone --recurse-submodules https://github.com/Kristoffer1122/the_hub.git
cd the_hub
docker compose up --build
```

See the [monorepo README](https://github.com/Kristoffer1122/the_hub) for full instructions.

## Connection

The frontend connects to this backend via the `BACKEND_URL` environment variable:

```bash
# Default (local development)
BACKEND_URL=http://localhost:7878

# Docker Compose (service-to-service)
BACKEND_URL=http://backend:7878
```