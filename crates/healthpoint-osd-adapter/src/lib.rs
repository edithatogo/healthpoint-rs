//! Future bridge from Healthpoint graph/FHIR records into open_social_data-style tabular views.
//!
//! This crate intentionally does not depend on `open_social_data` yet. It defines stable tabular
//! views that can later be implemented as a provider plugin once licensing/export policy is clear.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;

use healthpoint_core::{LocationRecord, ServiceRecord};
use serde::{Deserialize, Serialize};

/// Supported Healthpoint tabular views.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HealthpointView {
    /// One row per HealthcareService.
    Services,
    /// One row per Location.
    Locations,
    /// Many-to-many service-location edges.
    ServiceLocations,
    /// One row per service coding.
    ServiceCodes,
    /// One row per contact point.
    ServiceContacts,
}

/// Simple row representation suitable for CSV/Parquet adapters.
pub type Row = BTreeMap<String, String>;

/// Convert services into a named tabular view.
pub fn service_rows(view: HealthpointView, services: &[ServiceRecord]) -> Vec<Row> {
    match view {
        HealthpointView::Services => services.iter().map(service_row).collect(),
        HealthpointView::Locations => Vec::new(),
        HealthpointView::ServiceLocations => services
            .iter()
            .flat_map(|service| {
                service.locations.iter().map(move |location| {
                    let mut row = Row::new();
                    row.insert("service_id".into(), service.id.clone());
                    row.insert("location_reference".into(), location.reference.clone());
                    row.insert("location_display".into(), location.display.clone().unwrap_or_default());
                    row
                })
            })
            .collect(),
        HealthpointView::ServiceCodes => services
            .iter()
            .flat_map(service_code_rows)
            .collect(),
        HealthpointView::ServiceContacts => services
            .iter()
            .flat_map(|service| {
                service.contacts.iter().map(move |contact| {
                    let mut row = Row::new();
                    row.insert("service_id".into(), service.id.clone());
                    row.insert("system".into(), contact.system.clone().unwrap_or_default());
                    row.insert("value".into(), contact.value.clone().unwrap_or_default());
                    row.insert("use".into(), contact.use_code.clone().unwrap_or_default());
                    row
                })
            })
            .collect(),
    }
}

/// Convert locations into rows for the `locations` view.
pub fn location_rows(locations: &[LocationRecord]) -> Vec<Row> {
    locations
        .iter()
        .map(|location| {
            let mut row = Row::new();
            row.insert("id".into(), location.id.clone());
            row.insert("name".into(), location.name.clone().unwrap_or_default());
            row.insert("status".into(), location.status.clone().unwrap_or_default());
            row.insert("mode".into(), location.mode.clone().unwrap_or_default());
            row.insert(
                "address_text".into(),
                location
                    .address
                    .as_ref()
                    .and_then(|address| address.text.clone())
                    .unwrap_or_default(),
            );
            row.insert(
                "latitude".into(),
                location
                    .position
                    .map(|position| position.latitude.to_string())
                    .unwrap_or_default(),
            );
            row.insert(
                "longitude".into(),
                location
                    .position
                    .map(|position| position.longitude.to_string())
                    .unwrap_or_default(),
            );
            row.insert(
                "managing_organization_reference".into(),
                location
                    .managing_organization
                    .as_ref()
                    .map(|org| org.reference.clone())
                    .unwrap_or_default(),
            );
            row.insert(
                "retrieved_at".into(),
                location.provenance.retrieved_at.to_rfc3339(),
            );
            row
        })
        .collect()
}

fn service_row(service: &ServiceRecord) -> Row {
    let mut row = Row::new();
    row.insert("id".into(), service.id.clone());
    row.insert("name".into(), service.name.clone().unwrap_or_default());
    row.insert("active".into(), service.active.map(|b| b.to_string()).unwrap_or_default());
    row.insert(
        "provided_by_reference".into(),
        service
            .provided_by
            .as_ref()
            .map(|p| p.reference.clone())
            .unwrap_or_default(),
    );
    row.insert(
        "appointment_required".into(),
        service
            .appointment_required
            .map(|value| value.to_string())
            .unwrap_or_default(),
    );
    row.insert(
        "retrieved_at".into(),
        service.provenance.retrieved_at.to_rfc3339(),
    );
    row
}

fn service_code_rows(service: &ServiceRecord) -> Vec<Row> {
    let mut rows = Vec::new();
    for (field, codes) in [
        ("category", &service.categories),
        ("type", &service.service_types),
        ("specialty", &service.specialties),
        ("program", &service.programs),
        ("communication", &service.communications),
        ("referral_method", &service.referral_methods),
    ] {
        for code in codes {
            let mut row = Row::new();
            row.insert("service_id".into(), service.id.clone());
            row.insert("field".into(), field.into());
            row.insert("system".into(), code.system.clone().unwrap_or_default());
            row.insert("code".into(), code.code.clone());
            row.insert("display".into(), code.display.clone().unwrap_or_default());
            rows.push(row);
        }
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_code_view_includes_communications() {
        let value = healthpoint_testkit::healthcare_service_bundle();
        let services = healthpoint_fhir::services_from_fhir(
            value,
            healthpoint_core::SourceProvenance::healthpoint("mock"),
        )
        .expect("fixture maps");
        let rows = service_rows(HealthpointView::ServiceCodes, &services);
        assert!(rows.iter().any(|row| row.get("field").map(String::as_str) == Some("communication")));
    }
}
