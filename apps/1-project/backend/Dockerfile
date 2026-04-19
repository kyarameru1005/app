FROM rust:1.90-bookworm AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/memo-app-backend /usr/local/bin/memo-app-backend

EXPOSE 8080
CMD ["memo-app-backend"]
