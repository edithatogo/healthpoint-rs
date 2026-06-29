# Healthpoint resource URIs

The project uses a small internal URI scheme for read-only resources:

```text
healthpoint://service/{id}
healthpoint://location/{id}
healthpoint://organization/{id}
```

These URIs are not Healthpoint API URLs. They are stable adapter-facing identifiers used by the CLI and MCP server.

Examples:

```bash
healthpoint get uri healthpoint://service/svc-example --format json
```

MCP equivalent:

```text
healthpoint_read_resource_uri({"uri":"healthpoint://service/svc-example"})
```

FHIR ids are validated before being used as URL path segments. The client rejects slashes, empty ids, and dot-only path segments.
