set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

setup:
  bin/conductor-setup

status:
  bin/conductor-status

preflight:
  scripts/static-preflight.py
  scripts/generate-contract-schemas.py
  git diff --check

fmt:
  cargo fmt --all

check:
  cargo check --workspace --all-targets

clippy:
  cargo clippy --workspace --all-targets -- -D warnings

test:
  cargo test --workspace

deny:
  cargo deny check

ci: preflight fmt check clippy test deny

metadata-check:
  scripts/static-preflight.py

schemas:
  scripts/generate-contract-schemas.py

mock-server:
  bin/mock-healthpoint-server --port 8787

mock-smoke:
  #!/usr/bin/env bash
  set -euo pipefail
  bin/mock-healthpoint-server --port 8787 --quiet > /tmp/healthpoint-mock.log 2>&1 &
  pid=$$!
  trap 'kill "$pid" 2>/dev/null || true' EXIT
  sleep 1
  python3 - <<'PY'
  import json, urllib.request
  base='http://127.0.0.1:8787'
  for path, expected in [('/metadata','CapabilityStatement'),('/HealthcareService?_count=1','Bundle'),('/HealthcareService/svc-cervical-screening-1','HealthcareService')]:
      with urllib.request.urlopen(base+path, timeout=5) as response:
          data=json.load(response)
      assert data.get('resourceType') == expected, (path, data.get('resourceType'))
  print('mock smoke ok')
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
