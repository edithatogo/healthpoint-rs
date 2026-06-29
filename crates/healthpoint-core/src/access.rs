//! Access, licensing, and redistribution policy markers.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// How a user is permitted to access the upstream data source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum AccessMode {
    /// Fully public access.
    Public,
    /// User must provide their own licensed API key.
    BringYourOwnKey,
    /// Access is governed by a specific licence/contract.
    Licensed,
    /// The access state is not yet known.
    Unknown,
}

/// Whether retrieved data may be redistributed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum RedistributionStatus {
    /// Redistribution is allowed under documented terms.
    Allowed,
    /// Redistribution is prohibited.
    Prohibited,
    /// Redistribution requires human review.
    RequiresReview,
    /// Redistribution status has not been established.
    Unknown,
}

/// Export policy chosen for a command/session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ExportPolicy {
    /// Outputs remain local to a licensed user.
    #[default]
    LocalOnly,
    /// Outputs may be shared inside the user's licensed context.
    LicensedShare,
    /// Outputs are explicitly approved for open publication.
    OpenApproved,
}

/// Combined policy metadata attached to exported data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct AccessPolicy {
    /// How the source was accessed.
    pub access_mode: AccessMode,
    /// Current redistribution status.
    pub redistribution_status: RedistributionStatus,
    /// Runtime export policy selected by the user/tool.
    pub export_policy: ExportPolicy,
    /// Whether public caching is allowed.
    pub public_cache_allowed: bool,
    /// Human-readable note.
    pub note: String,
}

impl Default for AccessPolicy {
    fn default() -> Self {
        Self {
            access_mode: AccessMode::BringYourOwnKey,
            redistribution_status: RedistributionStatus::RequiresReview,
            export_policy: ExportPolicy::LocalOnly,
            public_cache_allowed: false,
            note: "User-provided API key; do not assume retrieved data is redistributable".into(),
        }
    }
}
