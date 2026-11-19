# Bounded parser fuzzing

## Scope and invariants

Successful FHIR mappings preserve the complete raw JSON resource, its ID and fixed provenance, including unknown extensions through serde serialization. Resource URIs round trip. Diagnostic redaction removes recognized header markers and a plain synthetic text marker. All three resource mappings and pagination helpers are exercised.

The target includes the same assertion helper as the native `fuzz_regressions`
integration test. Two native tests replay the corpus and deterministic truncation,
bit-flip and suffix mutations, so ordinary CI executes these invariants even
without libFuzzer. The libFuzzer workspace is independent of the production
workspace and adds no production dependency or feature.

All committed seeds are synthetic: authored here or copied from existing repository
fixtures. The minimal PDF, where present, is generated from simple locally authored
objects. Never add real messages, patient data, downloaded papers, credentials,
private logs or attachments. Only synthetic campaigns may upload crash/corpus
artifacts; review any newly minimized regression before committing it.

## Run and reproduce

```bash
cargo test --locked -p healthpoint-fhir --test fuzz_regressions
rustup toolchain install nightly-2026-11-19 --profile minimal
cargo +nightly-2026-11-19 install cargo-fuzz --version 0.13.2 --locked
bash fuzz/run.sh 30
# Longer bounded local campaign (1..3600 seconds accepted):
bash fuzz/run.sh 600
```

The runner fetches using the committed fuzz `Cargo.lock` with `--locked`, builds
offline, and refuses a changed lockfile. This is explicit because cargo-fuzz
0.13.2 itself has no `--locked` flag. Nightly and cargo-fuzz are pinned;
libfuzzer-sys is exactly 0.4.13. The runner uses AddressSanitizer, debug assertions,
a fixed starting seed, a 5-second per-input timeout, 1 GiB RSS and 512 MiB
allocation limits, and an external campaign watchdog. Maximum input size is
65,536 bytes. These harness limits do not claim a production resource
sandbox or complete parser coverage. Runs can end early on any invariant,
sanitizer, allocation or timeout failure; that is a failing result, not a pass.

Crashes go to `fuzz/artifacts/fhir_mapping/`. Tool versions, commit, lock hash,
budget, compiler output and full campaign statistics are under
`fuzz/target/evidence/`. New coverage inputs stay in
`fuzz/target/corpus/fhir_mapping/`; checked-in seeds are unchanged.

Replay a saved artifact from the same commit and fuzz lockfile:

```bash
cd fuzz
cargo +nightly-2026-11-19 fuzz run fhir_mapping artifacts/fhir_mapping/ARTIFACT -- -runs=1 -timeout=5 -rss_limit_mb=1024 -malloc_limit_mb=512
cargo +nightly-2026-11-19 fuzz tmin fhir_mapping artifacts/fhir_mapping/ARTIFACT -- -max_total_time=60 -timeout=5 -rss_limit_mb=1024 -malloc_limit_mb=512
```

Minimize locally with the same resource limits and promote only a reviewed,
synthetic reproducer to the native regression corpus. A finite campaign with no
finding is evidence of that execution, not proof of parser safety.

## CI and admission

`.github/workflows/fuzz.yml` runs 30 seconds on relevant PRs/main changes and
manual runs, and 600 seconds weekly. The workflow has read-only permissions,
no persisted checkout credential, a 30-minute timeout, cancellation of superseded
PR runs, and 14-day synthetic artifacts including failure evidence. Existing
native CI retains its own toolchain policy; only this isolated fuzzer uses the
pinned nightly.
