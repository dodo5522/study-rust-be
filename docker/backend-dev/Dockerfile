FROM rust:1.90.0-slim-trixie
RUN cargo install cargo-watch
CMD ["cargo-watch", "--delay", "1", "--watch", "Cargo.toml", "--watch", "src", "--watch", "layers", "-x", "'run'"]
