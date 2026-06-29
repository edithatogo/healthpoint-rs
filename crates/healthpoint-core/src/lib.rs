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
pub mod traits;

pub use access::{AccessMode, AccessPolicy, ExportPolicy, RedistributionStatus};
pub use error::{HealthpointError, Result};
pub use page::Page;
pub use provenance::SourceProvenance;
pub use query::{GeoPoint, QueryLimit, ServiceQuery};
pub use records::{Code, ContactPoint, OrganizationRecord, ResourceReference, ServiceRecord};
pub use traits::DirectoryProvider;
