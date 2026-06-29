//! FHIR R4 mapping helpers for Healthpoint resources.
//!
//! The mapper is deliberately tolerant: it preserves raw FHIR JSON and extracts the fields needed
//! by the CLI/MCP layers without requiring generated full-resource Rust bindings.

#![forbid(unsafe_code)]

use healthpoint_core::{
    Code, ContactPoint, HealthpointError, OrganizationRecord, ResourceReference, Result,
    ServiceRecord, SourceProvenance,
};
use serde_json::Value;

/// Convert a FHIR search response or single resource into service records.
pub fn services_from_fhir(value: Value, provenance: SourceProvenance) -> Result<Vec<ServiceRecord>> {
    match value.get("resourceType").and_then(Value::as_str) {
        Some("Bundle") => bundle_entries(value)
            .into_iter()
            .filter(|resource| resource.get("resourceType").and_then(Value::as_str) == Some("HealthcareService"))
            .map(|resource| service_from_resource(resource, provenance.clone()))
            .collect(),
        Some("HealthcareService") => Ok(vec![service_from_resource(value, provenance)?]),
        Some(other) => Err(HealthpointError::Parse(format!(
            "expected Bundle or HealthcareService, got {other}"
        ))),
        None => Err(HealthpointError::Parse("missing FHIR resourceType".into())),
    }
}

/// Convert a FHIR Organization resource into an organisation record.
pub fn organization_from_fhir(
    value: Value,
    provenance: SourceProvenance,
) -> Result<OrganizationRecord> {
    ensure_resource_type(&value, "Organization")?;
    Ok(OrganizationRecord {
        id: required_id(&value)?,
        name: string_field(&value, "name"),
        active: bool_field(&value, "active"),
        contacts: telecom_points(&value),
        provenance,
        raw_fhir: value,
    })
}

/// Extract next-page URL/cursor from a FHIR Bundle.
pub fn next_link(value: &Value) -> Option<String> {
    value
        .get("link")?
        .as_array()?
        .iter()
        .find(|link| link.get("relation").and_then(Value::as_str) == Some("next"))?
        .get("url")?
        .as_str()
        .map(ToOwned::to_owned)
}

/// Extract total count from a FHIR Bundle.
pub fn total(value: &Value) -> Option<u64> {
    value.get("total").and_then(Value::as_u64)
}

fn service_from_resource(value: Value, provenance: SourceProvenance) -> Result<ServiceRecord> {
    ensure_resource_type(&value, "HealthcareService")?;
    Ok(ServiceRecord {
        id: required_id(&value)?,
        name: string_field(&value, "name"),
        active: bool_field(&value, "active"),
        provided_by: value.get("providedBy").and_then(reference_from_value),
        locations: references_field(&value, "location"),
        categories: codeable_concepts_field(&value, "category"),
        service_types: codeable_concepts_field(&value, "type"),
        specialties: codeable_concepts_field(&value, "specialty"),
        contacts: telecom_points(&value),
        provenance,
        raw_fhir: value,
    })
}

fn bundle_entries(value: Value) -> Vec<Value> {
    value
        .get("entry")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.get("resource").cloned())
        .collect()
}

fn ensure_resource_type(value: &Value, expected: &str) -> Result<()> {
    match value.get("resourceType").and_then(Value::as_str) {
        Some(actual) if actual == expected => Ok(()),
        Some(actual) => Err(HealthpointError::Parse(format!(
            "expected {expected}, got {actual}"
        ))),
        None => Err(HealthpointError::Parse("missing FHIR resourceType".into())),
    }
}

fn required_id(value: &Value) -> Result<String> {
    value
        .get("id")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .ok_or_else(|| HealthpointError::Parse("FHIR resource is missing id".into()))
}

fn string_field(value: &Value, name: &str) -> Option<String> {
    value.get(name).and_then(Value::as_str).map(ToOwned::to_owned)
}

fn bool_field(value: &Value, name: &str) -> Option<bool> {
    value.get(name).and_then(Value::as_bool)
}

fn references_field(value: &Value, name: &str) -> Vec<ResourceReference> {
    value
        .get(name)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(reference_from_value)
        .collect()
}

fn reference_from_value(value: &Value) -> Option<ResourceReference> {
    Some(ResourceReference {
        reference: value.get("reference")?.as_str()?.to_owned(),
        display: value
            .get("display")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned),
    })
}

fn codeable_concepts_field(value: &Value, name: &str) -> Vec<Code> {
    value
        .get(name)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .flat_map(codes_from_codeable_concept)
        .collect()
}

fn codes_from_codeable_concept(value: &Value) -> Vec<Code> {
    value
        .get("coding")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|coding| {
            Some(Code {
                system: coding
                    .get("system")
                    .and_then(Value::as_str)
                    .map(ToOwned::to_owned),
                code: coding.get("code")?.as_str()?.to_owned(),
                display: coding
                    .get("display")
                    .and_then(Value::as_str)
                    .map(ToOwned::to_owned),
            })
        })
        .collect()
}

fn telecom_points(value: &Value) -> Vec<ContactPoint> {
    value
        .get("telecom")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(|point| ContactPoint {
            system: point
                .get("system")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned),
            value: point
                .get("value")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned),
            use_code: point
                .get("use")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_synthetic_healthcare_service_bundle() {
        let value: Value = serde_json::from_str(include_str!(
            "../../healthpoint-testkit/fixtures/fhir-bundle-healthcare-service.json"
        ))
        .expect("fixture is valid JSON");
        let records = services_from_fhir(value, SourceProvenance::healthpoint("mock"))
            .expect("fixture maps");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].id, "svc-cervical-screening-1");
        assert_eq!(records[0].service_types[0].code, "171149006");
    }
}
