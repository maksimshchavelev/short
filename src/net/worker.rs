use crate::net::request::CreateLinkRequest;
use crate::net::response::{DiscoverResponse, FailedResponse, LinkCreated};
use std::{error, process};
use ureq::Body;
use ureq::http::Response;

/// Proceeds HTTP requests to a server
pub struct Worker;

impl Worker {
    /// Create new short code by long URL and print final short link
    /// # Returns
    /// Nothing or error
    pub fn create_link(
        server: String,
        url: String,
        lifetime: Option<std::time::Duration>,
        clicks_limit: Option<i64>,
    ) -> Result<(), Box<dyn error::Error>> {
        let request = CreateLinkRequest {
            url,
            lifetime_seconds: lifetime.map(|value| value.as_secs() as i64),
            clicks_limit,
        };

        let config = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .build();

        let agent: ureq::Agent = config.into();

        let mut response = agent.post(format!("{server}/link")).send_json(request)?;

        if response.status().is_success() {
            let short_code = response.body_mut().read_json::<LinkCreated>()?.code;
            println!("Your short link is: {server}/{short_code}");
        } else {
            Self::log_error_response(response)?;
        }

        Ok(())
    }

    /// Discover short code and print result
    /// # Returns
    /// Nothing or error
    pub fn discover_link(server: String, code: String) -> Result<(), Box<dyn error::Error>> {
        let code = code
            .strip_prefix(&format!("{}/", server))
            .unwrap_or(&code)
            .to_string();

        let config = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .build();

        let agent: ureq::Agent = config.into();

        let mut response = agent.get(format!("{server}/link/{code}")).call()?;

        if response.status().is_success() {
            let response = response.body_mut().read_json::<DiscoverResponse>()?;

            println!("{:<16} {}", "Original URL:", response.url);
            println!("{:<16} {}", "Count of clicks:", response.clicks);

            let clicks_limit = response
                .clicks_limit
                .map(|v| v.to_string())
                .unwrap_or_else(|| "N/A".to_string());

            println!("{:<16} {}", "Clicks limit:", clicks_limit);

            println!(
                "{:<16} {} UTC",
                "Created at:",
                response.created_at.format("%d.%m.%Y %H:%M:%S")
            );

            let expires_at = response
                .expires_at
                .map(|date| date.format("%d.%m.%Y %H:%M:%S").to_string())
                .unwrap_or_else(|| "N/A".to_string());

            println!("{:<16} {} UTC", "Expires at:", expires_at);
        } else {
            Self::log_error_response(response)?;
        }

        Ok(())
    }

    /// Consumes error response and logs it
    /// # Returns
    /// Nothing or error
    fn log_error_response(mut response: Response<Body>) -> Result<(), Box<dyn error::Error>> {
        if response.status().is_client_error() {
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
