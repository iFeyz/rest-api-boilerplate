# Use a multi-stage build approach with explicit memory limits
FROM --platform=linux/amd64 rust:1.85 as builder

WORKDIR /usr/src/app
ENV DATABASE_URL=postgresql://postgres:postgres@localhost:5432/postgres

# Copy only the files needed for dependency resolution first
COPY Cargo.toml ./
COPY Cargo.lock* ./
# Create a dummy main.rs to build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > src/lib.rs

# Install system dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Build with reduced optimization and disable zstd feature
RUN RUSTFLAGS="-C opt-level=0" \
    CARGO_FEATURE_DISABLE_ZSTD=1 \
    cargo build --release --no-default-features

# Now copy the actual source code
COPY . .

# Touch the main files to ensure they get rebuilt
RUN find src -type f -name "*.rs" -exec touch {} \;

# Build the application with reduced optimization level and disable zstd
RUN RUSTFLAGS="-C opt-level=0" \
    CARGO_FEATURE_DISABLE_ZSTD=1 \
    cargo build --release --no-default-features

# Runtime stage
FROM --platform=linux/amd64 debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/api-boilerplate /usr/local/bin/

# Copy the GeoIP database
COPY --from=builder /usr/src/app/GeoIP2-City.mmdb /usr/local/bin/

# Copy migrations folder
COPY --from=builder /usr/src/app/migrations /usr/local/bin/migrations

# Set the working directory
WORKDIR /usr/local/bin

# Expose the port
EXPOSE 8080

# Run the binary
CMD ["api-boilerplate"]