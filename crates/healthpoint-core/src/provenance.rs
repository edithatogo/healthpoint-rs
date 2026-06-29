//! Source provenance attached to retrieved records and exports.

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::access::AccessPolicy;

/// Provenance for a record, page, or export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SourceProvenance {
    /// Source system name.
    pub source_name: String,
    /// Source/API base URL.
    pub source_url: String,
    /// Retrieval timestamp.
    pub retrieved_at: DateTime<Utc>,
    /// Access/reuse policy marker.
    pub access_policy: AccessPolicy,
    /// Tool/version string.
    pub tool: String,
}

impl SourceProvenance {
    /// Create standard Healthpoint provenance.
    pub fn healthpoint(source_url: impl Into<String>) -> Self {
        Self {
            source_name: "Healthpoint API".into(),
            source_url: source_url.into(),
            retrieved_at: Utc::now(),
            access_policy: AccessPolicy::default(),
            tool: format!("healthpoint-rs/{}", env!("CARGO_PKG_VERSION")),
        }
    }
}
