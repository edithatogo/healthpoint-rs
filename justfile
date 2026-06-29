set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

setup:
  bin/conductor-setup

fmt:
  cargo fmt --all

check:
  cargo check --workspace --all-targets

clippy:
  cargo clippy --workspace --all-targets -- -D warnings

test:
  cargo test --workspace

ci: fmt check clippy test

mcp:
  cargo run -p healthpoint-mcp

doctor:
  cargo run -p healthpoint-cli -- doctor

search-example:
  cargo run -p healthpoint-cli -- search services --text "cervical screening" --format json
