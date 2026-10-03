# Этап сборки
FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Финальный легкий образ
FROM debian:bookworm-slim
WORKDIR /app

# Копируем скомпилированный бинарник (поменяй имя на название своего проекта из Cargo.toml)
COPY --from=builder /app/target/release/nazva_tvajго_praekta /app/app

EXPOSE 8080
CMD ["./app"]