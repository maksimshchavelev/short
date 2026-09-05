use clap::{Parser, Subcommand};

/// CLI for working with the server
#[derive(Parser)]
pub struct CLI {
    /// Long URL.
    pub url: Option<String>,

    /// Shortener's server URL.
    /// You can set the SHORTENER_SERVER environment variable instead
    #[arg(short, long)]
    pub server: Option<String>,

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