# Multi-stage build for minimal image size
FROM rust:latest AS builder

# Create app directory
WORKDIR /usr/src/app

# Copy Cargo files for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create dummy files for dependency compilation
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "fn main() {}" > src/main_simple.rs && \
    echo "fn main() {}" > src/main_http.rs && \
    echo "fn main() {}" > src/test_api.rs

# Build dependencies only
RUN cargo build --release --bin splitwise-mcp

# Remove dummy files
RUN rm -rf src

# Copy actual source code
COPY src ./src

# Force rebuild of both binaries
RUN touch src/main_simple.rs src/main_http.rs && \
    cargo build --release --bin splitwise-mcp && \
    cargo build --release --bin splitwise-mcp-http

# Runtime stage - using debian slim for better compatibility
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 -s /bin/bash mcp

# Copy binaries from builder
COPY --from=builder /usr/src/app/target/release/splitwise-mcp /usr/local/bin/splitwise-mcp
COPY --from=builder /usr/src/app/target/release/splitwise-mcp-http /usr/local/bin/splitwise-mcp-http

# Set ownership
RUN chown mcp:mcp /usr/local/bin/splitwise-mcp /usr/local/bin/splitwise-mcp-http

# Switch to non-root user
USER mcp

# Set working directory
WORKDIR /home/mcp

# Expose HTTP/SSE port
EXPOSE 8080

# Run the HTTP/SSE server by default
CMD ["splitwise-mcp-http"]