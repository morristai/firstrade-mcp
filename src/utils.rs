use anyhow::Result;
use once_cell::sync::Lazy;
use rmcp::ErrorData as McpError;
use serde_json::json;
use std::env;
use std::sync::Arc;
use tracing::Level;
use tracing_subscriber::EnvFilter;

pub fn init_logging() -> Result<()> {
    let log_level = env::var("LOG_LEVEL")
        .map(|level| match level.to_lowercase().as_str() {
            "error" => Level::ERROR,
            "warn" => Level::WARN,
            "info" => Level::INFO,
            "debug" => Level::DEBUG,
            _ => Level::INFO,
        })
        .unwrap_or(Level::INFO);

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(log_level.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    Ok(())
}

pub static API_HOST: Lazy<Arc<String>> = Lazy::new(|| {
    let api_url = env::var("API_HOST").unwrap_or_else(|_| "localhost".to_string());
    let api_port = env::var("API_PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse::<u16>()
        .expect("API_PORT must be a valid u16");

    tracing::info!("Using API host: {}:{}", api_url, api_port);
    Arc::new(format!("{}:{}", api_url, api_port))
});

#[inline(always)]
pub(crate) fn parse_request_error(err: reqwest::Error) -> McpError {
    McpError::internal_error(
        format!("Failed to fetch URL: {}", err.url().unwrap()),
        Some(json!({ "reason": err.to_string() })),
    )
}

#[inline(always)]
pub(crate) fn parse_json_error(err: reqwest::Error) -> McpError {
    McpError::internal_error(
        "Failed to parse response Json",
        Some(json!({ "reason": err.to_string() })),
    )
}
