FROM rust:1.94-slim as frontend-builder

RUN apt-get update && apt-get install -y \
    curl pkg-config libssl-dev g++ binaryen

RUN ARCH=$(uname -m) && \
    if [ "$ARCH" = "aarch64" ]; then \
        curl -L https://github.com/trunk-rs/trunk/releases/download/v0.19.1/trunk-aarch64-unknown-linux-gnu.tar.gz | tar -xzf- -C /usr/local/bin; \
    else \
        curl -L https://github.com/trunk-rs/trunk/releases/download/v0.19.1/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf- -C /usr/local/bin; \
    fi

RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .

WORKDIR /app/frontend
RUN trunk build --release

FROM rust:1.94-slim as backend-builder
WORKDIR /app

COPY . .

WORKDIR /app/backend
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=backend-builder /app/target/release/api /app/api

COPY --from=frontend-builder /app/frontend/dist /app/dist

EXPOSE 8080
CMD ["./api"]