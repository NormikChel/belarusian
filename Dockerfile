# ==========================================
# ЭТАП 1: Планировщик (Chef)
# ==========================================
FROM rust:1.79-slim AS chef
# Устанавливаем cargo-chef для кэширования зависимостей
RUN cargo install cargo-chef
WORKDIR /app

# ==========================================
# ЭТАП 2: Анализ зависимостей
# ==========================================
FROM chef AS planner
COPY . .
# Генерируем "рецепт" (список зависимостей) на основе Cargo.toml и Cargo.lock
RUN cargo chef prepare --recipe-path recipe.json

# ==========================================
# ЭТАП 3: Сборка (Builder)
# ==========================================
FROM chef AS builder
# Копируем только рецепт, чтобы закэшировать слой с зависимостями
COPY --from=planner /app/recipe.json recipe.json

# Собираем зависимости. Этот слой будет кэшироваться, если Cargo.toml не менялся.
# Если ты используешь OpenSSL (например, для HTTPS-запросов), раскомментируй установку libssl-dev
# RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo chef cook --release --recipe-path recipe.json

# Копируем исходники и собираем сам проект
COPY . .
# Важно: указываем имя бинарника из твоего Cargo.toml (name = "belarusian")
RUN cargo build --release --bin belarusian

# ==========================================
# ЭТАП 4: Финальный рантайм (Runtime)
# ==========================================
FROM debian:bookworm-slim AS runtime

# Устанавливаем сертификаты (нужны для HTTPS-запросов изнутри приложения)
# и tzdata (часовые пояса)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    tzdata \
    && rm -rf /var/lib/apt/lists/*

# Создаем непривилегированного пользователя для безопасности (не запускаем от root)
RUN useradd -m -u 10001 appuser

WORKDIR /app

# Копируем скомпилированный бинарник из этапа сборки
# Обрати внимание: имя бинарника "belarusian" строго из Cargo.toml
COPY --from=builder /app/target/release/belarusian /app/belarusian

# Даем права на исполнение
RUN chmod +x /app/belarusian

# Переключаемся на безопасного пользователя
USER appuser

# Порт, который слушает твой Axum (из main.rs)
EXPOSE 8080

# Переменные окружения по умолчанию
ENV PORT=8080
ENV RUST_LOG=info

# Запуск
CMD ["./belarusian"]