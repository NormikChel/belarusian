# ==========================================
# ЭТАП 1: Зборка бінарніка (генэратара)
# ==========================================
FROM rust:1.79-slim AS builder

# Для musl не трэба, але для некаторых крейтаў — pkg-config
RUN apt-get update && apt-get install -y \
    pkg-config \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Капіруем усе зыходнікі
COPY Cargo.toml ./
COPY src ./src
COPY templates ./templates
COPY static ./static

# Кампілюем у release
RUN cargo build --release

# Запускаем генэратар — ён створыць dist/
RUN ./target/release/belarusian

# ==========================================
# ЭТАП 2: Раздача статыкі праз Static Web Server
# ==========================================
FROM joseluisq/static-web-server:2

COPY --from=builder /app/dist /public

ENV SERVER_ROOT=/public
ENV SERVER_PORT=8080
ENV SERVER_LOG_LEVEL=info

EXPOSE 8080

CMD ["static-web-server"]