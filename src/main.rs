use clap::Parser;
use dotenv::dotenv;
use short::cli;
use short::net::Worker;
use std::error;
use std::{env, process};

fn main() -> Result<(), Box<dyn error::Error>> {
    dotenv().ok();

    let cli = cli::CLI::parse();

    let server = cli
        .server
        .unwrap_or_else(|| match env::var("SHORTENER_SERVER") {
            Ok(url) => url,
            Err(e) => {
                eprintln!("Failed to parse SHORTENER_SERVER environment variable: {e}");
                eprintln!("You can specify the server manually using the --server option");
                process::exit(1);
            }
        });

    Worker::create_link(server, cli.url)?;

    Ok(())
}
