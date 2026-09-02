use serde::Serialize;

/// JSON net to server
#[derive(Serialize, Debug)]
pub struct Request {
    /// URL which will be shortened
    pub url: String,
}
