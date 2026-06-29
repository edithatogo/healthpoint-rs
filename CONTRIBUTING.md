# Contributing

This project is scaffold-stage. Contributions should preserve the core boundary:

- read-only by default,
- bring-your-own-key,
- no real Healthpoint data in Git,
- FHIR-first with tabular views as adapters,
- provenance on all exported data.

Before adding a new command, ask:

1. Does it require Healthpoint terms review?
2. Could it accidentally bulk-export or redistribute licensed data?
3. Does it expose secrets through logs, errors, or MCP outputs?
4. Can it be tested with synthetic fixtures?
