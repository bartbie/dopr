FROM rust:1.81-slim-bullseye as builder

WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Copy manifests
COPY Cargo.toml .
COPY Cargo.lock .

# Create dummy source for caching dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src src
RUN touch src/main.rs && cargo build --release

# Final stage
FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/dopr /usr/local/bin/

EXPOSE 3000

CMD ["dopr"]
