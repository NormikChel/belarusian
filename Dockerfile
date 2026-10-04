# Сборка
FROM rust:1.75-alpine AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Финал
FROM alpine:3.19
WORKDIR /app
# Копируем скомпилированный бинарник
COPY --from=builder /app/target/release/belarusian /app/belarusian
# Копируем статику, если она нужна отдельно (или если она встроена, но по коду у тебя ServeDir на "static")
COPY --from=builder /app/static /app/static

EXPOSE 8080
ENV PORT=8080
CMD ["./belarusian"]