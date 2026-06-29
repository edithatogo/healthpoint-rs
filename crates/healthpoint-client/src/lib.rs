//! Async Healthpoint API client.
//!
//! Authentication, base URL, and geography parameter strategy are configurable because the public
//! landing page does not publish a complete developer reference. The code defaults to safe,
//! read-only FHIR search semantics.

#![forbid(unsafe_code)]

use std::time::Duration;

use async_trait::async_trait;
use healthpoint_core::{
    DirectoryProvider, HealthpointError, LocationRecord, OrganizationRecord, Page, Result,
    ServiceQuery, ServiceRecord, SourceProvenance,
};
use reqwest::{header, RequestBuilder, Url};
use serde_json::Value;

/// Authentication strategy for Healthpoint API requests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthScheme {
    /// Do not send auth headers. Mostly useful for mocks.
    None,
    /// `Authorization: Bearer <token>`.
    Bearer,
    /// Custom header, e.g. `x-api-key: <token>`.
    Header(String),
}

/// Experimental nearby-search parameter encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeoSearchMode {
    /// Healthpoint-style custom `latitude`, `longitude`, `radius_km` query parameters.
    HealthpointLatLon,
    /// FHIR-ish `near=lat|lon|radius` parameter.
    FhirNear,
}

/// Client configuration.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// API base URL.
    pub base_url: Url,
    /// API key/token.
    pub api_key: Option<String>,
    /// Authentication scheme.
    pub auth_scheme: AuthScheme,
    /// Nearby search encoding.
    pub geo_search_mode: GeoSearchMode,
    /// Per-request timeout.
    pub timeout: Duration,
}

impl ClientConfig {
    /// Create a config from individual parts.
    pub fn new(base_url: Url, api_key: Option<String>, auth_scheme: AuthScheme) -> Self {
        Self {
            base_url,
            api_key,
            auth_scheme,
            geo_search_mode: GeoSearchMode::HealthpointLatLon,
            timeout: Duration::from_secs(30),
        }
    }

    /// Create a config from environment variables.
    pub fn from_env() -> Result<Self> {
        let base_url = std::env::var("HEALTHPOINT_BASE_URL")
            .unwrap_or_else(|_| "https://www.healthpointapi.com/".to_owned());
        let base_url = Url::parse(&base_url).map_err(|err| {
            HealthpointError::Config(format!("HEALTHPOINT_BASE_URL is not a valid URL: {err}"))
        })?;
        let api_key = std::env::var("HEALTHPOINT_API_KEY").ok().filter(|s| !s.is_empty());
        let auth_scheme = parse_auth_scheme(
            &std::env::var("HEALTHPOINT_AUTH_SCHEME").unwrap_or_else(|_| "bearer".into()),
        )?;
        let geo_search_mode = parse_geo_search_mode(
            &std::env::var("HEALTHPOINT_GEO_SEARCH_MODE")
                .unwrap_or_else(|_| "healthpoint-lat-lon".into()),
        )?;
        let timeout = parse_timeout_secs(
            std::env::var("HEALTHPOINT_TIMEOUT_SECS")
                .ok()
                .as_deref()
                .unwrap_or("30"),
        )?;
        Ok(Self {
            base_url,
            api_key,
            auth_scheme,
            geo_search_mode,
            timeout,
        })
    }
}

/// Parse CLI/env auth scheme string.
pub fn parse_auth_scheme(raw: &str) -> Result<AuthScheme> {
    let value = raw.trim();
    let normalized = value.to_ascii_lowercase();
    match normalized.as_str() {
        "none" => Ok(AuthScheme::None),
        "bearer" => Ok(AuthScheme::Bearer),
        "x-api-key" | "api-key" | "apikey" => Ok(AuthScheme::Header("x-api-key".into())),
        _ if normalized.starts_with("header:") => {
            let header_name = value
                .split_once(':')
                .map(|(_, name)| name.trim())
                .filter(|name| !name.is_empty())
                .ok_or_else(|| HealthpointError::Config("header auth scheme needs a name".into()))?;
            Ok(AuthScheme::Header(header_name.to_owned()))
        }
        _ => Err(HealthpointError::Config(format!(
            "unsupported auth scheme {value:?}; use bearer, x-api-key, header:<name>, or none"
        ))),
    }
}

/// Parse nearby search encoding mode.
pub fn parse_geo_search_mode(raw: &str) -> Result<GeoSearchMode> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "healthpoint" | "healthpoint-lat-lon" | "lat-lon" | "latlon" => {
            Ok(GeoSearchMode::HealthpointLatLon)
        }
        "fhir" | "fhir-near" | "near" => Ok(GeoSearchMode::FhirNear),
        other => Err(HealthpointError::Config(format!(
            "unsupported geo search mode {other:?}; use healthpoint-lat-lon or fhir-near"
        ))),
    }
}

