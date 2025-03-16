FROM rust:1.84.1 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/release/paste /app/paste
EXPOSE 8080
CMD ["/app/paste"]