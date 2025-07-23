# Build stage
FROM rust:1.88-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY shared/ ./shared/
COPY services/enterprise-service/ ./services/enterprise-service/

# Build the application
RUN cargo build --release --bin enterprise-service

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false appuser

# Copy binary from builder stage
COPY --from=builder /app/target/release/enterprise-service /usr/local/bin/enterprise-service

# Change ownership
RUN chown appuser:appuser /usr/local/bin/enterprise-service

# Switch to app user
USER appuser

# Set environment variables
ENV RUST_LOG=info

# Expose port
EXPOSE 8007

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8007/health || exit 1

# Run the application
CMD ["enterprise-service"] 