//! Read-only MCP server for Healthpoint.

use healthpoint_client::HealthpointClient;
use healthpoint_core::{Code, DirectoryProvider, QueryLimit, ServiceQuery};
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
    /// Service type code or system|code token.
    service_type: Option<String>,
    /// Maximum results. Clamped to 1..100.
    limit: Option<u16>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct GetResourceParams {
    /// FHIR resource id.
    id: String,
}

#[tool_router(server_handler)]
impl HealthpointMcpServer {
    #[tool(description = "Search Healthpoint HealthcareService records. Read-only; requires a user-provided API key.")]
    async fn healthpoint_search_services(
        &self,
        Parameters(params): Parameters<SearchServicesParams>,
    ) -> String {
        let query = ServiceQuery {
            text: params.text,
            service_types: params
                .service_type
                .as_ref()
                .map(parse_code)
                .into_iter()
                .collect(),
            limit: QueryLimit(params.limit.unwrap_or(10)),
            ..ServiceQuery::default()
        };
        match self.client.search_services(query).await {
            Ok(page) => serde_json::to_string_pretty(&page).unwrap_or_else(|err| err.to_string()),
            Err(err) => serde_json::json!({ "error": err.to_string() }).to_string(),
        }
    }

    #[tool(description = "Get a single Healthpoint HealthcareService record by FHIR id. Read-only.")]
    async fn healthpoint_get_service(&self, Parameters(params): Parameters<GetResourceParams>) -> String {
        match self.client.get_service(&params.id).await {
            Ok(service) => serde_json::to_string_pretty(&service).unwrap_or_else(|err| err.to_string()),
            Err(err) => serde_json::json!({ "error": err.to_string() }).to_string(),
        }
    }

    #[tool(description = "Get a single Healthpoint Organization record by FHIR id. Read-only.")]
    async fn healthpoint_get_organization(
        &self,
        Parameters(params): Parameters<GetResourceParams>,
    ) -> String {
        match self.client.get_organization(&params.id).await {
            Ok(org) => serde_json::to_string_pretty(&org).unwrap_or_else(|err| err.to_string()),
            Err(err) => serde_json::json!({ "error": err.to_string() }).to_string(),
        }
    }
}

fn parse_code(raw: &String) -> Code {
    if let Some((system, code)) = raw.split_once('|') {
        Code {
            system: Some(system.to_owned()),
            code: code.to_owned(),
            display: None,
        }
    } else {
        Code::bare(raw)
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
