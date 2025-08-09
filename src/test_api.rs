use anyhow::Result;
use dotenv::dotenv;
use std::env;

mod types;
mod splitwise;

// Import our Splitwise client
use splitwise::SplitwiseClient;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();

    // Get API key from environment
    let api_key = env::var("SPLITWISE_API_KEY")
        .expect("SPLITWISE_API_KEY environment variable not set");

    println!("Testing Splitwise API with key: {}...", &api_key[..4]);

    // Create Splitwise client
    let client = SplitwiseClient::new(api_key)?;

    // Test 1: Get current user
    println!("\n1. Testing get_current_user...");
    match client.get_current_user().await {
        Ok(user) => {
            println!("✅ Current user: {} {} ({})", 
                user.first_name, 
                user.last_name.as_deref().unwrap_or(""),
                user.email
            );
        }
        Err(e) => println!("❌ Failed to get current user: {}", e),
    }

    // Test 2: List groups
    println!("\n2. Testing list_groups...");
    match client.get_groups().await {
        Ok(groups) => {
            println!("✅ Found {} groups:", groups.len());
            for group in groups.iter().take(3) {
                println!("   - {} (ID: {})", group.name, group.id);
            }
        }
        Err(e) => println!("❌ Failed to list groups: {}", e),
    }

    // Test 3: List friends
    println!("\n3. Testing list_friends...");
    match client.get_friends().await {
        Ok(friends) => {
            println!("✅ Found {} friends:", friends.len());
            for friend in friends.iter().take(3) {
                println!("   - {} {}", 
                    friend.first_name,
                    friend.last_name.as_deref().unwrap_or("")
                );
            }
        }
        Err(e) => println!("❌ Failed to list friends: {}", e),
    }

    // Test 4: List recent expenses
    println!("\n4. Testing list_expenses...");
    match client.get_expenses(types::ListExpensesParams {
        limit: Some(5),
        ..Default::default()
    }).await {
        Ok(expenses) => {
            println!("✅ Found {} recent expenses:", expenses.len());
            for expense in expenses.iter() {
                println!("   - {} ({} {})", 
                    expense.description,
                    expense.cost,
                    expense.currency_code
                );
            }
        }
        Err(e) => println!("❌ Failed to list expenses: {}", e),
    }

    // Test 5: Get currencies
    println!("\n5. Testing get_currencies...");
    match client.get_currencies().await {
        Ok(currencies) => {
            println!("✅ Found {} currencies", currencies.len());
            for curr in currencies.iter().take(5) {
                println!("   - {} ({})", curr.currency_code, curr.unit);
            }
        }
        Err(e) => println!("❌ Failed to get currencies: {}", e),
    }

    println!("\n✅ All tests completed!");
    Ok(())
}