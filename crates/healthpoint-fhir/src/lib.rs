//! FHIR R4 mapping helpers for Healthpoint resources.
//!
//! The mapper is deliberately tolerant: it preserves raw FHIR JSON and extracts the fields needed
//! by the CLI/MCP layers without requiring generated full-resource Rust bindings.

#![forbid(unsafe_code)]

use healthpoint_core::{
    Address, Code, ContactPoint, GeoPosition, HealthpointError, LocationRecord,
    OrganizationRecord, ResourceReference, Result, ServiceRecord, SourceProvenance,
};
use serde_json::Value;

/// Convert a FHIR search response or single resource into service records.
pub fn services_from_fhir(value: Value, provenance: SourceProvenance) -> Result<Vec<ServiceRecord>> {
    resources_from_fhir(value, "HealthcareService", provenance, service_from_resource)
}

/// Convert a FHIR search response or single resource into location records.
pub fn locations_from_fhir(value: Value, provenance: SourceProvenance) -> Result<Vec<LocationRecord>> {
    resources_from_fhir(value, "Location", provenance, location_from_resource)
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

/// Convert a FHIR Location resource into a location record.
pub fn location_from_fhir(value: Value, provenance: SourceProvenance) -> Result<LocationRecord> {
    location_from_resource(value, provenance)
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

fn resources_from_fhir<T, F>(
    value: Value,
    expected_resource_type: &str,
    provenance: SourceProvenance,
    mapper: F,
) -> Result<Vec<T>>
where
    F: Fn(Value, SourceProvenance) -> Result<T>,
{
    match value.get("resourceType").and_then(Value::as_str) {
        Some("Bundle") => bundle_entries(value)
            .into_iter()
            .filter(|resource| {
                resource.get("resourceType").and_then(Value::as_str) == Some(expected_resource_type)
            })
            .map(|resource| mapper(resource, provenance.clone()))
            .collect(),
        Some(actual) if actual == expected_resource_type => Ok(vec![mapper(value, provenance)?]),
        Some(other) => Err(HealthpointError::Parse(format!(
            "expected Bundle or {expected_resource_type}, got {other}"
        ))),
        None => Err(HealthpointError::Parse("missing FHIR resourceType".into())),
    }
}

fn service_from_resource(value: Value, provenance: SourceProvenance) -> Result<ServiceRecord> {
    ensure_resource_type(&value, "HealthcareService")?;
    Ok(ServiceRecord {
        id: required_id(&value)?,
        name: string_field(&value, "name"),
        active: bool_field(&value, "active"),
        provided_by: value.get("providedBy").and_then(reference_from_value),
        locations: references_field(&value, "location"),
        coverage_areas: references_field(&value, "coverageArea"),
        categories: codeable_concepts_field(&value, "category"),
        service_types: codeable_concepts_field(&value, "type"),
        specialties: codeable_concepts_field(&value, "specialty"),
        programs: codeable_concepts_field(&value, "program"),
        communications: codeable_concepts_field(&value, "communication"),
        referral_methods: codeable_concepts_field(&value, "referralMethod"),
        appointment_required: bool_field(&value, "appointmentRequired"),
        contacts: telecom_points(&value),
        provenance,
        raw_fhir: value,
    })
}

fn location_from_resource(value: Value, provenance: SourceProvenance) -> Result<LocationRecord> {
    ensure_resource_type(&value, "Location")?;
    Ok(LocationRecord {
        id: required_id(&value)?,
        name: string_field(&value, "name"),
        status: string_field(&value, "status"),
        mode: string_field(&value, "mode"),
        location_types: codeable_concepts_field(&value, "type"),
        contacts: telecom_points(&value),
        address: value.get("address").and_then(address_from_value),
        position: value.get("position").and_then(position_from_value),
        managing_organization: value
            .get("managingOrganization")
            .and_then(reference_from_value),
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

fn address_from_value(value: &Value) -> Option<Address> {
    Some(Address {
        text: string_field(value, "text"),
        line: value
            .get("line")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .map(ToOwned::to_owned)
            .collect(),
        city: string_field(value, "city"),
        district: string_field(value, "district"),
        state: string_field(value, "state"),
        postal_code: string_field(value, "postalCode"),
        country: string_field(value, "country"),
    })
}

fn position_from_value(value: &Value) -> Option<GeoPosition> {
    Some(GeoPosition {
        longitude: value.get("longitude")?.as_f64()?,
        latitude: value.get("latitude")?.as_f64()?,
        altitude: value.get("altitude").and_then(Value::as_f64),
    })
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
        let next = next_link(&value).expect("bundle has next link");
        assert!(next.contains("page=2"));
        assert_eq!(total(&value), Some(1));

        let records = services_from_fhir(value, SourceProvenance::healthpoint("mock"))
            .expect("fixture maps");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].id, "svc-cervical-screening-1");
        assert_eq!(records[0].service_types[0].code, "171149006");
        assert_eq!(records[0].communications[0].code, "en");
    }

    #[test]
    fn maps_synthetic_location() {
        let value: Value = serde_json::from_str(include_str!(
            "../../healthpoint-testkit/fixtures/fhir-location.json"
        ))
        .expect("fixture is valid JSON");
        let record = location_from_fhir(value, SourceProvenance::healthpoint("mock"))
            .expect("fixture maps");
        assert_eq!(record.id, "loc-auckland-clinic-1");
        assert_eq!(record.address.as_ref().and_then(|a| a.city.as_deref()), Some("Auckland"));
        assert_eq!(record.position.expect("position").latitude, -36.8485);
    }

    #[test]
    fn maps_synthetic_organization() {
        let value: Value = serde_json::from_str(include_str!(
            "../../healthpoint-testkit/fixtures/fhir-organization.json"
        ))
        .expect("fixture is valid JSON");
        let record = organization_from_fhir(value, SourceProvenance::healthpoint("mock"))
            .expect("fixture maps");
        assert_eq!(record.id, "org-example-provider-1");
        assert_eq!(record.contacts[0].system.as_deref(), Some("phone"));
    }
}
