use serde::Deserialize;

/// JSON response from a server. Use it to parse failed 4xx answers
#[derive(Deserialize, Debug)]
pub struct FailedResponse {
    /// Title of error
    pub title: String,

    /// Type of error
    #[serde(rename = "type")]
    pub error_type: String,

    /// What's happened?
    pub detail: String,
}
