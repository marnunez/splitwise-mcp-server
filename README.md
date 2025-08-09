# Splitwise MCP Server

A Rust-based Model Context Protocol (MCP) server that provides Claude and other AI agents with access to your Splitwise account for expense tracking and management. This server can be deployed locally or remotely and accessed via HTTP API, making it compatible with web-based AI services.

## Features

### User Management
- Get current user information
- Retrieve details about other users

### Group Management
- List all groups
- Get group details
- Create new groups
- Add/remove users from groups

### Expense Management
- List expenses with filters (date range, group, friend)
- Get expense details
- Create new expenses (equal or custom split)
- Update existing expenses
- Delete expenses

### Friend Management
- List all friends with balances
- Get friend details
- Add new friends by email

### Utilities
- Get supported currencies
- Get expense categories

## Prerequisites

- Rust 1.70+ and Cargo
- Splitwise account and API key
- For local: Claude Desktop or other MCP-compatible client
- For remote: Docker and docker-compose (optional)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/marnunez/splitwise-mcp-server.git
cd splitwise-mcp-server
```

2. Copy the environment file and add your API key:
```bash
cp .env.example .env
# Edit .env and add your SPLITWISE_API_KEY
```

3. Get your Splitwise API key:
   - Go to https://secure.splitwise.com/apps
   - Register a new application
   - Copy the API key

4. Build the project:
```bash
cargo build --release
```

## Configuration

### Environment Variables

Create a `.env` file with:

```env
# Required
SPLITWISE_API_KEY=your_api_key_here

# For HTTP server (remote access)
MCP_AUTH_TOKEN=your_secure_token_here
PORT=8080

# Optional logging level
RUST_LOG=info
```

### Local Setup (Claude Desktop)

Add to your Claude configuration file (`claude.json`):

```json
{
  "mcpServers": {
    "splitwise": {
      "command": "/path/to/splitwise-mcp-server/target/release/splitwise-mcp",
      "args": [],
      "env": {
        "SPLITWISE_API_KEY": "your_api_key_here",
        "RUST_LOG": "info"
      }
    }
  }
}
```

### Remote Setup (HTTP Server)

#### Docker Deployment (Recommended)

1. Create `.env` file with your credentials:
```bash
SPLITWISE_API_KEY=your_api_key_here
MCP_AUTH_TOKEN=your_secure_token_here
```

2. Deploy with docker-compose:
```bash
docker-compose up -d
```

The server will be available at `http://your-server:8080/mcp`

#### Manual Deployment

```bash
# Build the HTTP server
cargo build --release --bin splitwise-mcp-http

# Run with authentication
SPLITWISE_API_KEY=your_key MCP_AUTH_TOKEN=your_token ./target/release/splitwise-mcp-http
```

## Usage Examples

Once configured, you can ask Claude to:

- **View expenses**: "Show me my recent Splitwise expenses"
- **Create expense**: "Add a $50 dinner expense to my 'Roommates' group"
- **Check balances**: "What are my current balances with friends?"
- **Manage groups**: "Create a new group called 'Weekend Trip'"
- **Track spending**: "How much did I spend on groceries last month?"

## Available Tools

### User Tools
- `get_current_user` - Get authenticated user info
- `get_user` - Get user by ID

### Group Tools
- `list_groups` - List all groups
- `get_group` - Get group details
- `create_group` - Create new group

### Expense Tools
- `list_expenses` - List expenses with filters
- `get_expense` - Get expense details
- `create_expense` - Create new expense
- `update_expense` - Update expense
- `delete_expense` - Delete expense

### Friend Tools
- `list_friends` - List friends and balances
- `get_friend` - Get friend details
- `add_friend` - Add friend by email

### Utility Tools
- `get_currencies` - List supported currencies
- `get_categories` - List expense categories

## Using with AI Services

### ChatGPT (Custom GPT)

1. Create a Custom GPT
2. Add an action with this OpenAPI spec:

```yaml
openapi: 3.0.0
info:
  title: Splitwise MCP Server
  version: 0.1.0
servers:
  - url: https://your-server.com:8080
paths:
  /mcp:
    post:
      operationId: callMCP
      security:
        - bearerAuth: []
      requestBody:
        required: true
        content:
          application/json:
            schema:
              type: object
      responses:
        '200':
          description: Success
components:
  securitySchemes:
    bearerAuth:
      type: http
      scheme: bearer
```

3. Configure authentication with your `MCP_AUTH_TOKEN`

### Claude (Web/Mobile)

For Claude on web or mobile, you can:
1. Deploy the server to a public URL
2. Use function calling to interact with the API
3. Provide the API documentation to Claude

## Development

### Running in Development Mode

```bash
# Local stdio server
export SPLITWISE_API_KEY=your_api_key
export RUST_LOG=debug
cargo run --bin splitwise-mcp

# HTTP server for remote access
export MCP_AUTH_TOKEN=test-token
cargo run --bin splitwise-mcp-http
```

### Testing

Test the server using the MCP Inspector:

1. Install MCP Inspector (if not already installed)
2. Run the server
3. Connect using stdio transport
4. Test individual tools

### Building for Production

```bash
cargo build --release
```

The optimized binary will be at `target/release/splitwise-mcp`.

## API Rate Limits

The Splitwise API has rate limits. The server handles these gracefully, but be aware:
- Conservative rate limits for self-serve API
- Consider caching for frequently accessed data
- Use filters to minimize API calls

## API Endpoints (HTTP Server)

### Server Info
```bash
curl http://localhost:8080/
```

### MCP Operations
```bash
# Initialize
curl -X POST http://localhost:8080/mcp \
  -H "Authorization: Bearer your_token" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"initialize","params":{},"id":1}'

# List tools
curl -X POST http://localhost:8080/mcp \
  -H "Authorization: Bearer your_token" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"tools/list","params":{},"id":2}'

# Call a tool
curl -X POST http://localhost:8080/mcp \
  -H "Authorization: Bearer your_token" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"tools/call","params":{"name":"get_current_user","arguments":{}},"id":3}'
```

## Security

- **Never commit your API keys** - Use environment variables
- The `.env` file is gitignored by default
- Use strong authentication tokens for HTTP server
- Consider HTTPS with proper certificates in production
- API keys are only stored in memory during runtime

## Troubleshooting

### Server won't start
- Check that `SPLITWISE_API_KEY` is set correctly
- Verify the API key is valid at https://secure.splitwise.com/apps
- Check logs with `RUST_LOG=debug`

### Connection issues
- Ensure the binary path in Claude config is absolute
- Check that the binary has execute permissions
- Verify stdio transport is working

### API errors
- Check Splitwise API status
- Verify API key permissions
- Review rate limit constraints

## Contributing

Contributions are welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Submit a pull request

## License

MIT License - See LICENSE file for details

## Disclaimer

This is an unofficial integration with Splitwise. Use responsibly and in accordance with Splitwise's API Terms of Service.