//! Read-only MCP server for Healthpoint.

use healthpoint_client::HealthpointClient;
use healthpoint_core::{
    AccessPolicy, Code, DirectoryProvider, GeoPoint, HealthpointResourceUri, QueryLimit,
    ServiceQuery,
};
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler, ServiceExt,
    handler::server::wrapper::Parameters,
    model::{
        GetPromptResult, ListResourceTemplatesResult, ListResourcesResult, PaginatedRequestParams,
        PromptMessage, ReadResourceRequestParams, ReadResourceResult, Resource, ResourceContents,
        ResourceTemplate, Role, ServerCapabilities, ServerInfo,
    },
    prompt, prompt_handler, prompt_router,
    schemars::JsonSchema,
    service::RequestContext,
    tool, tool_handler, tool_router,
    transport::stdio,
};
use serde::Deserialize;
use url::Url;

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
    /// Healthpoint branch code, e.g. primary.
    branch_code: Option<String>,
    /// Healthpoint region, e.g. Southland.
    region: Option<String>,
    /// Healthpoint DHB region, e.g. Southern.
    dhb_region: Option<String>,
    /// Healthpoint subregion, e.g. Ashburton.
    subregion: Option<String>,
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

#[derive(Debug, Deserialize, JsonSchema)]
struct SearchPromptParams {
    /// Plain-language service need, e.g. immunisation, GP, pharmacy.
    need: Option<String>,
    /// Region filter, e.g. Southland.
    region: Option<String>,
    /// Branch code filter, e.g. primary.
    branch_code: Option<String>,
}

#[tool_router]
impl HealthpointMcpServer {
    #[tool(
        description = "Show redacted Healthpoint client configuration and readiness. Never returns the API key."
    )]
    fn healthpoint_diagnostic_status(&self) -> String {
        serde_json::to_string_pretty(&self.client.diagnostic_status())
            .unwrap_or_else(|err| err.to_string())
    }

    #[tool(
        description = "Show Healthpoint API access notes discovered from the portal. Does not include secrets."
    )]
    fn healthpoint_api_access_notes(&self) -> String {
        serde_json::json!({
            "base_url": "https://uat.healthpointapi.com/baseR4/",
            "auth_header": "x-api-key",
            "resources": ["HealthcareService", "Location", "Practitioner", "PractitionerRole"],
            "methods": ["GET"],
            "attribution_required": true,
            "docs": [
                "docs/healthpoint-api-access.md",
                "docs/healthpoint-license-notes.md",
                "docs/live-validation.md"
            ],
            "secret_handling": "API keys must be supplied through environment variables and never returned by tools."
        })
        .to_string()
    }

    #[tool(
        description = "Show conservative Healthpoint access/export policy. Use before exporting or reusing data."
    )]
    fn healthpoint_access_policy(&self) -> String {
        serde_json::to_string_pretty(&AccessPolicy::default()).unwrap_or_else(|err| err.to_string())
    }

    #[tool(
        description = "Search Healthpoint HealthcareService records. Read-only; requires a user-provided API key."
    )]
    async fn healthpoint_search_services(
        &self,
        Parameters(params): Parameters<SearchServicesParams>,
    ) -> String {
        let mut service_types = params
            .service_type
            .iter()
            .map(|raw| Code::from_token(raw))
            .collect::<Vec<_>>();
        service_types.extend(params.snomed.iter().map(|raw| Code::snomed(raw.clone())));
        let query = ServiceQuery {
            text: params.text,
            categories: params
                .category
                .iter()
                .map(|raw| Code::from_token(raw))
                .collect(),
            service_types,
            specialties: params
                .specialty
                .iter()
                .map(|raw| Code::from_token(raw))
                .collect(),
            branch_code: params.branch_code,
            region: params.region,
            dhb_region: params.dhb_region,
            subregion: params.subregion,
            limit: QueryLimit(params.limit.unwrap_or(10)),
            cursor: params.cursor,
            ..ServiceQuery::default()
        };
        json_result(self.client.search_services(query).await)
    }

    #[tool(
        description = "Search Healthpoint HealthcareService records by SNOMED CT code. Read-only."
    )]
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

    #[tool(
        description = "Find nearby Healthpoint HealthcareService records by latitude/longitude. Read-only."
    )]
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

    #[tool(
        description = "Get a single Healthpoint HealthcareService record by FHIR id. Read-only."
    )]
    async fn healthpoint_get_service(
        &self,
        Parameters(params): Parameters<GetResourceParams>,
    ) -> String {
        json_result(self.client.get_service(&params.id).await)
    }

    #[tool(description = "Get a single Healthpoint Location record by FHIR id. Read-only.")]
    async fn healthpoint_get_location(
        &self,
        Parameters(params): Parameters<GetResourceParams>,
    ) -> String {
        json_result(self.client.get_location(&params.id).await)
    }

    #[tool(description = "Get a single Healthpoint Organization record by FHIR id. Read-only.")]
    async fn healthpoint_get_organization(
        &self,
        Parameters(params): Parameters<GetResourceParams>,
    ) -> String {
        json_result(self.client.get_organization(&params.id).await)
    }

    #[tool(
        description = "Read a supported healthpoint:// resource URI. This mirrors planned MCP resources while keeping the operation explicit and read-only."
    )]
    async fn healthpoint_read_resource_uri(
        &self,
        Parameters(params): Parameters<ReadResourceUriParams>,
    ) -> String {
        match HealthpointResourceUri::parse(&params.uri) {
            Ok(HealthpointResourceUri::Service(id)) => {
                json_result(self.client.get_service(&id).await)
            }
            Ok(HealthpointResourceUri::Location(id)) => {
                json_result(self.client.get_location(&id).await)
            }
            Ok(HealthpointResourceUri::Organization(id)) => {
                json_result(self.client.get_organization(&id).await)
            }
            Err(err) => serde_json::json!({ "error": err.to_string() }).to_string(),
        }
    }
}

