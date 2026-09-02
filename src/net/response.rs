use serde::Deserialize;

/// JSON response from a server. Use it to parse successful 2xx answers
#[derive(Deserialize, Debug)]
pub struct Response {
    /// Original URL
    pub url: String,

    /// Generated short code
    pub code: String,
}
