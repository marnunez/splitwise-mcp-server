use anyhow::{Context, Result};
use dotenv::dotenv;
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

// Simple stdio server that responds to JSON-RPC requests
async fn run_server() -> Result<()> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    
    let api_key = env::var("SPLITWISE_API_KEY")
        .context("SPLITWISE_API_KEY environment variable not set")?;

    let client = Arc::new(SplitwiseClient::new(api_key)?);
    let tools = Arc::new(SplitwiseTools::new(client));
    
    let stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();
    
    // Send initialization response
    let _init_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "splitwise-mcp-server",
                "version": "0.1.0"
            }
        }
    });
    
    info!("MCP Server ready. Waiting for requests...");
    
    while let Some(line) = lines.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }
        
        let request: serde_json::Value = serde_json::from_str(&line)?;
        
        let response = if let Some(method) = request.get("method").and_then(|m| m.as_str()) {
            match method {
                "initialize" => {
                    json!({
                        "jsonrpc": "2.0",
                        "id": request.get("id"),
                        "result": {
                            "protocolVersion": "2024-11-05",
                            "capabilities": {
                                "tools": {}
                            },
                            "serverInfo": {
                                "name": "splitwise-mcp-server",
                                "version": "0.1.0"
                            }
                        }
                    })
                }
                "tools/list" => {
                    let tool_list = tools.get_tools();
                    json!({
                        "jsonrpc": "2.0",
                        "id": request.get("id"),
                        "result": {
                            "tools": tool_list
                        }
                    })
                }
                "tools/call" => {
                    let empty_params = json!({});
                    let params = request.get("params").unwrap_or(&empty_params);
                    let tool_name = params.get("name")
                        .and_then(|n| n.as_str())
                        .unwrap_or("");
                    let arguments = params.get("arguments").cloned();
                    
                    match tools.handle_tool_call(tool_name, arguments).await {
                        Ok(result) => {
                            json!({
                                "jsonrpc": "2.0",
                                "id": request.get("id"),
                                "result": {
                                    "content": [{
                                        "type": "text",
                                        "text": result.to_string()
                                    }]
                                }
                            })
                        }
                        Err(e) => {
                            json!({
                                "jsonrpc": "2.0",
                                "id": request.get("id"),
                                "error": {
                                    "code": -32603,
                                    "message": e.to_string()
                                }
                            })
                        }
                    }
                }
                _ => {
                    json!({
                        "jsonrpc": "2.0",
                        "id": request.get("id"),
                        "error": {
                            "code": -32601,
                            "message": format!("Method not found: {}", method)
                        }
                    })
                }
            }
        } else {
            json!({
                "jsonrpc": "2.0",
                "id": request.get("id"),
                "error": {
                    "code": -32600,
                    "message": "Invalid request"
                }
            })
        };
        
        let response_str = format!("{}\n", response);
        stdout.write_all(response_str.as_bytes()).await?;
        stdout.flush().await?;
    }
    
    Ok(())
}

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

    info!("Starting Splitwise MCP server...");
    
    run_server().await?;
    
    Ok(())
}