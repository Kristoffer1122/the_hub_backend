# Next.js Frontend

This backend serves the API for The Hub frontend application.

## Repository

- **Repo:** [Kristoffer1122/the_hub](https://github.com/Kristoffer1122/the_hub)
- **Language:** TypeScript (Next.js + Tailwind CSS)
- **Default Port:** `3000`

## Connection

The frontend connects to this backend via the `BACKEND_URL` environment variable:

```bash
# Default (local development)
BACKEND_URL=http://localhost:7878

# Docker (container-to-host)
BACKEND_URL=http://host.docker.internal:7878
```

## Setup

1. Clone the frontend:
   ```bash
   git clone https://github.com/Kristoffer1122/the_hub.git
   ```
2. Install dependencies:
   ```bash
   cd the_hub
   npm install
   ```
3. Start this backend on port `7878`
4. Start the frontend with `npm run dev`
5. Open [http://localhost:3000](http://localhost:3000) in your browser
