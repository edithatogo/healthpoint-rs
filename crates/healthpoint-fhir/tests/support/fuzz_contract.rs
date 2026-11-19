//! Synthetic FHIR mapping must preserve raw bytes' JSON values and provenance.
use healthpoint_core::redaction::{redact_common_patterns, redact_known_secrets};
use healthpoint_core::{HealthpointResourceUri, SourceProvenance};
use healthpoint_fhir::{locations_from_fhir, organizations_from_fhir, services_from_fhir};
use serde_json::Value;

fn contains_resource(source: &Value, raw: &Value) -> bool {
    source == raw
        || source
            .get("entry")
            .and_then(Value::as_array)
            .is_some_and(|entries| {
                entries
                    .iter()
                    .any(|entry| entry.get("resource") == Some(raw))
            })
}

pub fn check(data: &[u8]) {
    if data.len() > 65_536 {
        return;
    }
    if let Ok(text) = std::str::from_utf8(data) {
        if let Ok(uri) = HealthpointResourceUri::parse(text) {
            assert_eq!(HealthpointResourceUri::parse(&uri.as_uri()).ok(), Some(uri));
        }
        let common = redact_common_patterns(text);
        // The helper normalizes trailing newlines; only secret removal is its contract.
        for line in common.lines() {
            let lower = line.to_ascii_lowercase();
            for marker in ["authorization:", "x-api-key", "api_key", "apikey"] {
                assert!(!lower.contains(marker));
            }
        }
        // A header without a credential still exercises the whole-line redaction rule.
        let diagnostic = format!("{text}\nAuthorization:");
        assert_eq!(
            redact_common_patterns(&diagnostic).lines().last(),
            Some("[REDACTED]")
        );
        // The known-value path uses a plain test marker, never a credential-shaped value.
        let marked = format!("{text}\nsynthetic-redaction-marker");
        assert!(
            !redact_known_secrets(&marked, ["synthetic-redaction-marker"])
                .contains("synthetic-redaction-marker")
        );
    }
    let Ok(value) = serde_json::from_slice::<Value>(data) else {
        return;
    };
    let mut provenance = SourceProvenance::healthpoint("https://synthetic.invalid/fhir");
    provenance.retrieved_at = "2026-01-01T00:00:00Z"
        .parse()
        .expect("fixed synthetic instant");
    macro_rules! check_mapping {
        ($mapper:ident, $resource_type:literal) => {
            if let Ok(records) = $mapper(value.clone(), provenance.clone()) {
                for record in records {
                    assert!(contains_resource(&value, &record.raw_fhir));
                    assert_eq!(
                        record.raw_fhir["resourceType"].as_str(),
                        Some($resource_type)
                    );
                    assert_eq!(record.raw_fhir["id"].as_str(), Some(record.id.as_str()));
                    assert_eq!(record.provenance, provenance);
                    let serialized = serde_json::to_value(&record).expect("record serializes");
                    assert_eq!(serialized["raw_fhir"], record.raw_fhir);
                }
            }
        };
    }
    check_mapping!(services_from_fhir, "HealthcareService");
    check_mapping!(locations_from_fhir, "Location");
    check_mapping!(organizations_from_fhir, "Organization");
    let _ = healthpoint_fhir::next_link(&value);
    let _ = healthpoint_fhir::total(&value);
}
