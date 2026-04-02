# Dockerfile for building musl-linux-mipsle binary
FROM rust:1.75 AS builder

# Install musl-cross toolchain for mipsle
RUN apt-get update && apt-get install -y \
    musl-tools \
    musl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install mipsel-musl cross compiler
RUN apt-get update && apt-get install -y \
    gcc-mipsel-linux-gnu \
    && rm -rf /var/lib/apt/lists/*

# Add mipsel target
RUN rustup target add mipsel-unknown-linux-musl

# Set working directory
WORKDIR /app

# Copy source code
COPY Cargo.toml Cargo.lock* ./
COPY src ./src

# Build for mipsel-unknown-linux-musl
ENV CARGO_TARGET_MIPSEL_UNKNOWN_LINUX_MUSL_LINKER=mipsel-linux-gnu-gcc \
    CC_mipsel_unknown_linux_musl=mipsel-linux-gnu-gcc \
    RUSTFLAGS="-C target-feature=+crt-static"

RUN cargo build --release --target mipsel-unknown-linux-musl --features static

# Final stage - minimal image
FROM scratch

# Copy the binary
COPY --from=builder /app/target/mipsel-unknown-linux-musl/release/sublink-worker /sublink-worker

# Set entrypoint
ENTRYPOINT ["/sublink-worker"]

# Expose default port
EXPOSE 8787
