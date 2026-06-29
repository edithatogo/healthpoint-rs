//! Future bridge from Healthpoint graph/FHIR records into open_social_data-style tabular views.
//!
//! This crate intentionally does not depend on `open_social_data` yet. It defines stable tabular
//! views that can later be implemented as a provider plugin once licensing/export policy is clear.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;

use healthpoint_core::ServiceRecord;
use serde::{Deserialize, Serialize};

/// Supported Healthpoint tabular views.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HealthpointView {
    /// One row per HealthcareService.
    Services,
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
            .flat_map(|service| {
                service
                    .service_types
                    .iter()
                    .map(move |code| {
                        let mut row = Row::new();
                        row.insert("service_id".into(), service.id.clone());
                        row.insert("field".into(), "type".into());
                        row.insert("system".into(), code.system.clone().unwrap_or_default());
                        row.insert("code".into(), code.code.clone());
                        row.insert("display".into(), code.display.clone().unwrap_or_default());
                        row
                    })
                    .chain(service.specialties.iter().map(move |code| {
                        let mut row = Row::new();
                        row.insert("service_id".into(), service.id.clone());
                        row.insert("field".into(), "specialty".into());
                        row.insert("system".into(), code.system.clone().unwrap_or_default());
                        row.insert("code".into(), code.code.clone());
                        row.insert("display".into(), code.display.clone().unwrap_or_default());
                        row
                    }))
            })
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
        "retrieved_at".into(),
        service.provenance.retrieved_at.to_rfc3339(),
    );
    row
}
