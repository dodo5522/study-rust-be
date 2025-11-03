FROM rust:1.90.0-trixie as builder
WORKDIR /tmp/backend
COPY ./layers ./layers
COPY ./migration ./migration
COPY ./src ./src
COPY Cargo.toml ./
RUN cargo build --release

FROM debian:trixie-20251020-slim
COPY --from=builder /tmp/backend/target/release/study-rust-be /usr/local/bin/study-rust-be
CMD ["study-rust-be"]
