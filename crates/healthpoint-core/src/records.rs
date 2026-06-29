//! Typed domain records extracted from FHIR resources.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::SourceProvenance;

/// Coding system + code + display triple, usually SNOMED CT for clinical concepts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Code {
    /// Coding system URI.
    pub system: Option<String>,
    /// Code value.
    pub code: String,
    /// Display text.
    pub display: Option<String>,
}

impl Code {
    /// Create a code with no explicit system.
    pub fn bare(code: impl Into<String>) -> Self {
        Self {
            system: None,
            code: code.into(),
            display: None,
        }
    }

    /// Create a SNOMED CT token.
    pub fn snomed(code: impl Into<String>) -> Self {
        Self {
            system: Some("http://snomed.info/sct".into()),
            code: code.into(),
            display: None,
        }
    }

    /// Parse either `code` or `system|code` syntax.
    pub fn from_token(raw: &str) -> Self {
        if let Some((system, code)) = raw.split_once('|') {
            Self {
                system: Some(system.to_owned()),
                code: code.to_owned(),
                display: None,
            }
        } else {
            Self::bare(raw)
        }
    }

    /// Return a FHIR token search representation.
    pub fn as_token(&self) -> String {
        match &self.system {
            Some(system) if !system.is_empty() => format!("{system}|{}", self.code),
            _ => self.code.clone(),
        }
    }
}

/// Reference to another FHIR resource.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ResourceReference {
    /// Raw FHIR reference, such as `Location/123`.
    pub reference: String,
    /// Optional display name.
    pub display: Option<String>,
}

/// Contact point for a service/organisation/location.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ContactPoint {
    /// phone | email | url | fax | sms | other.
    pub system: Option<String>,
    /// Contact value.
    pub value: Option<String>,
    /// work | home | temp | old | mobile.
    pub use_code: Option<String>,
}

/// FHIR address projected into a stable directory record shape.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Address {
    /// Human-readable full address.
    pub text: Option<String>,
    /// Street/address lines.
    pub line: Vec<String>,
    /// City/suburb/locality.
    pub city: Option<String>,
    /// District/territorial authority where supplied.
    pub district: Option<String>,
    /// Region/state where supplied.
    pub state: Option<String>,
    /// Postal code.
    pub postal_code: Option<String>,
    /// Country.
    pub country: Option<String>,
}

/// Latitude/longitude coordinates from FHIR `Location.position`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GeoPosition {
    /// Longitude.
    pub longitude: f64,
    /// Latitude.
    pub latitude: f64,
    /// Optional altitude.
    pub altitude: Option<f64>,
}

/// Directory service record derived from FHIR `HealthcareService`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ServiceRecord {
    /// Resource id.
    pub id: String,
    /// Human-readable name.
    pub name: Option<String>,
    /// Whether the service is active.
    pub active: Option<bool>,
    /// Service provider organisation.
    pub provided_by: Option<ResourceReference>,
    /// Locations where the service is provided.
    pub locations: Vec<ResourceReference>,
    /// Coverage areas where the service is intended/available.
    pub coverage_areas: Vec<ResourceReference>,
    /// FHIR `category` codings.
    pub categories: Vec<Code>,
    /// FHIR `type` codings.
    pub service_types: Vec<Code>,
    /// FHIR `specialty` codings.
    pub specialties: Vec<Code>,
    /// FHIR `program` codings.
    pub programs: Vec<Code>,
    /// FHIR `communication` codings.
    pub communications: Vec<Code>,
    /// FHIR `referralMethod` codings.
    pub referral_methods: Vec<Code>,
    /// Whether an appointment is required, when supplied.
    pub appointment_required: Option<bool>,
    /// FHIR `telecom` contacts.
    pub contacts: Vec<ContactPoint>,
    /// Source provenance.
    pub provenance: SourceProvenance,
    /// Raw FHIR resource for fields not yet mapped.
    pub raw_fhir: Value,
}

/// Organisation record derived from FHIR `Organization`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OrganizationRecord {
    /// Resource id.
    pub id: String,
    /// Organisation name.
    pub name: Option<String>,
    /// Whether the organisation is active.
    pub active: Option<bool>,
    /// Contact points.
    pub contacts: Vec<ContactPoint>,
    /// Source provenance.
    pub provenance: SourceProvenance,
    /// Raw FHIR resource.
    pub raw_fhir: Value,
}

/// Location record derived from FHIR `Location`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LocationRecord {
    /// Resource id.
    pub id: String,
    /// Location name.
    pub name: Option<String>,
    /// active | suspended | inactive.
    pub status: Option<String>,
    /// instance | kind.
    pub mode: Option<String>,
    /// Location type codings.
    pub location_types: Vec<Code>,
    /// Contact points.
    pub contacts: Vec<ContactPoint>,
    /// Address.
    pub address: Option<Address>,
    /// Geospatial position.
    pub position: Option<GeoPosition>,
    /// Managing organisation.
    pub managing_organization: Option<ResourceReference>,
    /// Source provenance.
    pub provenance: SourceProvenance,
    /// Raw FHIR resource.
    pub raw_fhir: Value,
}
