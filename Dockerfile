# Используем свежий официальный образ Rust на базе Alpine
FROM rust:alpine AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Финальный легковесный образ
FROM alpine:3.19
WORKDIR /app
COPY --from=builder /app/target/release/belarusian /app/belarusian

EXPOSE 8080
ENV PORT=8080
CMD ["./belarusian"]