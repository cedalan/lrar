FROM rust:latest AS builder

WORKDIR /app

COPY backend /app

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/backend /app/backend

EXPOSE 3001

CMD ["./backend"]
