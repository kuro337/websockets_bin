# Stage 1: Building
FROM rust:latest as builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

# Stage 2: Runtime
FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/kalz_websocket_server .

EXPOSE 9001

CMD ["./kalz_websocket_server"]
