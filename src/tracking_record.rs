use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TrackingRecord {
    pub id: String,
    pub description: String,
    pub time: Duration
}
