# SBOM and artifact attestations

Release hardening status for `healthpoint-rs`.

## Current state

- GitHub Release artifacts are built by `.github/workflows/release.yml`.
- Each binary archive has a SHA-256 sidecar.
- GitHub Actions has `id-token: write` enabled in the release workflow, so artifact attestations can be added without changing repository secrets.

## Target state

- Generate an SPDX SBOM for every tagged release.
- Upload SBOM files to the GitHub Release.
- Attest each release archive and checksum with GitHub artifact attestations.
- Keep signing/notarization separate from Healthpoint data/licensing approval.

## Local verification

```bash
gh release view v0.1.0 --json assets
shasum -a 256 -c <artifact>.sha256
```
