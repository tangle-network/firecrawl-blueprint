use crate::context::Context;
use blueprint_sdk::{
    Error as SdkError,
    extract::Context as SdkContext,
    tangle::extract::{TangleArg, TangleResult},
};
use ipfs_api_backend_hyper::IpfsApi; // Import IpfsApi trait for add method
use serde_json::Value;
use std::io::Cursor; // Required for ipfs_client.add with bytes
use thiserror::Error;

// Define a specific error type for this handler
#[derive(Error, Debug)]
pub enum ScrapeHandlerError {
    #[error("HTTP request failed: {0}")]
    HttpRequest(#[from] reqwest::Error),
    #[error("JSON parsing failed: {0}")]
    JsonParse(#[from] serde_json::Error),
    #[error("IPFS operation failed: {0}")]
    Ipfs(String), // Use String to capture IPFS error details
    #[error("Input data is not valid JSON")]
    InvalidInputJson,
}

// Map internal error to SDK error
impl From<ScrapeHandlerError> for SdkError {
    fn from(err: ScrapeHandlerError) -> Self {
        SdkError::JobExecutionFailed(anyhow::anyhow!(err.to_string()))
    }
}

/// Job ID for the scrape operation.
pub const JOB_SCRAPE_ID: u64 = 1;

/// Handles the scrape job.
/// Takes a JSON string input via TangleArg, sends it to the Firecrawl scrape endpoint,
/// stores the JSON response in IPFS, and returns the CID.
pub async fn handle_scrape(
    SdkContext(ctx): SdkContext<Context>,
    TangleArg(input): TangleArg<String>,
) -> Result<TangleResult<String>, SdkError> {
    // Ensure input is valid JSON before proceeding
    let input_json: Value =
        serde_json::from_str(&input).map_err(|_| ScrapeHandlerError::InvalidInputJson)?;

    let client = reqwest::Client::new();
    let url = format!(
        "http://localhost:{}/api/v1/scrape",
        ctx.env.firecrawl_http_port
    );

    let response = client
        .post(&url)
        .json(&input_json) // Send parsed JSON
        .send()
        .await
        .map_err(ScrapeHandlerError::HttpRequest)?;

    // Ensure the request was successful
    if !response.status().is_success() {
        let status = response.status();
        let text = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read error body".to_string());
        return Err(ScrapeHandlerError::HttpRequest(reqwest::Error::from(
            // Create an error that includes status and body text
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("HTTP Error {}: {}", status, text),
            ),
        ))
        .into());
    }

    let response_json: Value = response
        .json()
        .await
        .map_err(ScrapeHandlerError::JsonParse)?;

    // Convert the JSON response back to bytes for IPFS
    let response_bytes =
        serde_json::to_vec(&response_json).map_err(ScrapeHandlerError::JsonParse)?;

    // Add the response bytes to IPFS
    let response_cursor = Cursor::new(response_bytes);
    let ipfs_result = ctx
        .ipfs_client
        .add(response_cursor)
        .await
        .map_err(|e| ScrapeHandlerError::Ipfs(e.to_string()))?;

    let cid = ipfs_result.hash;

    // Return the CID as a TangleResult
    Ok(TangleResult::new(cid))
}
