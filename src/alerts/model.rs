use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookNotification {
    pub message: String,
    pub probe_name: String,
    pub failure_timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}
