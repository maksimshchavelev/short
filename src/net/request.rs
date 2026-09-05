use serde::Serialize;

/// JSON net to server
#[derive(Serialize, Debug)]
pub struct CreateLinkRequest {
    /// URL which will be shortened
    pub url: String,

    /// Link lifetime in seconds
    pub lifetime_seconds: Option<i64>,

    /// Limit of clicks to link
    pub clicks_limit: Option<i64>,
}
