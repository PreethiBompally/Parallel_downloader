# Start with Rust base image
FROM rust:1-slim-bookworm AS builder

# Install SSL/TLS dependencies and other build essentials
RUN apt-get update && \
    apt-get install -y \
        pkg-config \
        libssl-dev \
        build-essential \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty project
WORKDIR /usr/src/app

# Copy Cargo files first
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs

# Build dependencies - this will be cached if dependencies don't change
RUN cargo build --release

# Remove the dummy source code
RUN rm -rf src

# Copy your actual source code
COPY src ./src

# Build the actual application
RUN cargo build --release

# Create the runtime image
FROM debian:bookworm-slim

# Install SSL certificates and required runtime libraries
RUN apt-get update && \
    apt-get install -y \
        ca-certificates \
        openssl \
        libc6 \
    && rm -rf /var/lib/apt/lists/*

# Create a directory for downloads
WORKDIR /downloads

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/parallel-downloader /usr/local/bin/

# Configure container for proper stdio handling
ENV RUST_BACKTRACE=1
ENV PYTHONUNBUFFERED=1
STOPSIGNAL SIGINT

# Set the entrypoint
ENTRYPOINT ["parallel-downloader"]