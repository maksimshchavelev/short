use clap::{Parser, Subcommand};

/// CLI for working with the url shortener server
#[derive(Parser)]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct CLI {
    /// Long URL.
    pub url: Option<String>,

    /// Shortener's server URL.
    /// You can set the SHORTENER_SERVER environment variable instead
    #[arg(short, long)]
    pub server: Option<String>,

    /// Link lifetime duration. For example, 12d means 12 days
    #[arg(short, long)]
    pub lifetime: Option<String>,

    /// Limit of clicks for a link
    #[arg(short, long)]
    pub clicks: Option<i64>,

    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
    /// Discover info about short code
    Discover {
        /// Short code or full URL
        code: String
    }
}