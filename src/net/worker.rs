use crate::net::{request::CreateLinkRequest, response::FailedResponse, response::LinkCreated};
use std::{error, process};

/// Proceeds HTTP requests to a server
pub struct Worker;

impl Worker {
    /// Create new short code by long URL and print final short link
    /// # Returns
    /// Short code combined with `server` address or error
    pub fn create_link(server: String, url: String) -> Result<(), Box<dyn error::Error>> {
        let request = CreateLinkRequest { url };

        let config = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .build();

        let agent: ureq::Agent = config.into();

        let mut response = agent.post(format!("{server}/link")).send_json(request)?;

        if response.status().is_success() {
            let short_code = response.body_mut().read_json::<LinkCreated>()?.code;
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
}
