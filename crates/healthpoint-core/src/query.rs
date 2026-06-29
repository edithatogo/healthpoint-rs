//! Query model independent of any specific HTTP/FHIR encoding.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::records::Code;

/// Latitude/longitude point for nearby search.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GeoPoint {
    /// Latitude.
    pub lat: f64,
    /// Longitude.
    pub lon: f64,
}

/// Query limit wrapper to avoid accidental huge exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct QueryLimit(pub u16);

impl Default for QueryLimit {
    fn default() -> Self {
        Self(25)
    }
}

impl QueryLimit {
    /// Clamp to a conservative maximum.
    pub fn clamped(self) -> u16 {
        self.0.clamp(1, 100)
    }
}

/// Search query for `HealthcareService`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ServiceQuery {
    /// Full-text search term.
    pub text: Option<String>,
    /// FHIR category codes.
    pub categories: Vec<Code>,
    /// FHIR type codes.
    pub service_types: Vec<Code>,
    /// FHIR specialty codes.
    pub specialties: Vec<Code>,
    /// Nearby point.
    pub nearby: Option<GeoPoint>,
    /// Nearby radius in kilometres, if supported by the upstream API.
    pub radius_km: Option<f32>,
    /// Result limit.
    pub limit: QueryLimit,
    /// Cursor/token for the next page.
    pub cursor: Option<String>,
}

impl Default for ServiceQuery {
    fn default() -> Self {
        Self {
            text: None,
            categories: Vec::new(),
            service_types: Vec::new(),
            specialties: Vec::new(),
            nearby: None,
            radius_km: None,
            limit: QueryLimit::default(),
            cursor: None,
        }
    }
}
