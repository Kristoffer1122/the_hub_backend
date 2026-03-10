# ---- Stage 1: Build ----
FROM rust:1.86-slim-bullseye AS builder

WORKDIR /app

# Install MySQL client libraries for Diesel
RUN apt-get update && apt-get install -y --no-install-recommends \
    default-libmysqlclient-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests first (caches dependencies)
COPY Cargo.toml Cargo.lock ./

# Create a dummy src to build dependencies first (layer caching)
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Now copy your actual source code
COPY src/ src/

# Force cargo to detect changes
RUN touch src/main.rs
RUN cargo build --release

# Stage 2 Run
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    default-mysql-client \
    libmariadb3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder
COPY --from=builder /app/target/release/rust_backend .

EXPOSE 7878

# Just run the compiled binary
CMD ["./rust_backend"]
