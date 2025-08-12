use splitwise_mcp_server::splitwise::SplitwiseClient;
use splitwise_mcp_server::types::ListExpensesParams;

#[tokio::main]
async fn main() {
    let api_key = std::env::var("SPLITWISE_API_KEY").unwrap();
    let client = SplitwiseClient::new(api_key);
    
    // Test fetching many expenses
    for offset in (0..600).step_by(100) {
        println!("Fetching offset {}...", offset);
        let params = ListExpensesParams {
            group_id: None,
            friend_id: None,
            dated_after: None,
            dated_before: None,
            updated_after: None,
            updated_before: None,
            limit: Some(100),
            offset: Some(offset),
        };
        
        match client.get_expenses(params).await {
            Ok(expenses) => println!("  Got {} expenses", expenses.len()),
            Err(e) => {
                println!("  ERROR at offset {}: {}", offset, e);
                break;
            }
        }
    }
}