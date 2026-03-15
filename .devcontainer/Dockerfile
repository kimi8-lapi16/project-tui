FROM rust:latest

# Install development tools
RUN apt-get update && apt-get install -y \
    git \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user first
RUN useradd -m -s /bin/bash rustdev && \
    mkdir -p /workspace && \
    chown -R rustdev:rustdev /workspace

# Switch to non-root user
USER rustdev
WORKDIR /workspace

# Install cargo tools as the non-root user
RUN cargo install cargo-watch && \
    cargo install cargo-edit && \
    cargo install cargo-audit
