use clap::Parser;

/// CLI for working with the server
#[derive(Parser)]
pub struct CLI {
    /// Long URL.
    pub url: String,

    /// Shortener's server URL.
    /// You can set the SHORTENER_SERVER environment variable instead
    #[arg(short, long)]
    pub server: Option<String>,
}
