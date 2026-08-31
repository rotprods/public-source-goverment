FROM rust:1.98-bookworm AS builder
WORKDIR /workspace
COPY Cargo.toml rust-toolchain.toml ./
COPY crates ./crates
COPY services ./services
RUN cargo build --release -p civic-api

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
RUN useradd --system --uid 10001 --create-home civic
COPY --from=builder /workspace/target/release/civic-api /usr/local/bin/civic-api
USER civic
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/civic-api"]
