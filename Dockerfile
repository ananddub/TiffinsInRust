# Builder stage: Use an official Rust image
FROM rust:bookworm AS builder

# Set the working directory
WORKDIR /app

# Copy only the files necessary for building dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

# Copy the source code
COPY . .

# Build the release version
RUN cargo build --release

# Runner stage: Use a minimal base image for running the binary
FROM debian:bookworm-slim AS runner

# Install any required runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/backend /app/backend

# Expose a port (if your app uses a specific one, e.g., 8080)
EXPOSE 8000

# Set the command to run the binary
CMD ["/app/backend"]