//! Paginated result model.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::SourceProvenance;

/// Page of records from a provider.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Page<T> {
    /// Records on this page.
    pub items: Vec<T>,
    /// Optional next cursor.
    pub next_cursor: Option<String>,
    /// Optional total result count.
    pub total: Option<u64>,
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
            provenance,
        }
    }
}
