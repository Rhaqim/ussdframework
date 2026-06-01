# ── Stage 1: Build ────────────────────────────────────────────────────────────
#
# Build argument lets you choose the database backend at image build time:
#   docker build --build-arg DB_FEATURE=db-sqlite .   (default)
#   docker build --build-arg DB_FEATURE=db-postgres .
#
ARG DB_FEATURE=db-sqlite

FROM rust:1.96-slim AS builder

ARG DB_FEATURE

# System libraries required at compile time
RUN apt-get update && apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
        libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Cache dependency resolution/download separately from application source.
# Copy only the manifest files first so Docker can cache the cargo fetch layer.
COPY Cargo.toml Cargo.lock ./

# Create a dummy lib and main so `cargo build` can resolve the dependency graph
# without copying the full source tree.
RUN mkdir -p src && \
    echo 'fn main() {}' > src/main.rs && \
    echo '' > src/lib.rs && \
    cargo build --release --features "menubuilder,${DB_FEATURE}" --bin main && \
    rm -rf src

# Now copy the real source and rebuild (only application code recompiles)
COPY src ./src
COPY migrations ./migrations
COPY migrations_pg ./migrations_pg

# Touch main.rs so Cargo sees it as changed
RUN touch src/main.rs src/lib.rs && \
    cargo build --release --features "menubuilder,${DB_FEATURE}" --bin main

# ── Stage 2: Runtime ──────────────────────────────────────────────────────────
# Ubuntu 24.04 LTS ships GLIBC 2.39, which satisfies any requirement from
# the rust:*-slim builder images (based on Debian trixie / GLIBC 2.38).
# Using Ubuntu LTS avoids the class of GLIBC version mismatch errors that
# arise when the builder tracks a newer Debian release than the runtime.
FROM ubuntu:24.04

ARG DB_FEATURE

# Runtime libraries (match what was linked at build time)
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
        bash \
        libssl3 \
        ca-certificates \
    && if [ "${DB_FEATURE}" = "db-sqlite" ]; then \
           apt-get install -y --no-install-recommends libsqlite3-0; \
       fi \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /build/target/release/main /app/ussdframework

# ── Data directory for the SQLite database ────────────────────────────────────
# Mount a named volume here to persist the database across container restarts:
#   volumes:
#     - ussd-data:/app/data
RUN mkdir -p /app/data

# ── Default configuration ─────────────────────────────────────────────────────
# All values can be overridden with environment variables in docker-compose.yml
# or with `docker run -e VARIABLE=value`.
ENV USSD_PORT=8080
ENV USSD_DATABASE_URL=/app/data/menu.sqlite3

# Optional: mount a seed file to pre-populate the database on first run
# ENV USSD_JSON_SEED=/app/menu.json

EXPOSE ${USSD_PORT}

ENTRYPOINT ["/app/ussdframework"]
