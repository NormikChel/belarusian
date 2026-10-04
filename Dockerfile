FROM rust:1.79-slim AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin movasite

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates tzdata && rm -rf /var/lib/apt/lists/*
RUN useradd -m -u 10001 appuser
WORKDIR /app

COPY --from=builder /app/target/release/movasite /app/movasite
COPY --from=builder /app/templates /app/templates
COPY --from=builder /app/static /app/static

RUN chmod +x /app/movasite
USER appuser
ENV PORT=8080
EXPOSE 8080
CMD ["./movasite"]