#!/usr/bin/env bash
# Run only the checked-in synthetic corpus. Keep crashes and exact replay metadata.
set -euo pipefail
fuzz_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$fuzz_dir"
seconds="${1:-30}"
if [[ ! "$seconds" =~ ^[0-9]+$ ]] || (( seconds < 1 || seconds > 3600 )); then
  echo "budget must be 1..3600 seconds" >&2
  exit 2
fi
nightly=nightly-2026-11-19
target=fhir_mapping
if [[ "$(cargo +"$nightly" fuzz --version)" != 'cargo-fuzz 0.13.2' ]]; then
  echo 'install cargo-fuzz 0.13.2 before running this pinned harness' >&2
  exit 2
fi
shopt -s nullglob
seeds=("$fuzz_dir/corpus/$target"/*)
if (( ${#seeds[@]} == 0 )); then
  echo 'committed synthetic corpus is missing; refusing an empty-seed campaign' >&2
  exit 1
fi
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
mkdir -p "target/evidence" "target/corpus/$target" "artifacts/$target"
{
  git rev-parse HEAD
  cargo +"$nightly" --version
  rustc +"$nightly" --version
  cargo +"$nightly" fuzz --version
  sha256sum Cargo.lock
  printf 'target=%s budget_seconds=%s max_len=65536 timeout_seconds=5 rss_mb=1024 malloc_mb=512 seed=1\n' "$target" "$seconds"
} > target/evidence/metadata.txt
# cargo-fuzz 0.13.2 has no --locked option. Resolve with --locked first,
# build offline, and reject any lockfile change before executing input bytes.
cargo +"$nightly" fetch --locked --manifest-path Cargo.toml
lock_before="$(sha256sum Cargo.lock)"
CARGO_NET_OFFLINE=true cargo +"$nightly" fuzz build --fuzz-dir "$fuzz_dir" --codegen-units 16 "$target" 2>&1 | tee target/evidence/build.log
if [[ "$lock_before" != "$(sha256sum Cargo.lock)" ]]; then
  echo 'fuzz build changed Cargo.lock; refresh and review it explicitly' >&2
  exit 1
fi
# Build first so the campaign watchdog measures execution, not compilation.
# New corpus entries are transient; checked-in seeds stay unchanged.
CARGO_NET_OFFLINE=true timeout --kill-after=10 "$((seconds + 60))" \
  cargo +"$nightly" fuzz run --fuzz-dir "$fuzz_dir" --codegen-units 16 \
  "$target" "target/corpus/$target" "corpus/$target" -- \
  -max_total_time="$seconds" -max_len=65536 -timeout=5 \
  -rss_limit_mb=1024 -malloc_limit_mb=512 -seed=1 -print_final_stats=1 \
  -artifact_prefix="$fuzz_dir/artifacts/$target/" 2>&1 | tee target/evidence/campaign.log
