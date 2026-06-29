# Security policy

## Secrets

Healthpoint API keys must only be supplied through ignored local environment/configuration. The CLI/MCP server must never print keys, auth headers, or raw request metadata that could expose credentials.

## Reporting

Please open a private security advisory or contact the maintainer if you find:

- secret leakage,
- access-control bypass behaviour,
- unsafe default caching/export behaviour,
- MCP tool descriptions that could mislead clients,
- dependency supply-chain vulnerabilities.

## Supported posture

Initial scaffold: no release support guarantee yet. Treat this as experimental until the first tagged release.
