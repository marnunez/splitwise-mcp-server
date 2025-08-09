# Splitwise MCP Server

A Rust-based Model Context Protocol (MCP) server that provides Claude and other AI agents with access to your Splitwise account for expense tracking and management.

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
- Claude Desktop or other MCP-compatible client

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
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

# Optional logging level
RUST_LOG=info
```

### Claude Desktop Configuration

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

## Development

### Running in Development Mode

```bash
# Set environment variables
export SPLITWISE_API_KEY=your_api_key
export RUST_LOG=debug

# Run the server
cargo run
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

## Security

- **Never commit your API key** - Use environment variables
- The `.env` file is gitignored by default
- API keys are only stored in memory during runtime
- Use read-only access when possible

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