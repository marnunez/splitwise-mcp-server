use anyhow::{Context, Result};
use dotenv::dotenv;
use rmcp::model::{CallToolResult, InitializeResult, ServerCapabilities};
use rmcp::ServerHandler;
use rmcp::transport::stdio;
use serde_json::json;
use std::env;
use std::sync::Arc;
use tracing::{error, info};
use tracing_subscriber;

mod splitwise;
mod tools;
mod types;

use splitwise::SplitwiseClient;
use tools::SplitwiseTools;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    // Load environment variables
    dotenv().ok();

    // Get API key from environment
    let api_key = env::var("SPLITWISE_API_KEY")
        .context("SPLITWISE_API_KEY environment variable not set")?;

    info!("Starting Splitwise MCP server...");

    // Create Splitwise client
    let client = Arc::new(
        SplitwiseClient::new(api_key)
            .context("Failed to create Splitwise client")?,
    );

    // Create tools handler
    let tools = Arc::new(SplitwiseTools::new(client));

    // Create MCP server
    let server = ServerBuilder::new()
        .on_initialize(|params| {
            info!("Client connected: {:?}", params.client_info);
            
            Ok(InitializeResult {
                server_info: json!({
                    "name": "splitwise-mcp-server",
                    "version": "0.1.0",
                    "description": "MCP server for Splitwise expense tracking"
                }),
                capabilities: ServerCapabilities {
                    tools: Some(json!({
                        "available": true
                    })),
                    ..Default::default()
                },
                ..Default::default()
            })
        })
        .on_list_tools(move |_| {
            let tools = tools.clone();
            let tool_list = tools.get_tools();
            Ok(json!({
                "tools": tool_list
            }))
        })
        .on_call_tool({
            let tools = tools.clone();
            move |params| {
                let tools = tools.clone();
                Box::pin(async move {
                    match tools.handle_tool_call(&params.name, params.arguments).await {
                        Ok(result) => Ok(CallToolResult {
                            content: vec![json!({
                                "type": "text",
                                "text": result.to_string(),
                            })],
                            ..Default::default()
                        }),
                        Err(e) => {
                            error!("Tool call failed: {}", e);
                            Ok(CallToolResult {
                                content: vec![json!({
                                    "type": "text",
                                    "text": json!({
                                        "error": e.to_string()
                                    }).to_string(),
                                })],
                                is_error: Some(true),
                                ..Default::default()
                            })
                        }
                    }
                })
            }
        })
        .build();

    // Create stdio transport and run
    let transport = StdioTransport::new();

    info!("Splitwise MCP server started successfully");

    // Run the server
    server.run(transport).await?;

    Ok(())
}