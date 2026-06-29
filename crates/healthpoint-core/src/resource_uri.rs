//! `healthpoint://` resource URI parsing shared by CLI and MCP adapters.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    error::{HealthpointError, Result},
    validation::validate_resource_id,
};

/// Read-only Healthpoint resource URI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum HealthpointResourceUri {
    /// `healthpoint://service/{id}`.
    Service(String),
    /// `healthpoint://location/{id}`.
    Location(String),
    /// `healthpoint://organization/{id}`.
    Organization(String),
}

impl HealthpointResourceUri {
    /// Parse a Healthpoint resource URI.
    pub fn parse(raw: &str) -> Result<Self> {
        let url = Url::parse(raw).map_err(|err| {
            HealthpointError::InvalidInput(format!("invalid healthpoint resource URI: {err}"))
        })?;
        if url.scheme() != "healthpoint" {
            return Err(HealthpointError::InvalidInput(
                "resource URI must use the healthpoint:// scheme".into(),
            ));
        }
        let mut segments = url
            .path_segments()
            .ok_or_else(|| HealthpointError::InvalidInput("resource URI is missing path".into()))?;
        let id = segments
            .next()
            .filter(|segment| !segment.is_empty())
            .ok_or_else(|| HealthpointError::InvalidInput("resource URI is missing id".into()))?;
        if segments.next().is_some() {
            return Err(HealthpointError::InvalidInput(
                "resource URI must contain exactly one id path segment".into(),
            ));
        }
        validate_resource_id(id)?;
        match url.host_str() {
            Some("service") => Ok(Self::Service(id.to_owned())),
            Some("location") => Ok(Self::Location(id.to_owned())),
            Some("organization") => Ok(Self::Organization(id.to_owned())),
            Some(other) => Err(HealthpointError::InvalidInput(format!(
                "unsupported healthpoint resource host {other:?}; use service, location, or organization"
            ))),
            None => Err(HealthpointError::InvalidInput(
                "resource URI is missing host".into(),
            )),
        }
    }

    /// Render the resource URI.
    pub fn as_uri(&self) -> String {
        match self {
            Self::Service(id) => format!("healthpoint://service/{id}"),
            Self::Location(id) => format!("healthpoint://location/{id}"),
            Self::Organization(id) => format!("healthpoint://organization/{id}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_resource_uri() {
        assert_eq!(
            HealthpointResourceUri::parse("healthpoint://service/svc-1").expect("valid"),
            HealthpointResourceUri::Service("svc-1".into())
        );
        assert!(HealthpointResourceUri::parse("https://service/svc-1").is_err());
        assert!(HealthpointResourceUri::parse("healthpoint://service/Location/bad").is_err());
    }
}
