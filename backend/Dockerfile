FROM rust:latest AS builder

RUN apt update \
&& apt upgrade -y \
&& apt install -y libpq-dev \
&& rm -rf /var/lib/apt/lists/*

WORKDIR /lrar/backend

COPY backend /lrar/backend

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo build

FROM debian:bookworm-slim

RUN apt update \
&& apt upgrade -y \
&& apt install -y libpq5 ca-certificates \
&& rm -rf /var/lib/apt/lists/*

WORKDIR /lrar/backend

COPY --from=builder /lrar/backend/target/debug/backend ./backend_binary

COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

RUN mkdir -p /lrar/backend/migrations

COPY backend/assets /lrar/backend/assets

EXPOSE 3001

CMD ["./backend_binary"]
