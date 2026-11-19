# Bounded parser fuzzing checkpoint — 2026-11-20

The approved increment adds a shared assertion helper, two native tests, an isolated
libFuzzer workspace with a committed lockfile, synthetic FHIR/URI seeds and read-only
30-second PR / 600-second weekly CI. See `../../fuzz/README.md`.

`cargo +1.99.1 test --locked -p healthpoint-fhir --test fuzz_regressions` executed:
2 passed, 0 failed. The first assertion draft overconstrained newline normalization;
it was corrected to test the existing redaction contract, without production edits.
Sanitizer campaign and exact-revision hosted CI are still pending at this checkpoint.

This does not qualify live Healthpoint profiles, change the public API, lift any
external data/terms gate, or close the earlier registry/distribution work. The new
nightly is isolated to fuzzing; stable/MSRV policy remains in the main workflow.
Rollback is a revert of this focused test-infrastructure change.

## Admission evidence and limits

Base: `18de17db661cea66904e6a61c2ad6521c3105076`. The two native
regressions were independently rerun with Rust 1.99.1 and passed (0.12 seconds).
Changed-target scoped Clippy without dependency lints passed in the implementation
pass. Normal Clippy reaches existing `double_must_use` diagnostics in unchanged
core trait definitions; this test-only change does not suppress those findings.

An earlier harness revision's instrumented local campaign was terminated by SIGKILL after reporting
21,548 executions, before a completed budget receipt. It is not counted as a
passing campaign. Default-sanitizer execution remains an exact-head hosted gate.
Actionlint 1.7.9 syntax/expression validation and `git diff --check` passed.
No source, API, live service call or real service data is introduced. PR #111
retains ownership of its existing Serde/FHIR review.

The final preflight initially rejected a credential-shaped synthetic fixture.
That fixture was removed: header redaction now uses an empty header, and known
value redaction uses a plain text marker. No scanner rule or allowlist changed. The preflight then passed, and both
final native regression tests passed again (0.09 seconds).
