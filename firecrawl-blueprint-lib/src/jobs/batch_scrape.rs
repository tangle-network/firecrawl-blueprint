use crate::context::Context;
use anyhow::{Error, anyhow};
use blueprint_sdk::{
    Error as SdkError,
    extract::Context as SdkContext,
    tangle::extract::{TangleArg, TangleResult},
};
use serde_json::Value;
use std::io::Cursor; // Required for ipfs_client.add with bytes
// Removed custom error type: using unwrap() in handler

/// Job ID for the batch scrape operation.
pub const JOB_BATCH_SCRAPE_ID: u64 = 3;

/// Handles the batch scrape job.
/// Takes a JSON string input via TangleArg, sends it to the Firecrawl batch_scrape endpoint,
/// stores the JSON response in IPFS, and returns the CID.
pub async fn handle_batch_scrape(
    SdkContext(ctx): SdkContext<Context>,
    TangleArg(input): TangleArg<String>,
) -> Result<TangleResult<String>, SdkError> {
    // Ensure input is valid JSON before proceeding
    let input_json: Value =
        serde_json::from_str(&input).map_err(|_| BatchScrapeHandlerError::InvalidInputJson)?;

    let client = reqwest::Client::new();
    let url = format!(
        "http://localhost:{}/api/v1/batch_scrape", // Target batch_scrape endpoint
        ctx.env.firecrawl_http_port
    );

    let response = client
        .post(&url)
        .json(&input_json) // Send parsed JSON
        .send()
        .await
        .map_err(|e| BatchScrapeHandlerError::HttpRequest(anyhow!(e)))?;

    // Ensure the request was successful
    if !response.status().is_success() {
        let status = response.status();
        let text = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read error body".to_string());
        return Err(BatchScrapeHandlerError::HttpRequest(anyhow!(
            "HTTP Error {}: {}",
            status,
            text
        )));
    }

    let response_json: Value = response.json().await.unwrap();

    // Convert the JSON response back to bytes for IPFS
    let response_bytes =
        serde_json::to_vec(&response_json).map_err(BatchScrapeHandlerError::JsonParse)?;

    // Add the response bytes to IPFS
    let response_cursor = Cursor::new(response_bytes);
    let ipfs_result = ctx.ipfs_client.add(response_cursor).await.unwrap();

    let cid = ipfs_result.hash;

    // Return the CID as a TangleResult
    TangleResult(cid)
}
