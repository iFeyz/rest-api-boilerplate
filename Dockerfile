# Postgres stage for build
FROM postgres:latest AS postgres_builder
ENV POSTGRES_USER=postgres
ENV POSTGRES_PASSWORD=postgres
ENV POSTGRES_DB=rust_api_db

# Builder stage
FROM rust:1.84 as builder

WORKDIR /usr/src/app

# Install sqlx-cli
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres

# First copy migrations and cargo files
COPY migrations ./migrations
COPY scripts ./scripts
COPY Cargo.toml Cargo.lock ./
COPY .sqlx ./.sqlx
COPY GeoIP2-City.mmdb ./GeoIP2-City.mmdb

# Create dummy src layout
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs

# Build dependencies only
RUN cargo build --release
RUN rm -rf src

# Now copy the real source code
COPY . .

# Enable offline mode and build
ENV SQLX_OFFLINE=true
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    postgresql-client \
    curl \
    netcat-openbsd \
    net-tools \
    && rm -rf /var/lib/apt/lists/*

# Install sqlx-cli in the runtime image
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

WORKDIR /usr/local/bin

# Copy the built binary and scripts
COPY --from=builder /usr/src/app/target/release/api-boilerplate .
COPY --from=builder /usr/src/app/scripts/wait-for-db.sh .
COPY --from=builder /usr/src/app/migrations ./migrations
COPY --from=builder /usr/src/app/GeoIP2-City.mmdb ./GeoIP2-City.mmdb

RUN chmod +x wait-for-db.sh

EXPOSE 8080

# Use wait-for-db script before starting the application
CMD ["sh", "-c", "./wait-for-db.sh && ./api-boilerplate"] 