use crate::url::*;
use crate::utils::{parse_json_error, parse_request_error};
use reqwest::Client as HttpClient;
use rmcp::handler::server::tool::{Parameters, ToolRouter};
use rmcp::model::{CallToolResult, Content};
use rmcp::{
    ErrorData as McpError,
    model::{Implementation, ServerCapabilities, ServerInfo},
    tool,
};
use rmcp::{ServerHandler, tool_handler, tool_router};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct FirstradeServer {
    client: HttpClient,
    tool_router: ToolRouter<Self>,
}

#[derive(Debug, JsonSchema, Deserialize, Serialize)]
struct WatchListRequest {
    id: Option<u8>,
    name: Option<String>,
    symbol: Option<String>,
}

#[derive(Debug, JsonSchema, Deserialize, Serialize)]
struct StockQuoteRequest {
    symbol: String,
}

#[tool_router]
impl FirstradeServer {
    pub fn new() -> Self {
        FirstradeServer {
            client: HttpClient::new(),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Get User Info")]
    async fn get_user_info(&self) -> Result<CallToolResult, McpError> {
        let url = user_info();
        let user_info = self.send_get_request(url).await?;
        Ok(CallToolResult::success(vec![Content::json(user_info)?]))
    }

    #[tool(description = "Get Account positions")]
    async fn get_account_position(&self) -> Result<CallToolResult, McpError> {
        let positions = self.send_get_request(position()).await?;
        Ok(CallToolResult::success(vec![Content::json(positions)?]))
    }

    #[tool(description = "Get Account balance")]
    async fn get_account_balance(&self) -> Result<CallToolResult, McpError> {
        let balances = self.send_get_request(account_balances()).await?;
        Ok(CallToolResult::success(vec![Content::json(balances)?]))
    }

    #[tool(description = "Get Market Time")]
    async fn get_market_time(&self) -> Result<CallToolResult, McpError> {
        let balances = self.send_get_request(market_time()).await?;
        Ok(CallToolResult::success(vec![Content::json(balances)?]))
    }

    #[tool(description = "Get Account history")]
    async fn get_account_history(&self) -> Result<CallToolResult, McpError> {
        let history = self.send_get_request(account_history()).await?;
        Ok(CallToolResult::success(vec![Content::json(history)?]))
    }

    #[tool(description = "Get Account watchlists")]
    async fn get_account_watchlists(&self) -> Result<CallToolResult, McpError> {
        let watchlists = self.send_get_request(watchlists()).await?;
        Ok(CallToolResult::success(vec![Content::json(watchlists)?]))
    }

    #[tool(description = "Get Account watchlist quotes")]
    async fn get_watchlist_quote(
        &self,
        Parameters(WatchListRequest { id, .. }): Parameters<WatchListRequest>,
    ) -> Result<CallToolResult, McpError> {
        let url = watchlist_quote(id.unwrap());
        let quotes = self.send_get_request(url).await?;
        Ok(CallToolResult::success(vec![Content::json(quotes)?]))
    }

    #[tool(description = "Add a new Account watchlist")]
    async fn add_new_watchlist(
        &self,
        Parameters(WatchListRequest { name, .. }): Parameters<WatchListRequest>,
    ) -> Result<CallToolResult, McpError> {
        let url = format!("{}/{}", watchlists(), name.unwrap());
        let quotes = self.send_post_request(url, None).await?;
        Ok(CallToolResult::success(vec![Content::json(quotes)?]))
    }

    #[tool(description = "Delete a watchlist")]
    async fn delete_watchlist(
        &self,
        Parameters(WatchListRequest { id, .. }): Parameters<WatchListRequest>,
    ) -> Result<CallToolResult, McpError> {
        let url = format!("{}/{}", watchlists(), id.unwrap());
        let quotes = self.send_delete_request(url).await?;
        Ok(CallToolResult::success(vec![Content::json(quotes)?]))
    }

    #[tool(description = "Add a symbol to watchlist using watchlist id")]
    async fn watchlist_add_symbol(
        &self,
        // TODO: make LLM understand parameters' type
        Parameters(WatchListRequest { id, symbol, .. }): Parameters<WatchListRequest>,
    ) -> Result<CallToolResult, McpError> {
        let url = format!("{}/{}", watchlist(), id.unwrap());
        let mut body = HashMap::new();
        body.insert("symbol".to_string(), symbol.unwrap());
        let quotes = self.send_post_request(url, Some(body)).await?;
        Ok(CallToolResult::success(vec![Content::json(quotes)?]))
    }

    #[tool(description = "Remove a symbol from watchlist, symbol id can be found inside watchlist quote")]
    async fn watchlist_remove_symbol(
        &self,
        Parameters(WatchListRequest { id, .. }): Parameters<WatchListRequest>,
    ) -> Result<CallToolResult, McpError> {
        let url = format!("{}/{}", watchlist(), id.unwrap());
        let quotes = self.send_delete_request(url).await?;
        Ok(CallToolResult::success(vec![Content::json(quotes)?]))
    }

    #[tool(description = "Get Single Stock Quote")]
    async fn get_single_quote(
        &self,
        Parameters(StockQuoteRequest { symbol }): Parameters<StockQuoteRequest>,
    ) -> Result<CallToolResult, McpError> {
        let url = single_quote(&symbol);
        let quote = self.send_get_request(url).await?;
        Ok(CallToolResult::success(vec![Content::json(quote)?]))
    }

    #[tool(description = "Get Fundamental data for a stock")]
    async fn get_fundamental(
        &self,
        Parameters(StockQuoteRequest { symbol }): Parameters<StockQuoteRequest>,
    ) -> Result<CallToolResult, McpError> {
        let url = fundamental(&symbol);
        let data = self.send_get_request(url).await?;
        Ok(CallToolResult::success(vec![Content::json(data)?]))
    }

    #[tool(description = "Get Company Profile for a stock")]
    async fn get_company_profile(
        &self,
        Parameters(StockQuoteRequest { symbol }): Parameters<StockQuoteRequest>,
    ) -> Result<CallToolResult, McpError> {
        let url = company_profile(&symbol);
        let data = self.send_get_request(url).await?;
        Ok(CallToolResult::success(vec![Content::json(data)?]))
    }

    #[tool(description = "Get Cash Dividend for a stock")]
    async fn get_cash_dividend(
        &self,
        Parameters(StockQuoteRequest { symbol }): Parameters<StockQuoteRequest>,
    ) -> Result<CallToolResult, McpError> {
        let url = cash_dividend(&symbol);
        let data = self.send_get_request(url).await?;
        Ok(CallToolResult::success(vec![Content::json(data)?]))
    }

    #[tool(description = "Get Corporate Calendar for a stock")]
    async fn get_corp_calendar(
        &self,
        Parameters(StockQuoteRequest { symbol }): Parameters<StockQuoteRequest>,
    ) -> Result<CallToolResult, McpError> {
        let url = corp_calendar(&symbol);
        let data = self.send_get_request(url).await?;
        Ok(CallToolResult::success(vec![Content::json(data)?]))
    }
}

#[tool_handler]
impl ServerHandler for FirstradeServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("Firstrade MCP Server".to_string()),
            ..Default::default()
        }
    }
}

impl FirstradeServer {
    async fn send_get_request(&self, url: String) -> Result<Value, McpError> {
        tracing::debug!("Send get to URL: {}", url);
        let resp = self.client.get(url).send().await.map_err(parse_request_error)?;
        resp.json::<Value>().await.map_err(parse_json_error)
    }

    async fn send_post_request(
        &self,
        url: String,
        body: Option<HashMap<String, String>>,
    ) -> Result<Value, McpError> {
        tracing::debug!("Send post to URL: {}", url);
        let mut req = self.client.post(&url);
        if let Some(b) = body {
            req = req.form(&b);
        }
        let resp = req.send().await.map_err(parse_request_error)?;
        resp.json::<Value>().await.map_err(parse_json_error)
    }

    async fn send_delete_request(&self, url: String) -> Result<Value, McpError> {
        tracing::debug!("Send post to URL: {}", url);
        let resp = self
            .client
            .delete(&url)
            .send()
            .await
            .map_err(parse_request_error)?;
        resp.json::<Value>().await.map_err(parse_json_error)
    }
}
