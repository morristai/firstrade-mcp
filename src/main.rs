use crate::server::FirstradeServer;
use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};
use utils::*;

mod server;
mod url;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    init_logging()?;

    let service = FirstradeServer::new().serve(stdio()).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    service.waiting().await?;
    Ok(())
}
