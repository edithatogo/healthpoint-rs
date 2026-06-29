//! FHIR R4 mapping helpers for Healthpoint resources.
//!
//! The mapper is deliberately tolerant: it preserves raw FHIR JSON and extracts the fields needed
//! by the CLI/MCP layers without requiring generated full-resource Rust bindings.

#![forbid(unsafe_code)]

use healthpoint_core::{
    Address, AvailableTime, Code, ContactPoint, Eligibility, GeoPosition, HealthpointError,
    Identifier, LocationRecord, NotAvailable, OrganizationRecord, Period, ResourceReference,
    Result, ServiceRecord, SourceProvenance,
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

/// Convert a FHIR search response or single resource into organization records.
pub fn organizations_from_fhir(
    value: Value,
    provenance: SourceProvenance,
) -> Result<Vec<OrganizationRecord>> {
    resources_from_fhir(value, "Organization", provenance, organization_from_resource)
}

/// Convert a FHIR Organization resource into an organisation record.
pub fn organization_from_fhir(
    value: Value,
    provenance: SourceProvenance,
) -> Result<OrganizationRecord> {
    organization_from_resource(value, provenance)
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
        identifiers: identifiers_field(&value),
        name: string_field(&value, "name"),
        active: bool_field(&value, "active"),
        provided_by: value.get("providedBy").and_then(reference_from_value),
        locations: references_field(&value, "location"),
        coverage_areas: references_field(&value, "coverageArea"),
        endpoints: references_field(&value, "endpoint"),
        categories: codeable_concepts_field(&value, "category"),
        service_types: codeable_concepts_field(&value, "type"),
        specialties: codeable_concepts_field(&value, "specialty"),
        service_provision_codes: codeable_concepts_field(&value, "serviceProvisionCode"),
        programs: codeable_concepts_field(&value, "program"),
        characteristics: codeable_concepts_field(&value, "characteristic"),
        communications: codeable_concepts_field(&value, "communication"),
        referral_methods: codeable_concepts_field(&value, "referralMethod"),
        eligibilities: eligibilities_field(&value),
        appointment_required: bool_field(&value, "appointmentRequired"),
        comment: string_field(&value, "comment"),
        extra_details: string_field(&value, "extraDetails"),
        available_times: available_times_field(&value, "availableTime"),
        not_available: not_available_field(&value),
        contacts: telecom_points(&value),
        provenance,
        raw_fhir: value,
    })
}

fn organization_from_resource(value: Value, provenance: SourceProvenance) -> Result<OrganizationRecord> {
    ensure_resource_type(&value, "Organization")?;
    Ok(OrganizationRecord {
        id: required_id(&value)?,
        identifiers: identifiers_field(&value),
        organization_types: codeable_concepts_field(&value, "type"),
        name: string_field(&value, "name"),
        aliases: string_array_field(&value, "alias"),
        active: bool_field(&value, "active"),
        part_of: value.get("partOf").and_then(reference_from_value),
        endpoints: references_field(&value, "endpoint"),
        contacts: telecom_points(&value),
        provenance,
        raw_fhir: value,
    })
}

fn location_from_resource(value: Value, provenance: SourceProvenance) -> Result<LocationRecord> {
    ensure_resource_type(&value, "Location")?;
    Ok(LocationRecord {
        id: required_id(&value)?,
        identifiers: identifiers_field(&value),
        name: string_field(&value, "name"),
        status: string_field(&value, "status"),
        mode: string_field(&value, "mode"),
        location_types: codeable_concepts_field(&value, "type"),
        physical_types: value
            .get("physicalType")
            .map(codes_from_codeable_concept)
            .unwrap_or_default(),
        contacts: telecom_points(&value),
        address: value.get("address").and_then(address_from_value),
        position: value.get("position").and_then(position_from_value),
        managing_organization: value
            .get("managingOrganization")
            .and_then(reference_from_value),
        part_of: value.get("partOf").and_then(reference_from_value),
        endpoints: references_field(&value, "endpoint"),
        hours_of_operation: available_times_field(&value, "hoursOfOperation"),
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

fn string_array_field(value: &Value, name: &str) -> Vec<String> {
    value
        .get(name)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(ToOwned::to_owned)
        .collect()
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

fn identifiers_field(value: &Value) -> Vec<Identifier> {
    value
        .get("identifier")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(|identifier| Identifier {
            use_code: string_field(identifier, "use"),
            system: string_field(identifier, "system"),
            value: string_field(identifier, "value"),
        })
        .collect()
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

fn eligibilities_field(value: &Value) -> Vec<Eligibility> {
    value
        .get("eligibility")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(|eligibility| Eligibility {
            codes: eligibility
                .get("code")
                .map(codes_from_codeable_concept)
                .unwrap_or_default(),
            comment: string_field(eligibility, "comment"),
        })
        .collect()
}

fn available_times_field(value: &Value, name: &str) -> Vec<AvailableTime> {
    value
        .get(name)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(|available| AvailableTime {
            days_of_week: string_array_field(available, "daysOfWeek"),
            all_day: bool_field(available, "allDay"),
            available_start_time: string_field(available, "availableStartTime")
                .or_else(|| string_field(available, "openingTime")),
            available_end_time: string_field(available, "availableEndTime")
                .or_else(|| string_field(available, "closingTime")),
        })
        .collect()
}

fn not_available_field(value: &Value) -> Vec<NotAvailable> {
    value
        .get("notAvailable")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(|item| NotAvailable {
            description: string_field(item, "description"),
            during: item.get("during").map(period_from_value),
        })
        .collect()
}

fn period_from_value(value: &Value) -> Period {
    Period {
        start: string_field(value, "start"),
        end: string_field(value, "end"),
    }
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
        line: string_array_field(value, "line"),
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
        assert_eq!(records[0].eligibilities[0].comment.as_deref(), Some("Synthetic eligibility comment"));
        assert_eq!(records[0].available_times[0].available_start_time.as_deref(), Some("09:00:00"));
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
        assert_eq!(record.hours_of_operation[0].days_of_week[0], "mon");
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
        assert_eq!(record.aliases[0], "Synthetic CHT");
    }
}
