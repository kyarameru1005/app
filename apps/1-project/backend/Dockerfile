FROM rust:1.79-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/memo-app-backend /usr/local/bin/memo-app-backend

EXPOSE 8080
CMD ["memo-app-backend"]
