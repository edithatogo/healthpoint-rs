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

metadata-check:
  python3 - <<'PY'
  import json, pathlib, tomllib
  for p in pathlib.Path('.').rglob('*.json'):
      if '.git' not in p.parts:
          json.loads(p.read_text())
  for p in pathlib.Path('.').rglob('*.toml'):
      if '.git' not in p.parts:
          tomllib.loads(p.read_text())
  print('json/toml ok')
  PY

mcp:
  cargo run -p healthpoint-mcp

doctor:
  cargo run -p healthpoint-cli -- doctor

fixture:
  cargo run -p healthpoint-cli -- fixture services --format human

inspect-example:
  cargo run -p healthpoint-cli -- inspect search-url --text "cervical screening" --snomed 171149006 --limit 10

search-example:
  cargo run -p healthpoint-cli -- search services --text "cervical screening" --format json

schema name:
  cargo run -p healthpoint-cli -- schema {{name}}
