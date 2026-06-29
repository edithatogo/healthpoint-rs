//! Synthetic Healthpoint/FHIR fixtures and mock providers for tests.

#![forbid(unsafe_code)]

use async_trait::async_trait;
use healthpoint_core::{
    DirectoryProvider, HealthpointError, LocationRecord, OrganizationRecord, Page, Result,
    ServiceQuery, ServiceRecord, SourceProvenance,
};
use serde_json::Value;

/// Synthetic HealthcareService search bundle.
pub fn healthcare_service_bundle() -> Value {
    serde_json::from_str(include_str!("../fixtures/fhir-bundle-healthcare-service.json"))
        .expect("synthetic fixture is valid JSON")
}

/// Synthetic Location resource.
pub fn location() -> Value {
    serde_json::from_str(include_str!("../fixtures/fhir-location.json"))
        .expect("synthetic fixture is valid JSON")
}

/// Synthetic Organization resource.
pub fn organization() -> Value {
    serde_json::from_str(include_str!("../fixtures/fhir-organization.json"))
        .expect("synthetic fixture is valid JSON")
}

/// Synthetic, in-memory provider for offline CLI/MCP/export tests.
#[derive(Debug, Clone, Default)]
pub struct FixtureDirectoryProvider;

impl FixtureDirectoryProvider {
    /// Create a new fixture provider.
    pub fn new() -> Self {
        Self
    }

    fn provenance(&self) -> SourceProvenance {
        SourceProvenance::healthpoint("fixture://healthpoint-testkit")
    }

    fn services(&self) -> Result<Vec<ServiceRecord>> {
        healthpoint_fhir::services_from_fhir(healthcare_service_bundle(), self.provenance())
    }

    fn location_record(&self) -> Result<LocationRecord> {
        healthpoint_fhir::location_from_fhir(location(), self.provenance())
    }

    fn organization_record(&self) -> Result<OrganizationRecord> {
        healthpoint_fhir::organization_from_fhir(organization(), self.provenance())
    }
}

#[async_trait]
impl DirectoryProvider for FixtureDirectoryProvider {
    async fn search_services(&self, query: ServiceQuery) -> Result<Page<ServiceRecord>> {
        query.validate()?;
        let mut items = self.services()?;
        if let Some(text) = query.text.as_deref() {
            let needle = text.to_ascii_lowercase();
            items.retain(|service| {
                service
                    .name
                    .as_deref()
                    .unwrap_or_default()
                    .to_ascii_lowercase()
                    .contains(&needle)
                    || service
                        .comment
                        .as_deref()
                        .unwrap_or_default()
                        .to_ascii_lowercase()
                        .contains(&needle)
            });
        }
        if !query.service_types.is_empty() {
            items.retain(|service| {
                query.service_types.iter().any(|wanted| {
                    service
                        .service_types
                        .iter()
                        .any(|actual| actual.as_token() == wanted.as_token() || actual.code == wanted.code)
                })
            });
        }
        let limit = usize::from(query.limit.clamped());
        items.truncate(limit);
        Ok(Page::new(items, self.provenance()))
    }

    async fn get_service(&self, id: &str) -> Result<ServiceRecord> {
        self.services()?
            .into_iter()
            .find(|service| service.id == id)
            .ok_or_else(|| HealthpointError::Parse(format!("fixture service {id:?} not found")))
    }

    async fn get_organization(&self, id: &str) -> Result<OrganizationRecord> {
        let organization = self.organization_record()?;
        if organization.id == id {
            Ok(organization)
        } else {
            Err(HealthpointError::Parse(format!(
                "fixture organization {id:?} not found"
            )))
        }
    }

    async fn get_location(&self, id: &str) -> Result<LocationRecord> {
        let location = self.location_record()?;
        if location.id == id {
            Ok(location)
        } else {
            Err(HealthpointError::Parse(format!("fixture location {id:?} not found")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use healthpoint_core::{Code, QueryLimit};

    #[tokio::test]
    async fn fixture_provider_supports_snomed_filter() {
        let provider = FixtureDirectoryProvider::new();
        let page = provider
            .search_services(ServiceQuery {
                service_types: vec![Code::snomed("171149006")],
                limit: QueryLimit(10),
                ..ServiceQuery::default()
            })
            .await
            .expect("search works");
        assert_eq!(page.items.len(), 1);
    }
}
