FROM rust:1.96.1-trixie as builder
WORKDIR /tmp/backend
COPY ./crates ./crates
COPY ./src ./src
COPY ./Cargo.toml ./
COPY ./rust-toolchain.toml ./
RUN cargo build --release

FROM debian:trixie-20251020-slim
COPY --from=builder /tmp/backend/target/release/energy-tracer /usr/local/bin/energy-tracer
CMD ["energy-tracer"]
