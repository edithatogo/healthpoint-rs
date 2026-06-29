//! Query model independent of any specific HTTP/FHIR encoding.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Result, records::Code};

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
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct ServiceQuery {
    /// Full-text search term.
    pub text: Option<String>,
    /// FHIR category codes.
    pub categories: Vec<Code>,
    /// FHIR type codes.
    pub service_types: Vec<Code>,
    /// FHIR specialty codes.
    pub specialties: Vec<Code>,
    /// Healthpoint branch code, encoded as `branch-code`.
    pub branch_code: Option<String>,
    /// Healthpoint region parameter.
    pub region: Option<String>,
    /// Healthpoint DHB region parameter, encoded as `dhb-region`.
    pub dhb_region: Option<String>,
    /// Healthpoint subregion parameter.
    pub subregion: Option<String>,
    /// Nearby point.
    pub nearby: Option<GeoPoint>,
    /// Nearby radius in kilometres, if supported by the upstream API.
    pub radius_km: Option<f32>,
    /// Result limit.
    pub limit: QueryLimit,
    /// Cursor/token for the next page.
    pub cursor: Option<String>,
}

impl ServiceQuery {
    /// Create an empty service query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a text search term.
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Add a service type code.
    pub fn with_service_type(mut self, code: Code) -> Self {
        self.service_types.push(code);
        self
    }

    /// Add a SNOMED CT service type code.
    pub fn with_snomed_type(self, code: impl Into<String>) -> Self {
        self.with_service_type(Code::snomed(code))
    }

    /// Set Healthpoint branch and region parameters.
    pub fn with_branch_region(
        mut self,
        branch_code: impl Into<String>,
        region: impl Into<String>,
    ) -> Self {
        self.branch_code = Some(branch_code.into());
        self.region = Some(region.into());
        self
    }

    /// Set nearby latitude/longitude and optional radius.
    pub fn with_nearby(mut self, point: GeoPoint, radius_km: Option<f32>) -> Self {
        self.nearby = Some(point);
        self.radius_km = radius_km;
        self
    }

    /// Validate the query according to conservative client limits.
    pub fn validate(&self) -> Result<()> {
        crate::validation::validate_service_query(self)
    }
}
