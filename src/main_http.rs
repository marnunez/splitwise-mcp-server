use anyhow::{Context, Result};
use axum::{
    extract::{Query, State},
    http::{header, HeaderMap, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
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
    client_id: String,
    client_secret: String,
}

#[derive(Deserialize)]
struct TokenRequest {
    grant_type: String,
    client_id: String,
    client_secret: String,
}

#[derive(Serialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i32,
}

// Authentication middleware - supports both Bearer token and Basic auth
async fn check_auth(headers: &HeaderMap, state: &AppState) -> Result<(), StatusCode> {
    // First try Bearer token
    if let Some(auth_header) = headers.get(header::AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            // Check Bearer token
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                if token == state.auth_token {
                    return Ok(());
                }
            }
            
            // Check Basic auth (client_id:client_secret base64 encoded)
            if let Some(basic) = auth_str.strip_prefix("Basic ") {
                if let Ok(decoded) = STANDARD.decode(basic) {
                    if let Ok(credentials) = String::from_utf8(decoded) {
                        let parts: Vec<&str> = credentials.split(':').collect();
                        if parts.len() == 2 && 
                           parts[0] == state.client_id && 
                           parts[1] == state.client_secret {
                            return Ok(());
                        }
                    }
                }
            }
        }
    }
    
    Err(StatusCode::UNAUTHORIZED)
}

// OAuth2 token endpoint
async fn oauth_token_handler(
    State(state): State<AppState>,
    Json(request): Json<TokenRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate client credentials
    if request.grant_type != "client_credentials" {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    if request.client_id != state.client_id || request.client_secret != state.client_secret {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    // Return access token (which is our MCP_AUTH_TOKEN)
    Ok(Json(TokenResponse {
        access_token: state.auth_token.clone(),
        token_type: "Bearer".to_string(),
        expires_in: 3600, // 1 hour
    }))
}


// HTTP POST endpoint for MCP requests
async fn mcp_handler(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> Result<impl IntoResponse, StatusCode> {
    // Check authentication
    check_auth(&headers, &state).await?;

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
    
    let client_id = env::var("OAUTH_CLIENT_ID")
        .unwrap_or_else(|_| {
            info!("OAUTH_CLIENT_ID not set, generating default");
            "splitwise-mcp-client".to_string()
        });
    
    let client_secret = env::var("OAUTH_CLIENT_SECRET")
        .unwrap_or_else(|_| {
            warn!("OAUTH_CLIENT_SECRET not set, generating random secret");
            // Generate a random secret if not provided
            STANDARD.encode(&rand::random::<[u8; 32]>())
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
        client_id: client_id.clone(),
        client_secret: client_secret.clone(),
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
        // OAuth2 token endpoint
        .route("/oauth/token", post(oauth_token_handler))
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
    info!("OAuth2 token endpoint: http://{}:{}/oauth/token", "localhost", port);
    info!("Client ID: {}", client_id);
    info!("Client Secret: {} (keep this secret!)", if client_secret.len() > 10 { 
        format!("{}...", &client_secret[..10]) 
    } else { 
        "GENERATED".to_string() 
    });

    // Start the server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}