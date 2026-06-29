//! Provider traits for directory-like Healthpoint access.

use async_trait::async_trait;

use crate::{OrganizationRecord, Page, Result, ServiceQuery, ServiceRecord};

/// Read-only directory provider.
#[async_trait]
pub trait DirectoryProvider: Send + Sync {
    /// Search healthcare services.
    async fn search_services(&self, query: ServiceQuery) -> Result<Page<ServiceRecord>>;

    /// Get a single healthcare service.
    async fn get_service(&self, id: &str) -> Result<ServiceRecord>;

    /// Get a single organisation.
    async fn get_organization(&self, id: &str) -> Result<OrganizationRecord>;
}
