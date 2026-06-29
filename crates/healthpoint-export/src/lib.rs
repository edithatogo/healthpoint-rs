//! Export helpers for local, licensed, and future open-approved Healthpoint outputs.

#![forbid(unsafe_code)]

use std::io::Write;

use chrono::{DateTime, Utc};
use healthpoint_core::{
    AccessPolicy, LocationRecord, OrganizationRecord, Result, ServiceRecord, SourceProvenance,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Export manifest attached to generated files.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExportManifest {
    /// Manifest schema version.
    pub schema_version: String,
    /// Source provenance.
    pub provenance: SourceProvenance,
    /// Access policy.
    pub access_policy: AccessPolicy,
    /// Export creation time.
    pub created_at: DateTime<Utc>,
    /// Whether real Healthpoint-derived data is present.
    pub contains_healthpoint_data: bool,
    /// Human-readable warning.
    pub warning: String,
}

impl ExportManifest {
    /// Create a manifest from provenance.
    pub fn new(provenance: SourceProvenance, contains_healthpoint_data: bool) -> Self {
        Self {
            schema_version: "healthpoint.export-manifest.v1".into(),
            access_policy: provenance.access_policy.clone(),
            provenance,
            created_at: Utc::now(),
            contains_healthpoint_data,
            warning: "Do not redistribute Healthpoint-derived data unless your licence/terms permit it.".into(),
        }
    }
}

/// Supported data export formats for service records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ServiceExportFormat {
    /// Pretty JSON array.
    Json,
    /// Newline-delimited JSON.
    Jsonl,
    /// Conservative flat CSV.
    Csv,
}

/// Write service records in a selected format.
pub fn write_services<W: Write>(records: &[ServiceRecord], format: ServiceExportFormat, writer: W) -> Result<()> {
    match format {
        ServiceExportFormat::Json => write_services_json(records, writer),
        ServiceExportFormat::Jsonl => write_services_jsonl(records, writer),
        ServiceExportFormat::Csv => write_services_csv(records, writer),
    }
}

/// Write service records as pretty JSON.
pub fn write_services_json<W: Write>(records: &[ServiceRecord], mut writer: W) -> Result<()> {
    serde_json::to_writer_pretty(&mut writer, records)
        .map_err(|err| healthpoint_core::HealthpointError::Parse(err.to_string()))?;
    writer
        .write_all(b"\n")
        .map_err(|err| healthpoint_core::HealthpointError::Request(err.to_string()))?;
    Ok(())
}

/// Write records as newline-delimited JSON.
pub fn write_services_jsonl<W: Write>(records: &[ServiceRecord], mut writer: W) -> Result<()> {
    for record in records {
        serde_json::to_writer(&mut writer, record)
            .map_err(|err| healthpoint_core::HealthpointError::Parse(err.to_string()))?;
        writer
            .write_all(b"\n")
            .map_err(|err| healthpoint_core::HealthpointError::Request(err.to_string()))?;
    }
    Ok(())
}

/// Write a conservative flat CSV service view.
pub fn write_services_csv<W: Write>(records: &[ServiceRecord], writer: W) -> Result<()> {
    let mut wtr = csv::Writer::from_writer(writer);
    wtr.write_record([
        "id",
        "name",
        "active",
        "provided_by_reference",
        "provided_by_display",
        "category_codes",
        "service_type_codes",
        "specialty_codes",
        "program_codes",
        "communication_codes",
        "location_references",
        "coverage_area_references",
        "appointment_required",
        "comment",
        "retrieved_at",
    ])
    .map_err(|err| healthpoint_core::HealthpointError::Request(err.to_string()))?;

    for record in records {
        let provider = record.provided_by.as_ref();
        let active = record.active.map(|b| b.to_string()).unwrap_or_default();
        let categories = join_codes(&record.categories);
        let service_types = join_codes(&record.service_types);
        let specialties = join_codes(&record.specialties);
        let programs = join_codes(&record.programs);
        let communications = join_codes(&record.communications);
        let locations = join_references(&record.locations);
        let coverage_areas = join_references(&record.coverage_areas);
        let appointment_required = record
            .appointment_required
            .map(|value| value.to_string())
            .unwrap_or_default();
        let retrieved_at = record.provenance.retrieved_at.to_rfc3339();
        wtr.write_record([
            record.id.as_str(),
            record.name.as_deref().unwrap_or_default(),
            active.as_str(),
            provider.map(|p| p.reference.as_str()).unwrap_or_default(),
            provider.and_then(|p| p.display.as_deref()).unwrap_or_default(),
            categories.as_str(),
            service_types.as_str(),
            specialties.as_str(),
            programs.as_str(),
            communications.as_str(),
            locations.as_str(),
            coverage_areas.as_str(),
            appointment_required.as_str(),
            record.comment.as_deref().unwrap_or_default(),
            retrieved_at.as_str(),
        ])
        .map_err(|err| healthpoint_core::HealthpointError::Request(err.to_string()))?;
    }
    wtr.flush()
        .map_err(|err| healthpoint_core::HealthpointError::Request(err.to_string()))?;
    Ok(())
}

/// Write location records as pretty JSON.
pub fn write_locations_json<W: Write>(records: &[LocationRecord], mut writer: W) -> Result<()> {
    serde_json::to_writer_pretty(&mut writer, records)
        .map_err(|err| healthpoint_core::HealthpointError::Parse(err.to_string()))?;
    writer
        .write_all(b"\n")
        .map_err(|err| healthpoint_core::HealthpointError::Request(err.to_string()))?;
    Ok(())
}

/// Write organization records as pretty JSON.
pub fn write_organizations_json<W: Write>(records: &[OrganizationRecord], mut writer: W) -> Result<()> {
    serde_json::to_writer_pretty(&mut writer, records)
        .map_err(|err| healthpoint_core::HealthpointError::Parse(err.to_string()))?;
    writer
        .write_all(b"\n")
        .map_err(|err| healthpoint_core::HealthpointError::Request(err.to_string()))?;
    Ok(())
}

fn join_codes(codes: &[healthpoint_core::Code]) -> String {
    codes
        .iter()
        .map(|code| code.as_token())
        .collect::<Vec<_>>()
        .join(";")
}

fn join_references(references: &[healthpoint_core::ResourceReference]) -> String {
    references
        .iter()
        .map(|reference| reference.reference.clone())
        .collect::<Vec<_>>()
        .join(";")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_csv_with_expected_headers() {
        let records = healthpoint_fhir::services_from_fhir(
            healthpoint_testkit::healthcare_service_bundle(),
            healthpoint_core::SourceProvenance::healthpoint("mock"),
        )
        .expect("fixture maps");
        let mut bytes = Vec::new();
        write_services_csv(&records, &mut bytes).expect("csv writes");
        let rendered = String::from_utf8(bytes).expect("utf8");
        assert!(rendered.starts_with("id,name,active"));
        assert!(rendered.contains("svc-cervical-screening-1"));
    }
}