#[prompt_router]
impl HealthpointMcpServer {
    #[prompt(
        name = "healthpoint_safe_search",
        description = "Draft a safe Healthpoint service-search request using licensed, read-only access."
    )]
    async fn healthpoint_safe_search(
        &self,
        Parameters(params): Parameters<SearchPromptParams>,
    ) -> GetPromptResult {
        let need = params
            .need
            .unwrap_or_else(|| "the user's service need".into());
        let mut filters = Vec::new();
        if let Some(region) = params.region {
            filters.push(format!("region={region}"));
        }
        if let Some(branch_code) = params.branch_code {
            filters.push(format!("branch-code={branch_code}"));
        }
        let filter_text = if filters.is_empty() {
            "Use only filters the user supplied.".to_owned()
        } else {
            format!("Apply these filters: {}.", filters.join(", "))
        };
        GetPromptResult::new(vec![PromptMessage::new_text(
            Role::User,
            format!(
                "Search Healthpoint for {need}. {filter_text} Return concise directory results with provenance, attribution, and no clinical advice. Do not expose API keys or cache/publicly redistribute returned data."
            ),
        )])
        .with_description("Safe read-only Healthpoint service search")
    }

    #[prompt(
        name = "healthpoint_license_check",
        description = "Prepare a license/export safety check before using Healthpoint data."
    )]
    async fn healthpoint_license_check(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            "Before using Healthpoint data, check the access policy resource and confirm the use is local-only, attributed, read-only, non-production unless approved in writing, and not a bulk dump, public cache, resale, redistribution, competing directory, or AI-training dataset.",
        )]
    }
}

