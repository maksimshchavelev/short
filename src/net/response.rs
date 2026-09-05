use chrono::{DateTime, Utc};
use serde::Deserialize;

/// JSON response from a server. Use it to parse successful 2xx answers
/// to link create requests
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct LinkCreated {
    /// Original URL
    pub url: String,

    /// Generated short code
    pub code: String,
}

/// JSON response from a server. Use it to parse failed 4xx answers
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct FailedResponse {
    /// Title of error
    pub title: String,

    /// Type of error
    #[serde(rename = "type")]
    pub error_type: String,

    /// What's happened?
    pub detail: String,
}

/// JSON response from a server. Use it to parse successfull 2xx answers
/// to discover short code
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct DiscoverResponse {
    /// Original URL
    pub url: String,

    /// Short code
    pub code: String,

    /// Count of clicks
    pub clicks: i64,

    /// Limit of clicks
    pub clicks_limit: Option<i64>,

    /// When short link created
    pub created_at: DateTime<Utc>,

    /// When short link expires
    pub expires_at: Option<DateTime<Utc>>,
}
