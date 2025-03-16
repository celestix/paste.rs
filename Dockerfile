FROM rust:1.84.1 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder /app/target/release/paste /paste
ENV RUST_LOG=debug
ENV RUST_BACKTRACE=1
EXPOSE 8080
ENTRYPOINT ["/paste"]