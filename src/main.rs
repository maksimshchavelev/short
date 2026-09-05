use clap::Parser;
use dotenv::dotenv;
use duration_str::parse;
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

    match cli.command {
        Some(cli::Commands::Discover { code }) => {
            Worker::discover_link(server, code)?;
        }
        None => {
            if let Some(url) = cli.url {
                let lifetime = cli.lifetime.map(|value| {
                    parse(value).unwrap_or_else(|e| {
                        eprintln!("Failed to parse lifetime argument: {e}");
                        process::exit(1);
                    })
                });

                Worker::create_link(server, url, lifetime, cli.clicks)?;
            } else {
                eprintln!("URL argument not found");
            }
        }
    }

    Ok(())
}
