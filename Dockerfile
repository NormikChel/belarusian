FROM rust:alpine AS builder
WORKDIR /app
RUN apk add --no-cache musl-dev
COPY . .
RUN cargo build --release

FROM alpine:3.19
WORKDIR /app

# Капіюем бінарнік
COPY --from=builder /app/target/release/belarusian /app/belarusian

# І статыку — абавязкова!
COPY --from=builder /app/static /app/static

EXPOSE 8080
ENV PORT=8080
CMD ["./belarusian"]