#!/bin/bash

echo "Testing Splitwise MCP Server..."
echo ""
echo "1. Initializing..."
echo '{"jsonrpc":"2.0","method":"initialize","params":{"clientInfo":{"name":"test"}},"id":1}'

echo ""
echo "2. Listing tools..."
echo '{"jsonrpc":"2.0","method":"tools/list","params":{},"id":2}'

echo ""
echo "3. Getting current user..."
echo '{"jsonrpc":"2.0","method":"tools/call","params":{"name":"get_current_user"},"id":3}'

echo ""
echo "4. Listing groups..."
echo '{"jsonrpc":"2.0","method":"tools/call","params":{"name":"list_groups"},"id":4}'