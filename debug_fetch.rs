use splitwise_mcp_server::splitwise::SplitwiseClient;
use splitwise_mcp_server::types::ListExpensesParams;

#[tokio::main]
async fn main() {
    let api_key = std::env::var("SPLITWISE_API_KEY").unwrap();
    let client = SplitwiseClient::new(api_key);
    
    println!("Fetching offset 2500...");
    let params = ListExpensesParams {
        group_id: None,
        friend_id: None,
        dated_after: None,
        dated_before: None,
        updated_after: None,
        updated_before: None,
        limit: Some(100),
        offset: Some(2500),
    };
    
    match client.get_expenses(params).await {
        Ok(expenses) => {
            println!("Success! Got {} expenses", expenses.len());
            // Print first expense to see structure
            if let Some(first) = expenses.first() {
                println!("First expense: ID={}, desc={}", first.id, first.description);
            }
        }
        Err(e) => {
            println!("ERROR: {:#}", e);
            // Try to get raw response
            println!("\nTrying raw request...");
            let client = reqwest::Client::new();
            let resp = client
                .get("https://api.splitwise.com/api/v3.0/get_expenses")
                .query(&[("limit", "100"), ("offset", "2500")])
                .header("Authorization", format!("Bearer {}", api_key))
                .send()
                .await
                .unwrap();
            
            let status = resp.status();
            let text = resp.text().await.unwrap();
            println!("Status: {}", status);
            println!("Response length: {}", text.len());
            
            // Save to file for analysis
            std::fs::write("/tmp/response_2500.json", &text).unwrap();
            println!("Saved to /tmp/response_2500.json");
            
            // Try to parse as JSON to see where it fails
            match serde_json::from_str::<serde_json::Value>(&text) {
                Ok(_) => println!("JSON is valid!"),
                Err(e) => println!("JSON parse error: {}", e),
            }
        }
    }
}