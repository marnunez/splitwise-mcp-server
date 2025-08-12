#!/bin/bash

# Test the new category filter functionality
TOOL_NAME="${1:-list_expenses}"
TOOL_ARGS="${2:-{}}"

(
  echo '{"jsonrpc":"2.0","method":"initialize","params":{"clientInfo":{"name":"test"}},"id":1}'
  sleep 0.5
  echo "{\"jsonrpc\":\"2.0\",\"method\":\"tools/call\",\"params\":{\"name\":\"$TOOL_NAME\",\"arguments\":$TOOL_ARGS},\"id\":2}"
  sleep 0.5
) | cargo run --bin splitwise-mcp 2>/dev/null | grep -E '"id":2'