#!/bin/bash

(
  echo '{"jsonrpc":"2.0","method":"initialize","params":{"clientInfo":{"name":"test"}},"id":1}'
  sleep 0.5
  echo '{"jsonrpc":"2.0","method":"tools/call","params":{"name":"update_expense","arguments":{"expense_id":3979725244,"split_equally":false,"split_by_shares":[{"user_id":11116744,"paid_share":"28000","owed_share":"28000"},{"user_id":11116812,"paid_share":"0","owed_share":"0"}]}},"id":2}'
  sleep 0.5
) | /home/marcos/claude-assistant/mcps/splitwise-mcp-server/target/release/splitwise-mcp 2>/dev/null | grep -E '"id":2' | jq -r '.result.content[0].text' | jq .