# Integration roadmap

## Target architecture

```text
healthpoint-rs
  -> Healthpoint FHIR/domain SDK
  -> CLI
  -> read-only MCP
  -> local exports
  -> optional tabular adapter

open_social_data
  -> consumes approved tabular views later
  -> handles open dataset packs/catalogues when terms permit

substack-cli-ts
  -> workflow reference only
  -> no integration target in this project
```

## Stable seams

- `DirectoryProvider`: service/organisation read interface.
- `ServiceQuery`: Healthpoint-independent query model.
- `SourceProvenance` and `AccessPolicy`: data governance boundary.
- `HealthpointView`: tabular view names for future OSD bridge.

## Future extraction candidate

If multiple repos use the same model, consider a shared crate:

```text
edithatogo-data-core
  AccessPolicy
  SourceProvenance
  ExportManifest
  QualityReport
  TabularViewProvider
```

Do not extract this until at least two integrations are real.
