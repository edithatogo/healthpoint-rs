//! Core domain types for Healthpoint tooling.
//!
//! This crate deliberately has no HTTP client and no MCP dependency. It is the stable seam
//! for CLI, MCP, export, and future open_social_data adapters.

#![forbid(unsafe_code)]

pub mod access;
pub mod error;
pub mod page;
pub mod provenance;
pub mod query;
pub mod records;
pub mod redaction;
pub mod resource_uri;
pub mod traits;
pub mod validation;

pub use access::{AccessMode, AccessPolicy, ExportPolicy, RedistributionStatus};
pub use error::{HealthpointError, Result};
pub use page::{Page, ResponseMetadata};
pub use provenance::SourceProvenance;
pub use query::{GeoPoint, QueryLimit, ServiceQuery};
pub use records::{
    Address, AvailableTime, Code, ContactPoint, Eligibility, GeoPosition, Identifier,
    LocationRecord, NotAvailable, OrganizationRecord, Period, ResourceReference, ServiceRecord,
};
pub use resource_uri::HealthpointResourceUri;
pub use traits::DirectoryProvider;
pub use validation::{
    validate_geo_point, validate_radius_km, validate_resource_id, validate_service_query,
};
