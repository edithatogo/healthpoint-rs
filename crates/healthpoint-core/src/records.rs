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
    /// FHIR `category` codings.
    pub categories: Vec<Code>,
    /// FHIR `type` codings.
    pub service_types: Vec<Code>,
    /// FHIR `specialty` codings.
    pub specialties: Vec<Code>,
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
