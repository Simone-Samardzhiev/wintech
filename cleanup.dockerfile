FROM rust:1.94-slim as builder

WORKDIR /app
COPY . .

WORKDIR /app/backend
RUN cargo build --release --bin cleanup

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/cleanup /app/cleanup

CMD ["./cleanup"]

