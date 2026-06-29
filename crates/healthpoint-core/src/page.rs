//! Paginated result model.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::SourceProvenance;

/// HTTP/provider response metadata safe to expose in CLI/MCP outputs.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ResponseMetadata {
    /// ETag header, when supplied.
    pub etag: Option<String>,
    /// Last-Modified header, when supplied.
    pub last_modified: Option<String>,
    /// Retry-After header, when supplied.
    pub retry_after: Option<String>,
    /// X-Request-Id or equivalent request identifier, when supplied.
    pub request_id: Option<String>,
    /// Rate-limit remaining header, when supplied.
    pub rate_limit_remaining: Option<String>,
    /// Rate-limit reset header, when supplied.
    pub rate_limit_reset: Option<String>,
}

/// Page of records from a provider.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Page<T> {
    /// Records on this page.
    pub items: Vec<T>,
    /// Optional next cursor.
    pub next_cursor: Option<String>,
    /// Optional total result count.
    pub total: Option<u64>,
    /// Safe response metadata.
    pub response_metadata: ResponseMetadata,
    /// Page-level provenance.
    pub provenance: SourceProvenance,
}

impl<T> Page<T> {
    /// Construct a page.
    pub fn new(items: Vec<T>, provenance: SourceProvenance) -> Self {
        Self {
            items,
            next_cursor: None,
            total: None,
            response_metadata: ResponseMetadata::default(),
            provenance,
        }
    }
}
