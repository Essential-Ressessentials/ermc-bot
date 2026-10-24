FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:latest

WORKDIR /app

COPY --from=builder --chmod=755 /app/target/release/ermc-bot /app/ermc-bot

CMD ["/app/ermc-bot"]
