use clap::Parser;
use dotenv::dotenv;
use short::cli;
use short::net::{FailedResponse, Request, Response};
use std::error;
use std::{env, process};
use ureq;

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

    let request = Request { url: cli.url };

    let config = ureq::Agent::config_builder()
        .http_status_as_error(false)
        .build();

    let agent: ureq::Agent = config.into();

    let mut response = agent.post(format!("{server}/create")).send_json(request)?;

    if response.status().is_success() {
        let short_code = response.body_mut().read_json::<Response>()?.code;
        println!("Your short link is: {server}/{short_code}");
    } else if response.status().is_client_error() {
        let error = response.body_mut().read_json::<FailedResponse>()?;
        eprintln!("{}\n{}", error.title, error.detail);
        process::exit(1);
    } else if response.status().is_server_error() {
        eprintln!("Internal Server Error ({})", response.status());
        process::exit(1);
    } else {
        eprintln!("Error: {response:?}");
    }

    Ok(())
}
