use serde::Serialize;

/// JSON net to server
#[derive(Serialize, Debug)]
pub struct CreateLinkRequest {
    /// URL which will be shortened
    pub url: String,
}