fn parse_timeout_secs(raw: &str) -> Result<Duration> {
    let secs: u64 = raw.trim().parse().map_err(|err| {
        HealthpointError::Config(format!("HEALTHPOINT_TIMEOUT_SECS must be an integer: {err}"))
    })?;
    Ok(Duration::from_secs(secs.clamp(1, 300)))
}

/// Async API client.
#[derive(Debug, Clone)]
pub struct HealthpointClient {
    http: reqwest::Client,
    config: ClientConfig,
}

impl HealthpointClient {
    /// Construct a client.
    pub fn new(config: ClientConfig) -> Self {
        let http = reqwest::Client::builder()
            .user_agent(format!("healthpoint-rs/{}", env!("CARGO_PKG_VERSION")))
            .timeout(config.timeout)
            .build()
            .expect("reqwest client builder should not fail with static config");
        Self { http, config }
    }

    /// Construct from environment variables.
    pub fn from_env() -> Result<Self> {
        Ok(Self::new(ClientConfig::from_env()?))
    }

    /// Return redacted runtime status for diagnostics.
    pub fn diagnostic_status(&self) -> serde_json::Value {
        serde_json::json!({
            "base_url": self.config.base_url.as_str(),
            "auth_scheme": match &self.config.auth_scheme {
                AuthScheme::None => "none".to_string(),
                AuthScheme::Bearer => "bearer".to_string(),
                AuthScheme::Header(name) => format!("header:{name}"),
            },
            "api_key_present": self.config.api_key.as_ref().is_some_and(|s| !s.is_empty()),
            "geo_search_mode": match self.config.geo_search_mode {
                GeoSearchMode::HealthpointLatLon => "healthpoint-lat-lon",
                GeoSearchMode::FhirNear => "fhir-near",
            },
            "timeout_secs": self.config.timeout.as_secs(),
        })
    }

    /// Build a HealthcareService search URL without sending it.
    pub fn service_search_url(&self, query: &ServiceQuery) -> Result<Url> {
        if let Some(cursor) = absolute_same_origin_cursor(&self.config.base_url, query.cursor.as_deref())? {
            return Ok(cursor);
        }
        let mut url = self.resource_url("HealthcareService", None)?;
        self.encode_service_query(&mut url, query);
        Ok(url)
    }

    fn provenance(&self) -> SourceProvenance {
        SourceProvenance::healthpoint(self.config.base_url.as_str())
    }

    fn resource_url(&self, resource_type: &str, id: Option<&str>) -> Result<Url> {
        let mut url = self.config.base_url.clone();
        let mut path = url.path().trim_end_matches('/').to_owned();
        if !path.is_empty() {
            path.push('/');
        }
        path.push_str(resource_type.trim_start_matches('/'));
        if let Some(id) = id {
            path.push('/');
            path.push_str(id);
        }
        url.set_path(&path);
        url.set_query(None);
        Ok(url)
    }

    fn apply_auth(&self, request: RequestBuilder) -> Result<RequestBuilder> {
        let request = request.header(
            header::ACCEPT,
            "application/fhir+json, application/json;q=0.9, */*;q=0.1",
        );
        let Some(api_key) = &self.config.api_key else {
            return Ok(request);
        };
        match &self.config.auth_scheme {
            AuthScheme::None => Ok(request),
            AuthScheme::Bearer => Ok(request.bearer_auth(api_key)),
            AuthScheme::Header(name) => {
                let header_name = header::HeaderName::from_bytes(name.as_bytes()).map_err(|err| {
                    HealthpointError::Config(format!("invalid auth header name {name:?}: {err}"))
                })?;
                Ok(request.header(header_name, api_key))
            }
        }
    }

    async fn get_json(&self, url: Url) -> Result<Value> {
        let request = self.apply_auth(self.http.get(url))?;
        let response = request
            .send()
            .await
            .map_err(|err| HealthpointError::Request(err.to_string()))?;
        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|err| HealthpointError::Request(err.to_string()))?;
        if !status.is_success() {
            return Err(HealthpointError::Api {
                status: status.as_u16(),
                message: body,
            });
        }
        serde_json::from_str(&body).map_err(|err| HealthpointError::Parse(err.to_string()))
    }

    fn encode_service_query(&self, url: &mut Url, query: &ServiceQuery) {
        let mut pairs = url.query_pairs_mut();
        if let Some(text) = &query.text {
            pairs.append_pair("_content", text);
        }
        for code in &query.categories {
            pairs.append_pair("category", &code.as_token());
        }
        for code in &query.service_types {
            pairs.append_pair("type", &code.as_token());
        }
        for code in &query.specialties {
            pairs.append_pair("specialty", &code.as_token());
        }
        pairs.append_pair("_count", &query.limit.clamped().to_string());
        if let Some(cursor) = &query.cursor {
            pairs.append_pair("_cursor", cursor);
        }
        if let Some(point) = query.nearby {
            match self.config.geo_search_mode {
                GeoSearchMode::HealthpointLatLon => {
                    pairs.append_pair("latitude", &point.lat.to_string());
                    pairs.append_pair("longitude", &point.lon.to_string());
                    if let Some(radius) = query.radius_km {
                        pairs.append_pair("radius_km", &radius.to_string());
                    }
                }
                GeoSearchMode::FhirNear => {
                    let mut near = format!("{}|{}", point.lat, point.lon);
                    if let Some(radius) = query.radius_km {
                        near.push('|');
                        near.push_str(&radius.to_string());
                    }
                    pairs.append_pair("near", &near);
                }
            }
        }
    }
}

