set shell := ["bash", "-euo", "pipefail", "-c"]

default:
  @just --list

bootstrap:
  corepack enable
  pnpm install --frozen-lockfile
  cargo fetch --locked

up:
  docker compose up -d postgres valkey minio otel-collector

down:
  docker compose down

api:
  RUST_LOG=${RUST_LOG:-info} cargo run -p civic-api

web:
  pnpm --filter @civic/web dev

fmt:
  cargo fmt --all

check:
  cargo fmt --all -- --check
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  cargo test --workspace
  cargo build --workspace --release
  pnpm -r typecheck
  pnpm --filter @civic/web build

security:
  cargo audit
  cargo deny check
  pnpm audit --audit-level high

clean:
  cargo clean
  pnpm -r exec -- rm -rf .next dist coverage || true
