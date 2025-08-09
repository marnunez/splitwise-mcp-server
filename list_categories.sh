#!/bin/bash
(echo '{"jsonrpc":"2.0","method":"initialize","params":{},"id":1}'; 
 sleep 0.5; 
 echo '{"jsonrpc":"2.0","method":"tools/call","params":{"name":"get_categories"},"id":2}') | \
./target/release/splitwise-mcp 2>/dev/null | \
grep '"id":2' | \
python3 -c "
import sys, json
data = json.load(sys.stdin)
categories = json.loads(data['result']['content'][0]['text'])
print('Available Splitwise Categories:')
print('=' * 40)
for c in categories:
    # Each category has an associated icon
    print(f\"ID {c['id']:3}: {c['name']:<30}\")
"