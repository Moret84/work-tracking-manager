use serde::{Deserialize, Serialize};

use crate::WorkDuration;

#[derive(Deserialize, Serialize, Clone)]
pub struct TrackingRecord {
    pub id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    pub duration: WorkDuration
}
