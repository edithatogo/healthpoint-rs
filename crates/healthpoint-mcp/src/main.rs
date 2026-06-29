//! Read-only MCP server for Healthpoint.

use healthpoint_client::HealthpointClient;
use healthpoint_core::{
    Code, DirectoryProvider, GeoPoint, HealthpointResourceUri, QueryLimit, ServiceQuery,
};
use rmcp::{
    handler::server::wrapper::Parameters, schemars::JsonSchema, tool, tool_router,
    transport::stdio, ServiceExt,
};
use serde::Deserialize;

#[derive(Clone)]
struct HealthpointMcpServer {
    client: HealthpointClient,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct SearchServicesParams {
    /// Full-text search term.
    text: Option<String>,
    /// FHIR category code or system|code token. Repeatable.
    #[serde(default)]
    category: Vec<String>,
    /// FHIR service type code or system|code token. Repeatable.
    #[serde(default)]
    service_type: Vec<String>,
    /// Convenience SNOMED CT service type codes. Repeatable.
    #[serde(default)]
    snomed: Vec<String>,
    /// FHIR specialty code or system|code token. Repeatable.
    #[serde(default)]
    specialty: Vec<String>,
    /// Pagination cursor from a previous response.
    cursor: Option<String>,
    /// Maximum results. Clamped to 1..100.
    limit: Option<u16>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct NearbyServicesParams {
    /// Latitude.
    lat: f64,
    /// Longitude.
    lon: f64,
    /// Optional radius in kilometres.
    radius_km: Option<f32>,
    /// Optional full-text search term.
    text: Option<String>,
    /// Optional service type code or system|code token.
    service_type: Option<String>,
    /// Maximum results. Clamped to 1..100.
    limit: Option<u16>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct SnomedSearchParams {
    /// SNOMED CT code.
    code: String,
    /// Search field: type, category, or specialty. Defaults to type.
    field: Option<SnomedField>,
    /// Maximum results. Clamped to 1..100.
    limit: Option<u16>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
enum SnomedField {
    /// Search HealthcareService.type.
    Type,
    /// Search HealthcareService.category.
    Category,
    /// Search HealthcareService.specialty.
    Specialty,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct GetResourceParams {
    /// FHIR resource id.
    id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ReadResourceUriParams {
    /// Resource URI such as healthpoint://service/<id>.
    uri: String,
}

#[tool_router(server_handler)]
impl HealthpointMcpServer {
    #[tool(description = "Show redacted Healthpoint client configuration and readiness. Never returns the API key.")]
    fn healthpoint_diagnostic_status(&self) -> String {
        serde_json::to_string_pretty(&self.client.diagnostic_status())
            .unwrap_or_else(|err| err.to_string())
    }

    #[tool(description = "Search Healthpoint HealthcareService records. Read-only; requires a user-provided API key.")]
    async fn healthpoint_search_services(
        &self,
        Parameters(params): Parameters<SearchServicesParams>,
    ) -> String {
        let mut service_types = params.service_type.iter().map(|raw| Code::from_token(raw)).collect::<Vec<_>>();
        service_types.extend(params.snomed.iter().map(|raw| Code::snomed(raw.clone())));
        let query = ServiceQuery {
            text: params.text,
            categories: params.category.iter().map(|raw| Code::from_token(raw)).collect(),
            service_types,
            specialties: params.specialty.iter().map(|raw| Code::from_token(raw)).collect(),
            limit: QueryLimit(params.limit.unwrap_or(10)),
            cursor: params.cursor,
            ..ServiceQuery::default()
        };
        json_result(self.client.search_services(query).await)
    }

    #[tool(description = "Search Healthpoint HealthcareService records by SNOMED CT code. Read-only.")]
    async fn healthpoint_search_by_snomed(
        &self,
        Parameters(params): Parameters<SnomedSearchParams>,
    ) -> String {
        let code = Code::snomed(params.code);
        let mut query = ServiceQuery {
            limit: QueryLimit(params.limit.unwrap_or(10)),
            ..ServiceQuery::default()
        };
        match params.field.unwrap_or(SnomedField::Type) {
            SnomedField::Type => query.service_types.push(code),
            SnomedField::Category => query.categories.push(code),
            SnomedField::Specialty => query.specialties.push(code),
        }
        json_result(self.client.search_services(query).await)
    }

    #[tool(description = "Find nearby Healthpoint HealthcareService records by latitude/longitude. Read-only.")]
    async fn healthpoint_find_nearby_services(
        &self,
        Parameters(params): Parameters<NearbyServicesParams>,
    ) -> String {
        let query = ServiceQuery {
            text: params.text,
            service_types: params
                .service_type
                .as_ref()
                .map(|raw| Code::from_token(raw))
                .into_iter()
                .collect(),
            nearby: Some(GeoPoint {
                lat: params.lat,
                lon: params.lon,
            }),
            radius_km: params.radius_km,
            limit: QueryLimit(params.limit.unwrap_or(10)),
            ..ServiceQuery::default()
        };
        json_result(self.client.search_services(query).await)
    }

    #[tool(description = "Get a single Healthpoint HealthcareService record by FHIR id. Read-only.")]
    async fn healthpoint_get_service(&self, Parameters(params): Parameters<GetResourceParams>) -> String {
        json_result(self.client.get_service(&params.id).await)
    }

    #[tool(description = "Get a single Healthpoint Location record by FHIR id. Read-only.")]
    async fn healthpoint_get_location(&self, Parameters(params): Parameters<GetResourceParams>) -> String {
        json_result(self.client.get_location(&params.id).await)
    }

    #[tool(description = "Get a single Healthpoint Organization record by FHIR id. Read-only.")]
    async fn healthpoint_get_organization(
        &self,
        Parameters(params): Parameters<GetResourceParams>,
    ) -> String {
        json_result(self.client.get_organization(&params.id).await)
    }

    #[tool(description = "Read a supported healthpoint:// resource URI. This mirrors planned MCP resources while keeping the operation explicit and read-only.")]
    async fn healthpoint_read_resource_uri(
        &self,
        Parameters(params): Parameters<ReadResourceUriParams>,
    ) -> String {
        match HealthpointResourceUri::parse(&params.uri) {
            Ok(HealthpointResourceUri::Service(id)) => json_result(self.client.get_service(&id).await),
            Ok(HealthpointResourceUri::Location(id)) => json_result(self.client.get_location(&id).await),
            Ok(HealthpointResourceUri::Organization(id)) => {
                json_result(self.client.get_organization(&id).await)
            }
            Err(err) => serde_json::json!({ "error": err.to_string() }).to_string(),
        }
    }
}

fn json_result<T: serde::Serialize>(result: healthpoint_core::Result<T>) -> String {
    match result {
        Ok(value) => serde_json::to_string_pretty(&value).unwrap_or_else(|err| err.to_string()),
        Err(err) => serde_json::json!({ "error": err.to_string() }).to_string(),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();
    let service = HealthpointMcpServer {
        client: HealthpointClient::from_env()?,
    }
    .serve(stdio())
    .await?;
    service.waiting().await?;
    Ok(())
}