fn absolute_same_origin_cursor(base_url: &Url, cursor: Option<&str>) -> Result<Option<Url>> {
    let Some(cursor) = cursor.filter(|value| !value.trim().is_empty()) else {
        return Ok(None);
    };
    let parsed = match Url::parse(cursor) {
        Ok(url) => url,
        Err(_) => return Ok(None),
    };
    if parsed.origin() != base_url.origin() {
        return Err(HealthpointError::Config(
            "refusing to follow a pagination cursor from a different origin".into(),
        ));
    }
    Ok(Some(parsed))
}

#[async_trait]
impl DirectoryProvider for HealthpointClient {
    async fn search_services(&self, query: ServiceQuery) -> Result<Page<ServiceRecord>> {
        let url = self.service_search_url(&query)?;
        let value = self.get_json(url).await?;
        let total = healthpoint_fhir::total(&value);
        let next_cursor = healthpoint_fhir::next_link(&value);
        let provenance = self.provenance();
        let items = healthpoint_fhir::services_from_fhir(value, provenance.clone())?;
        Ok(Page {
            items,
            next_cursor,
            total,
            provenance,
        })
    }

    async fn get_service(&self, id: &str) -> Result<ServiceRecord> {
        let url = self.resource_url("HealthcareService", Some(id))?;
        let provenance = self.provenance();
        let value = self.get_json(url).await?;
        let mut records = healthpoint_fhir::services_from_fhir(value, provenance)?;
        records
            .pop()
            .ok_or_else(|| HealthpointError::Parse("empty HealthcareService response".into()))
    }

    async fn get_organization(&self, id: &str) -> Result<OrganizationRecord> {
        let url = self.resource_url("Organization", Some(id))?;
        let provenance = self.provenance();
        let value = self.get_json(url).await?;
        healthpoint_fhir::organization_from_fhir(value, provenance)
    }

    async fn get_location(&self, id: &str) -> Result<LocationRecord> {
        let url = self.resource_url("Location", Some(id))?;
        let provenance = self.provenance();
        let value = self.get_json(url).await?;
        healthpoint_fhir::location_from_fhir(value, provenance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use healthpoint_core::{Code, GeoPoint, QueryLimit};

    #[test]
    fn auth_scheme_parses_named_headers() {
        assert!(matches!(parse_auth_scheme("Bearer"), Ok(AuthScheme::Bearer)));
        assert!(matches!(
            parse_auth_scheme("header:X-Healthpoint-Key"),
            Ok(AuthScheme::Header(name)) if name == "X-Healthpoint-Key"
        ));
    }

    #[test]
    fn service_url_encodes_token_searches() {
        let config = ClientConfig::new(
            Url::parse("https://example.test/fhir/").expect("valid URL"),
            None,
            AuthScheme::None,
        );
        let client = HealthpointClient::new(config);
        let query = ServiceQuery {
            text: Some("cervical screening".into()),
            service_types: vec![Code::snomed("171149006")],
            nearby: Some(GeoPoint { lat: -36.8, lon: 174.7 }),
            radius_km: Some(10.0),
            limit: QueryLimit(250),
            ..ServiceQuery::default()
        };
        let url = client.service_search_url(&query).expect("URL builds");
        let rendered = url.as_str();
        assert!(rendered.starts_with("https://example.test/fhir/HealthcareService?"));
        assert!(rendered.contains("_content=cervical"));
        assert!(rendered.contains("type=http%3A%2F%2Fsnomed.info%2Fsct%7C171149006"));
        assert!(rendered.contains("_count=100"));
        assert!(rendered.contains("latitude=-36.8"));
    }

    #[test]
    fn absolute_cursor_must_match_origin() {
        let base = Url::parse("https://example.test/fhir/").expect("valid URL");
        assert!(absolute_same_origin_cursor(
            &base,
            Some("https://other.test/HealthcareService?page=2")
        )
        .is_err());
    }
}
