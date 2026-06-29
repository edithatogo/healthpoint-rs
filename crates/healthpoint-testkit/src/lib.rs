//! Synthetic Healthpoint/FHIR fixtures for tests.

#![forbid(unsafe_code)]

use serde_json::Value;

/// Synthetic HealthcareService search bundle.
pub fn healthcare_service_bundle() -> Value {
    serde_json::from_str(include_str!("../fixtures/fhir-bundle-healthcare-service.json"))
        .expect("synthetic fixture is valid JSON")
}
