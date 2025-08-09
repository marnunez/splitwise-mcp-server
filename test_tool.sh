#!/bin/bash

# Test getting current user
(
  echo '{"jsonrpc":"2.0","method":"initialize","params":{"clientInfo":{"name":"test"}},"id":1}'
  sleep 0.5
  echo '{"jsonrpc":"2.0","method":"tools/call","params":{"name":"get_current_user"},"id":2}'
  sleep 0.5
) | cargo run --bin splitwise-mcp 2>/dev/null | grep -E '"id":2'