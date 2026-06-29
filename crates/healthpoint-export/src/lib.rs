//! Export helpers for local, licensed, and future open-approved Healthpoint outputs.

#![forbid(unsafe_code)]

use std::io::Write;

use chrono::{DateTime, Utc};
use healthpoint_core::{AccessPolicy, Result, ServiceRecord, SourceProvenance};
use serde::{Deserialize, Serialize};

/// Export manifest attached to generated files.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
        "service_type_codes",
        "specialty_codes",
        "location_references",
        "coverage_area_references",
        "appointment_required",
        "retrieved_at",
    ])
    .map_err(|err| healthpoint_core::HealthpointError::Request(err.to_string()))?;

    for record in records {
        let provider = record.provided_by.as_ref();
        let active = record.active.map(|b| b.to_string()).unwrap_or_default();
        let service_types = record
            .service_types
            .iter()
            .map(|code| code.as_token())
            .collect::<Vec<_>>()
            .join(";");
        let specialties = record
            .specialties
            .iter()
            .map(|code| code.as_token())
            .collect::<Vec<_>>()
            .join(";");
        let locations = record
            .locations
            .iter()
            .map(|location| location.reference.clone())
            .collect::<Vec<_>>()
            .join(";");
        let coverage_areas = record
            .coverage_areas
            .iter()
            .map(|area| area.reference.clone())
            .collect::<Vec<_>>()
            .join(";");
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
            service_types.as_str(),
            specialties.as_str(),
            locations.as_str(),
            coverage_areas.as_str(),
            appointment_required.as_str(),
            retrieved_at.as_str(),
        ])
        .map_err(|err| healthpoint_core::HealthpointError::Request(err.to_string()))?;
    }
    wtr.flush()
        .map_err(|err| healthpoint_core::HealthpointError::Request(err.to_string()))?;
    Ok(())
}