#[tool_handler]
#[prompt_handler]
impl ServerHandler for HealthpointMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_prompts()
                .enable_resources()
                .build(),
        )
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        Ok(ListResourcesResult {
            resources: vec![
                Resource::new("healthpoint://diagnostic/status", "Diagnostic status")
                    .with_description("Redacted runtime client configuration")
                    .with_mime_type("application/json"),
                Resource::new("healthpoint://api/access-notes", "API access notes")
                    .with_description("Non-secret Healthpoint API endpoint and access notes")
                    .with_mime_type("application/json"),
                Resource::new("healthpoint://access/policy", "Access policy")
                    .with_description("Conservative Healthpoint access and export policy")
                    .with_mime_type("application/json"),
            ],
            next_cursor: None,
            meta: None,
        })
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        Ok(ListResourceTemplatesResult {
            resource_templates: vec![
                ResourceTemplate::new("healthpoint://service/{id}", "HealthcareService")
                    .with_description("Read one Healthpoint HealthcareService by FHIR id")
                    .with_mime_type("application/json"),
                ResourceTemplate::new("healthpoint://location/{id}", "Location")
                    .with_description("Read one Healthpoint Location by FHIR id")
                    .with_mime_type("application/json"),
                ResourceTemplate::new("healthpoint://organization/{id}", "Organization")
                    .with_description("Read one Healthpoint Organization by FHIR id")
                    .with_mime_type("application/json"),
                ResourceTemplate::new(
                    "healthpoint://query/services?text={text}&region={region}&limit={limit}",
                    "Service search query",
                )
                .with_description("Search Healthpoint HealthcareService records")
                .with_mime_type("application/json"),
            ],
            next_cursor: None,
            meta: None,
        })
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        let uri = request.uri;
        let body = match uri.as_str() {
            "healthpoint://diagnostic/status" => {
                serde_json::to_string_pretty(&self.client.diagnostic_status())
                    .unwrap_or_else(|err| err.to_string())
            }
            "healthpoint://api/access-notes" => self.healthpoint_api_access_notes(),
            "healthpoint://access/policy" => self.healthpoint_access_policy(),
            _ if uri.starts_with("healthpoint://query/services?") => {
                match service_query_from_resource_uri(&uri) {
                    Ok(query) => json_result(self.client.search_services(query).await),
                    Err(err) => {
                        return Err(McpError::invalid_params(
                            err.to_string(),
                            Some(serde_json::json!({ "uri": uri })),
                        ));
                    }
                }
            }
            _ => match HealthpointResourceUri::parse(&uri) {
                Ok(HealthpointResourceUri::Service(id)) => {
                    json_result(self.client.get_service(&id).await)
                }
                Ok(HealthpointResourceUri::Location(id)) => {
                    json_result(self.client.get_location(&id).await)
                }
                Ok(HealthpointResourceUri::Organization(id)) => {
                    json_result(self.client.get_organization(&id).await)
                }
                Err(err) => {
                    return Err(McpError::resource_not_found(
                        "resource_not_found",
                        Some(serde_json::json!({ "uri": uri, "error": err.to_string() })),
                    ));
                }
            },
        };
        Ok(ReadResourceResult::new(vec![
            ResourceContents::text(body, uri).with_mime_type("application/json"),
        ]))
    }
}

fn service_query_from_resource_uri(uri: &str) -> healthpoint_core::Result<ServiceQuery> {
    let parsed = Url::parse(uri).map_err(|err| {
        healthpoint_core::HealthpointError::InvalidInput(format!(
            "invalid Healthpoint query resource URI: {err}"
        ))
    })?;
    if parsed.scheme() != "healthpoint" || parsed.host_str() != Some("query") {
        return Err(healthpoint_core::HealthpointError::InvalidInput(
            "expected healthpoint://query/services?...".into(),
        ));
    }
    if parsed.path() != "/services" {
        return Err(healthpoint_core::HealthpointError::InvalidInput(
            "only healthpoint://query/services is supported".into(),
        ));
    }
    let mut query = ServiceQuery::default();
    for (name, value) in parsed.query_pairs() {
        match name.as_ref() {
            "text" if !value.is_empty() => query.text = Some(value.into_owned()),
            "region" if !value.is_empty() => query.region = Some(value.into_owned()),
            "branch-code" | "branch_code" if !value.is_empty() => {
                query.branch_code = Some(value.into_owned());
            }
            "type" | "service-type" | "service_type" if !value.is_empty() => {
                query.service_types.push(Code::from_token(&value));
            }
            "category" if !value.is_empty() => query.categories.push(Code::from_token(&value)),
            "specialty" if !value.is_empty() => {
                query.specialties.push(Code::from_token(&value));
            }
            "limit" if !value.is_empty() => {
                let limit = value.parse::<u16>().map_err(|err| {
                    healthpoint_core::HealthpointError::InvalidInput(format!(
                        "invalid limit in resource URI: {err}"
                    ))
                })?;
                query.limit = QueryLimit(limit);
            }
            _ => {}
        }
    }
    Ok(query)
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
