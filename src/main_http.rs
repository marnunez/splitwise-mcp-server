use anyhow::{Context, Result};
use axum::{
    extract::State,
    http::{header, HeaderMap, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use serde_json::json;
use std::{env, sync::Arc};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, warn};
use tracing_subscriber;

mod splitwise;
mod tools;
mod types;

use splitwise::SplitwiseClient;
use tools::SplitwiseTools;

#[derive(Clone)]
struct AppState {
    tools: Arc<SplitwiseTools>,
    auth_token: String,
}

// Authentication middleware
async fn check_auth(headers: &HeaderMap) -> Result<(), StatusCode> {
    let auth_header = headers
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Check Bearer token
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // In production, validate token properly
    let expected_token = env::var("MCP_AUTH_TOKEN").unwrap_or_else(|_| "default-token".to_string());
    
    if token != expected_token {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(())
}


// HTTP POST endpoint for MCP requests
async fn mcp_handler(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> Result<impl IntoResponse, StatusCode> {
    // Check authentication
    check_auth(&headers).await?;

    info!("HTTP request received: {:?}", request);

    // Parse the JSON-RPC request
    let method = request
        .get("method")
        .and_then(|m| m.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let response = match method {
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
            let tools = state.tools.get_tools();
            json!({
                "jsonrpc": "2.0",
                "id": request.get("id"),
                "result": {
                    "tools": tools
                }
            })
        }
        "tools/call" => {
            let params = request.get("params").ok_or(StatusCode::BAD_REQUEST)?;
            let tool_name = params
                .get("name")
                .and_then(|n| n.as_str())
                .ok_or(StatusCode::BAD_REQUEST)?;
            let arguments = params.get("arguments").cloned();

            match state.tools.handle_tool_call(tool_name, arguments).await {
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
    };

    Ok(Json(response))
}

// Health check endpoint
async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "service": "splitwise-mcp-server",
        "transport": "http"
    }))
}

// Server info endpoint
async fn server_info() -> impl IntoResponse {
    Json(json!({
        "name": "splitwise-mcp-server",
        "version": "0.1.0",
        "protocol": "2024-11-05",
        "transport": "http",
        "capabilities": {
            "tools": true,
            "resources": false,
            "prompts": false
        },
        "endpoints": {
            "mcp": "/mcp",
            "health": "/health",
            "info": "/"
        }
    }))
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

    info!("Starting Splitwise MCP HTTP/SSE server...");

    // Get configuration from environment
    let api_key = env::var("SPLITWISE_API_KEY")
        .context("SPLITWISE_API_KEY environment variable not set")?;
    
    let auth_token = env::var("MCP_AUTH_TOKEN")
        .unwrap_or_else(|_| {
            warn!("MCP_AUTH_TOKEN not set, using default token (INSECURE!)");
            "default-token".to_string()
        });

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .context("Invalid PORT")?;

    // Initialize Splitwise client and tools
    let client = Arc::new(SplitwiseClient::new(api_key)?);
    let tools = Arc::new(SplitwiseTools::new(client));

    // Create application state
    let state = AppState {
        tools,
        auth_token: auth_token.clone(),
    };

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);

    // Build the router
    let app = Router::new()
        // MCP endpoint
        .route("/mcp", post(mcp_handler))
        // Utility endpoints
        .route("/health", get(health_check))
        .route("/", get(server_info))
        // Add state and middleware
        .with_state(state)
        .layer(ServiceBuilder::new().layer(cors));

    // Bind to address
    let addr = format!("0.0.0.0:{}", port);
    info!("HTTP server listening on {}", addr);
    info!("MCP endpoint: http://{}:{}/mcp", "localhost", port);
    info!("Using auth token: {}", if auth_token == "default-token" { "DEFAULT (INSECURE!)" } else { "CUSTOM" });

    // Start the server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}