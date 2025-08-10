use anyhow::Result;
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;

use crate::splitwise::SplitwiseClient;
use crate::types::*;

pub struct SplitwiseTools {
    client: Arc<SplitwiseClient>,
}

impl SplitwiseTools {
    pub fn new(client: Arc<SplitwiseClient>) -> Self {
        Self { client }
    }

    pub fn get_tools(&self) -> Vec<Value> {
        vec![
            // User tools
            json!({
                "name": "get_current_user",
                "description": "Get information about the currently authenticated user",
                "inputSchema": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            }),
            json!({
                "name": "get_user",
                "description": "Get information about a specific user by ID",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "user_id": {
                            "type": "integer",
                            "description": "The ID of the user to retrieve"
                        }
                    },
                    "required": ["user_id"]
                }
            }),
            // Group tools
            json!({
                "name": "list_groups",
                "description": "List all groups the current user belongs to",
                "inputSchema": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            }),
            json!({
                "name": "get_group",
                "description": "Get detailed information about a specific group",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "group_id": {
                            "type": "integer",
                            "description": "The ID of the group to retrieve"
                        }
                    },
                    "required": ["group_id"]
                }
            }),
            json!({
                "name": "create_group",
                "description": "Create a new group",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "Name of the group"
                        },
                        "group_type": {
                            "type": "string",
                            "enum": ["home", "trip", "couple", "other"],
                            "description": "Type of group (default: other)"
                        },
                        "simplify_by_default": {
                            "type": "boolean",
                            "description": "Whether to simplify debts by default"
                        }
                    },
                    "required": ["name"]
                }
            }),
            // Expense tools
            json!({
                "name": "list_expenses",
                "description": "List expenses with optional filters",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "group_id": {
                            "type": "integer",
                            "description": "Filter by group ID"
                        },
                        "friend_id": {
                            "type": "integer",
                            "description": "Filter by friend ID"
                        },
                        "dated_after": {
                            "type": "string",
                            "description": "Filter expenses after this date (YYYY-MM-DD)"
                        },
                        "dated_before": {
                            "type": "string",
                            "description": "Filter expenses before this date (YYYY-MM-DD)"
                        },
                        "limit": {
                            "type": "integer",
                            "description": "Maximum number of expenses to return"
                        },
                        "offset": {
                            "type": "integer",
                            "description": "Number of expenses to skip"
                        },
                        "fields": {
                            "type": "array",
                            "description": "Fields to include (REQUIRED). Common: id, description, cost, currency_code, date, category, payment, group_id. All available: id, description, cost, currency_code, date, category (id & name), payment (true if payment/settlement), group_id (null if personal), friendship_id (for non-group expenses), details (notes), users (array with paid_share, owed_share, net_balance per user), repayments (simplified debt flows), created_at, created_by, updated_at, updated_by, deleted_at (when deleted), deleted_by, receipt (image URLs), comments_count, transaction_confirmed (for integrated payments), transaction_id, transaction_method, transaction_status, repeats, repeat_interval (weekly/monthly/yearly), next_repeat, email_reminder, email_reminder_in_advance, expense_bundle_id",
                            "items": {
                                "type": "string"
                            }
                        },
                        "search_text": {
                            "type": "string",
                            "description": "Text to search for (case-insensitive substring match)"
                        },
                        "search_fields": {
                            "type": "array",
                            "description": "Fields to search in. Options: description, details, category. If omitted when search_text is provided, searches all fields",
                            "items": {
                                "type": "string"
                            }
                        },
                        "category_ids": {
                            "type": "array",
                            "description": "Filter by specific category IDs (e.g., [12] for Alimentos, [18] for General, or [12, 18] for both)",
                            "items": {
                                "type": "integer"
                            }
                        },
                        "include_deleted": {
                            "type": "string",
                            "description": "Control deleted expense filtering: 'exclude' (default), 'include' (show all), or 'only' (show only deleted)",
                            "enum": ["exclude", "include", "only"]
                        }
                    },
                    "required": ["fields"]
                }
            }),
            json!({
                "name": "get_expense",
                "description": "Get detailed information about a specific expense",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "expense_id": {
                            "type": "integer",
                            "description": "The ID of the expense to retrieve"
                        },
                        "fields": {
                            "type": "array",
                            "items": {
                                "type": "string"
                            },
                            "description": "Fields to include (REQUIRED). Available: id, description, cost, currency_code, date, category, payment, group_id, friendship_id, details, users, repayments, created_at, created_by, updated_at, updated_by, deleted_at, deleted_by, receipt, comments_count, transaction_confirmed, transaction_id, transaction_method, transaction_status, repeats, repeat_interval, next_repeat, email_reminder, email_reminder_in_advance, expense_bundle_id"
                        }
                    },
                    "required": ["expense_id", "fields"]
                }
            }),
            json!({
                "name": "create_expense",
                "description": "Create a new expense. IMPORTANT: Always call get_categories first to choose the most appropriate category/subcategory ID for the expense type. Categories determine the icon shown in Splitwise.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "cost": {
                            "type": "string",
                            "description": "Total cost of the expense (e.g., '25.00')"
                        },
                        "description": {
                            "type": "string",
                            "description": "Description of the expense"
                        },
                        "currency_code": {
                            "type": "string",
                            "description": "Currency code (e.g., 'USD', 'EUR')"
                        },
                        "group_id": {
                            "type": "integer",
                            "description": "Group ID to add expense to"
                        },
                        "split_equally": {
                            "type": "boolean",
                            "description": "Whether to split equally among all group members. Default: true. Set to false when using split_by_shares."
                        },
                        "split_by_shares": {
                            "type": "array",
                            "description": "Custom split amounts. Each entry specifies a user and their paid/owed amounts. Use this for unequal splits or when multiple people pay.",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "user_id": {
                                        "type": "integer",
                                        "description": "User ID (get from list_friends or get_group)"
                                    },
                                    "email": {
                                        "type": "string",
                                        "description": "User email (alternative to user_id)"
                                    },
                                    "paid_share": {
                                        "type": "string",
                                        "description": "Amount this user paid (e.g., '50.00')"
                                    },
                                    "owed_share": {
                                        "type": "string",
                                        "description": "Amount this user owes (e.g., '25.00')"
                                    }
                                },
                                "required": ["paid_share", "owed_share"]
                            }
                        },
                        "date": {
                            "type": "string",
                            "description": "Date of the expense (YYYY-MM-DD)"
                        },
                        "category_id": {
                            "type": "integer",
                            "description": "Category or subcategory ID from get_categories. Use the most specific subcategory when possible (e.g., 13 for Restaurants instead of 25 for Food). Required for proper icon display."
                        },
                        "details": {
                            "type": "string",
                            "description": "Additional details about the expense"
                        }
                    },
                    "required": ["cost", "description"]
                }
            }),
            json!({
                "name": "update_expense",
                "description": "Update an existing expense",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "expense_id": {
                            "type": "integer",
                            "description": "The ID of the expense to update"
                        },
                        "cost": {
                            "type": "string",
                            "description": "New total cost of the expense"
                        },
                        "description": {
                            "type": "string",
                            "description": "New description of the expense"
                        },
                        "currency_code": {
                            "type": "string",
                            "description": "New currency code"
                        },
                        "category_id": {
                            "type": "integer",
                            "description": "Category or subcategory ID from get_categories"
                        },
                        "date": {
                            "type": "string",
                            "description": "New date (YYYY-MM-DD)"
                        }
                    },
                    "required": ["expense_id"]
                }
            }),
            json!({
                "name": "delete_expense",
                "description": "Delete an expense",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "expense_id": {
                            "type": "integer",
                            "description": "The ID of the expense to delete"
                        }
                    },
                    "required": ["expense_id"]
                }
            }),
            // Friend tools
            json!({
                "name": "list_friends",
                "description": "List all friends and their balances",
                "inputSchema": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            }),
            json!({
                "name": "get_friend",
                "description": "Get detailed information about a specific friend",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "friend_id": {
                            "type": "integer",
                            "description": "The user ID of the friend"
                        }
                    },
                    "required": ["friend_id"]
                }
            }),
            json!({
                "name": "add_friend",
                "description": "Add a new friend by email",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "email": {
                            "type": "string",
                            "description": "Email address of the friend to add"
                        }
                    },
                    "required": ["email"]
                }
            }),
            // Utility tools
            json!({
                "name": "get_currencies",
                "description": "Get list of supported currencies",
                "inputSchema": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            }),
            json!({
                "name": "get_categories",
                "description": "Get list of expense categories with their IDs. Each category has an associated icon in Splitwise (e.g., 25=Food has a restaurant icon, 31=Transportation has a car icon)",
                "inputSchema": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            }),
        ]
    }

    pub async fn handle_tool_call(&self, name: &str, arguments: Option<Value>) -> Result<Value> {
        let arguments = arguments.unwrap_or_else(|| json!({}));
        
        match name {
            // User tools
            "get_current_user" => {
                let user = self.client.get_current_user().await?;
                Ok(serde_json::to_value(user)?)
            }
            "get_user" => {
                #[derive(Deserialize)]
                struct Args {
                    user_id: i64,
                }
                let args: Args = serde_json::from_value(arguments)?;
                let user = self.client.get_user(args.user_id).await?;
                Ok(serde_json::to_value(user)?)
            }
            // Group tools
            "list_groups" => {
                let groups = self.client.get_groups().await?;
                Ok(serde_json::to_value(groups)?)
            }
            "get_group" => {
                #[derive(Deserialize)]
                struct Args {
                    group_id: i64,
                }
                let args: Args = serde_json::from_value(arguments)?;
                let group = self.client.get_group(args.group_id).await?;
                Ok(serde_json::to_value(group)?)
            }
            "create_group" => {
                #[derive(Deserialize)]
                struct Args {
                    name: String,
                    group_type: Option<String>,
                    simplify_by_default: Option<bool>,
                }
                let args: Args = serde_json::from_value(arguments)?;
                let request = CreateGroupRequest {
                    name: args.name,
                    group_type: args.group_type,
                    simplify_by_default: args.simplify_by_default,
                    users: vec![], // Current user is added automatically
                };
                let group = self.client.create_group(request).await?;
                Ok(serde_json::to_value(group)?)
            }
            // Expense tools
            "list_expenses" => {
                #[derive(Deserialize)]
                struct Args {
                    group_id: Option<i64>,
                    friend_id: Option<i64>,
                    dated_after: Option<String>,
                    dated_before: Option<String>,
                    limit: Option<i32>,
                    offset: Option<i32>,
                    fields: Vec<String>,  // Now required
                    search_text: Option<String>,
                    search_fields: Option<Vec<String>>,
                    category_ids: Option<Vec<i64>>,
                    include_deleted: Option<String>,
                }
                let args: Args = serde_json::from_value(arguments)?;
                
                // Default to excluding deleted expenses
                let include_deleted = args.include_deleted.as_deref().unwrap_or("exclude");
                
                let mut expenses = Vec::new();
                
                // If searching or filtering by category, fetch in batches until we have enough matches
                if args.search_text.is_some() || args.category_ids.is_some() {
                    let search_lower = args.search_text.as_ref().map(|s| s.to_lowercase());
                    let search_fields = args.search_fields.clone().unwrap_or_else(|| {
                        vec!["description".to_string(), "details".to_string(), "category".to_string()]
                    });
                    
                    let desired_count = args.limit.map(|l| l as usize);
                    let batch_size = 100;
                    let mut current_offset = args.offset.unwrap_or(0);
                    
                    // Keep fetching batches until we have enough matches (if limit set) or run out of expenses
                    loop {
                        // If we have a limit and reached it, stop
                        if let Some(limit) = desired_count {
                            if expenses.len() >= limit {
                                break;
                            }
                        }
                        let params = ListExpensesParams {
                            group_id: args.group_id,
                            friend_id: args.friend_id,
                            dated_after: args.dated_after.clone(),
                            dated_before: args.dated_before.clone(),
                            updated_after: None,
                            updated_before: None,
                            limit: Some(batch_size),
                            offset: Some(current_offset),
                        };
                        
                        let mut batch = self.client.get_expenses(params.clone()).await
                            .map_err(|e| anyhow::anyhow!("Failed to fetch batch at offset {}: {}", current_offset, e))?;
                        
                        // Store the original batch size to check if we've reached the end
                        let batch_had_results = !batch.is_empty();
                        
                        // Filter this batch
                        batch.retain(|expense| {
                            // Handle deleted expense filtering
                            match include_deleted {
                                "exclude" => {
                                    if expense.deleted_at.is_some() {
                                        return false;
                                    }
                                },
                                "only" => {
                                    if expense.deleted_at.is_none() {
                                        return false;
                                    }
                                },
                                "include" => {
                                    // Include all expenses regardless of deleted status
                                },
                                _ => {
                                    // Default to exclude if somehow invalid value
                                    if expense.deleted_at.is_some() {
                                        return false;
                                    }
                                }
                            }
                            
                            // Check category filter first
                            if let Some(ref category_ids) = args.category_ids {
                                if !category_ids.contains(&expense.category.id) {
                                    return false;
                                }
                            }
                            
                            // Then check text search if present
                            if let Some(ref search_lower) = search_lower {
                                for field in &search_fields {
                                    match field.as_str() {
                                        "description" => {
                                            if expense.description.to_lowercase().contains(search_lower) {
                                                return true;
                                            }
                                        },
                                        "details" => {
                                            if expense.details.as_ref().map_or(false, |d| d.to_lowercase().contains(search_lower)) {
                                                return true;
                                            }
                                        },
                                        "category" => {
                                            if expense.category.name.to_lowercase().contains(search_lower) {
                                                return true;
                                            }
                                        },
                                        _ => {}
                                    }
                                }
                                // If search text was provided but no match found, exclude this expense
                                return false;
                            }
                            
                            // If no search text but category matched (or no filters), include it
                            true
                        });
                        
                        // Add matches to our results
                        for expense in batch {
                            expenses.push(expense);
                            if let Some(limit) = desired_count {
                                if expenses.len() >= limit {
                                    break;
                                }
                            }
                        }
                        
                        // If the original batch was empty, we've reached the end
                        if !batch_had_results {
                            break;
                        }
                        
                        current_offset += batch_size;
                    }
                    
                    // Truncate to requested limit if there is one
                    if let Some(limit) = desired_count {
                        expenses.truncate(limit);
                    }
                } else {
                    // No search or category filter, but still need to handle deleted filtering properly with limit
                    
                    // If we're filtering deleted expenses AND have a limit, we need to fetch in batches
                    // to ensure we get enough non-deleted results
                    if include_deleted != "include" && args.limit.is_some() {
                        let desired_count = args.limit.map(|l| l as usize);
                        let batch_size = 100;
                        let mut current_offset = args.offset.unwrap_or(0);
                        
                        loop {
                            // If we have a limit and reached it, stop
                            if let Some(limit) = desired_count {
                                if expenses.len() >= limit {
                                    break;
                                }
                            }
                            
                            let params = ListExpensesParams {
                                group_id: args.group_id,
                                friend_id: args.friend_id,
                                dated_after: args.dated_after.clone(),
                                dated_before: args.dated_before.clone(),
                                updated_after: None,
                                updated_before: None,
                                limit: Some(batch_size),
                                offset: Some(current_offset),
                            };
                            
                            let mut batch = self.client.get_expenses(params).await?;
                            let batch_had_results = !batch.is_empty();
                            
                            // Apply deleted expense filtering
                            match include_deleted {
                                "exclude" => {
                                    batch.retain(|expense| expense.deleted_at.is_none());
                                },
                                "only" => {
                                    batch.retain(|expense| expense.deleted_at.is_some());
                                },
                                _ => {
                                    // Default to exclude
                                    batch.retain(|expense| expense.deleted_at.is_none());
                                }
                            }
                            
                            // Add filtered results
                            for expense in batch {
                                expenses.push(expense);
                                if let Some(limit) = desired_count {
                                    if expenses.len() >= limit {
                                        break;
                                    }
                                }
                            }
                            
                            // If the original batch was empty, we've reached the end
                            if !batch_had_results {
                                break;
                            }
                            
                            current_offset += batch_size;
                        }
                        
                        // Truncate to requested limit if there is one
                        if let Some(limit) = desired_count {
                            expenses.truncate(limit);
                        }
                    } else {
                        // Simple case: include all deleted or no limit specified
                        let params = ListExpensesParams {
                            group_id: args.group_id,
                            friend_id: args.friend_id,
                            dated_after: args.dated_after,
                            dated_before: args.dated_before,
                            updated_after: None,
                            updated_before: None,
                            limit: args.limit,
                            offset: args.offset,
                        };
                        expenses = self.client.get_expenses(params).await?;
                        
                        // Apply deleted expense filtering if not including all
                        if include_deleted != "include" {
                            match include_deleted {
                                "exclude" => {
                                    expenses.retain(|expense| expense.deleted_at.is_none());
                                },
                                "only" => {
                                    expenses.retain(|expense| expense.deleted_at.is_some());
                                },
                                _ => {
                                    // Default to exclude
                                    expenses.retain(|expense| expense.deleted_at.is_none());
                                }
                            }
                        }
                    }
                }
                
                // Filter to requested fields
                let filtered: Vec<serde_json::Value> = expenses.into_iter().map(|exp| {
                    let mut obj = serde_json::Map::new();
                    for field in &args.fields {
                        match field.as_str() {
                            "id" => { obj.insert("id".to_string(), json!(exp.id)); },
                            "description" => { obj.insert("description".to_string(), json!(exp.description)); },
                            "cost" => { obj.insert("cost".to_string(), json!(exp.cost)); },
                            "currency_code" => { obj.insert("currency_code".to_string(), json!(exp.currency_code)); },
                            "date" => { obj.insert("date".to_string(), json!(exp.date)); },
                            "category" => { 
                                obj.insert("category".to_string(), json!({"id": exp.category.id, "name": exp.category.name}));
                            },
                            "payment" => { obj.insert("payment".to_string(), json!(exp.payment)); },
                            "group_id" => { obj.insert("group_id".to_string(), json!(exp.group_id)); },
                            "friendship_id" => { obj.insert("friendship_id".to_string(), json!(exp.friendship_id)); },
                            "details" => { obj.insert("details".to_string(), json!(exp.details)); },
                            "users" => { obj.insert("users".to_string(), json!(exp.users)); },
                            "repayments" => { obj.insert("repayments".to_string(), json!(exp.repayments)); },
                            "created_at" => { obj.insert("created_at".to_string(), json!(exp.created_at)); },
                            "created_by" => { obj.insert("created_by".to_string(), json!(exp.created_by)); },
                            "updated_at" => { obj.insert("updated_at".to_string(), json!(exp.updated_at)); },
                            "updated_by" => { obj.insert("updated_by".to_string(), json!(exp.updated_by)); },
                            "deleted_at" => { 
                                if exp.deleted_at.is_some() {
                                    obj.insert("deleted_at".to_string(), json!(exp.deleted_at));
                                }
                            },
                            "deleted_by" => { 
                                if exp.deleted_by.is_some() {
                                    obj.insert("deleted_by".to_string(), json!(exp.deleted_by));
                                }
                            },
                            "receipt" => { obj.insert("receipt".to_string(), json!(exp.receipt)); },
                            "comments_count" => { obj.insert("comments_count".to_string(), json!(exp.comments_count)); },
                            "transaction_confirmed" => { obj.insert("transaction_confirmed".to_string(), json!(exp.transaction_confirmed)); },
                            "transaction_id" => { obj.insert("transaction_id".to_string(), json!(exp.transaction_id)); },
                            "transaction_method" => { obj.insert("transaction_method".to_string(), json!(exp.transaction_method)); },
                            "transaction_status" => { obj.insert("transaction_status".to_string(), json!(exp.transaction_status)); },
                            "repeats" => { obj.insert("repeats".to_string(), json!(exp.repeats)); },
                            "repeat_interval" => { obj.insert("repeat_interval".to_string(), json!(exp.repeat_interval)); },
                            "next_repeat" => { obj.insert("next_repeat".to_string(), json!(exp.next_repeat)); },
                            "email_reminder" => { obj.insert("email_reminder".to_string(), json!(exp.email_reminder)); },
                            "email_reminder_in_advance" => { obj.insert("email_reminder_in_advance".to_string(), json!(exp.email_reminder_in_advance)); },
                            "expense_bundle_id" => { obj.insert("expense_bundle_id".to_string(), json!(exp.expense_bundle_id)); },
                            _ => {}
                        }
                    }
                    serde_json::Value::Object(obj)
                }).collect();
                Ok(serde_json::Value::Array(filtered))
            }
            "get_expense" => {
                #[derive(Deserialize)]
                struct Args {
                    expense_id: i64,
                    fields: Vec<String>,  // Now required
                }
                let args: Args = serde_json::from_value(arguments)?;
                let expense = self.client.get_expense(args.expense_id).await?;
                
                // Filter to requested fields
                let mut obj = serde_json::Map::new();
                for field in &args.fields {
                    match field.as_str() {
                            "id" => { obj.insert("id".to_string(), json!(expense.id)); },
                            "description" => { obj.insert("description".to_string(), json!(expense.description)); },
                            "cost" => { obj.insert("cost".to_string(), json!(expense.cost)); },
                            "currency_code" => { obj.insert("currency_code".to_string(), json!(expense.currency_code)); },
                            "date" => { obj.insert("date".to_string(), json!(expense.date)); },
                            "category" => { 
                                obj.insert("category".to_string(), json!({"id": expense.category.id, "name": expense.category.name}));
                            },
                            "payment" => { obj.insert("payment".to_string(), json!(expense.payment)); },
                            "group_id" => { obj.insert("group_id".to_string(), json!(expense.group_id)); },
                            "friendship_id" => { obj.insert("friendship_id".to_string(), json!(expense.friendship_id)); },
                            "details" => { obj.insert("details".to_string(), json!(expense.details)); },
                            "users" => { obj.insert("users".to_string(), json!(expense.users)); },
                            "repayments" => { obj.insert("repayments".to_string(), json!(expense.repayments)); },
                            "created_at" => { obj.insert("created_at".to_string(), json!(expense.created_at)); },
                            "created_by" => { obj.insert("created_by".to_string(), json!(expense.created_by)); },
                            "updated_at" => { obj.insert("updated_at".to_string(), json!(expense.updated_at)); },
                            "updated_by" => { obj.insert("updated_by".to_string(), json!(expense.updated_by)); },
                            "deleted_at" => { 
                                if expense.deleted_at.is_some() {
                                    obj.insert("deleted_at".to_string(), json!(expense.deleted_at));
                                }
                            },
                            "deleted_by" => { 
                                if expense.deleted_by.is_some() {
                                    obj.insert("deleted_by".to_string(), json!(expense.deleted_by));
                                }
                            },
                            "receipt" => { obj.insert("receipt".to_string(), json!(expense.receipt)); },
                            "comments_count" => { obj.insert("comments_count".to_string(), json!(expense.comments_count)); },
                            "transaction_confirmed" => { obj.insert("transaction_confirmed".to_string(), json!(expense.transaction_confirmed)); },
                            "transaction_id" => { obj.insert("transaction_id".to_string(), json!(expense.transaction_id)); },
                            "transaction_method" => { obj.insert("transaction_method".to_string(), json!(expense.transaction_method)); },
                            "transaction_status" => { obj.insert("transaction_status".to_string(), json!(expense.transaction_status)); },
                            "repeats" => { obj.insert("repeats".to_string(), json!(expense.repeats)); },
                            "repeat_interval" => { obj.insert("repeat_interval".to_string(), json!(expense.repeat_interval)); },
                            "next_repeat" => { obj.insert("next_repeat".to_string(), json!(expense.next_repeat)); },
                            "email_reminder" => { obj.insert("email_reminder".to_string(), json!(expense.email_reminder)); },
                            "email_reminder_in_advance" => { obj.insert("email_reminder_in_advance".to_string(), json!(expense.email_reminder_in_advance)); },
                            "expense_bundle_id" => { obj.insert("expense_bundle_id".to_string(), json!(expense.expense_bundle_id)); },
                            _ => {}
                    }
                }
                Ok(serde_json::Value::Object(obj))
            }
            "create_expense" => {
                #[derive(Deserialize)]
                struct ShareInput {
                    user_id: Option<i64>,
                    email: Option<String>,
                    first_name: Option<String>,
                    last_name: Option<String>,
                    paid_share: String,
                    owed_share: String,
                }
                
                #[derive(Deserialize)]
                struct Args {
                    cost: String,
                    description: String,
                    currency_code: Option<String>,
                    group_id: Option<i64>,
                    split_equally: Option<bool>,
                    split_by_shares: Option<Vec<ShareInput>>,
                    date: Option<String>,
                    category_id: Option<i64>,
                    details: Option<String>,
                }
                let args: Args = serde_json::from_value(arguments)?;
                
                // Convert ShareInput to ExpenseShare
                let split_by_shares = args.split_by_shares.map(|shares| {
                    shares.into_iter().map(|s| ExpenseShare {
                        user_id: s.user_id,
                        email: s.email,
                        first_name: s.first_name,
                        last_name: s.last_name,
                        paid_share: s.paid_share,
                        owed_share: s.owed_share,
                    }).collect()
                });
                
                // If shares are provided, split_equally should be false
                let split_equally = if split_by_shares.is_some() {
                    Some(false)
                } else {
                    args.split_equally.or(Some(true))
                };
                
                let request = CreateExpenseRequest {
                    cost: args.cost,
                    description: args.description,
                    currency_code: args.currency_code,
                    category_id: args.category_id,
                    date: args.date,
                    repeat_interval: None,
                    details: args.details,
                    payment: Some(false),
                    group_id: args.group_id,
                    split_equally,
                    split_by_shares,
                };
                let expenses = self.client.create_expense(request).await?;
                Ok(serde_json::to_value(expenses)?)
            }
            "update_expense" => {
                #[derive(Deserialize)]
                struct Args {
                    expense_id: i64,
                    cost: Option<String>,
                    description: Option<String>,
                    currency_code: Option<String>,
                    category_id: Option<i64>,
                    date: Option<String>,
                    split_equally: Option<bool>,
                    split_by_shares: Option<Vec<ExpenseShare>>,
                }
                let args: Args = serde_json::from_value(arguments)?;
                let request = UpdateExpenseRequest {
                    cost: args.cost,
                    description: args.description,
                    currency_code: args.currency_code,
                    category_id: args.category_id,
                    date: args.date,
                    details: None,
                    payment: None,
                    group_id: None,
                    split_equally: args.split_equally,
                    split_by_shares: args.split_by_shares,
                };
                let expenses = self.client.update_expense(args.expense_id, request).await?;
                Ok(serde_json::to_value(expenses)?)
            }
            "delete_expense" => {
                #[derive(Deserialize)]
                struct Args {
                    expense_id: i64,
                }
                let args: Args = serde_json::from_value(arguments)?;
                let success = self.client.delete_expense(args.expense_id).await?;
                Ok(json!({ "success": success }))
            }
            // Friend tools
            "list_friends" => {
                let friends = self.client.get_friends().await?;
                Ok(serde_json::to_value(friends)?)
            }
            "get_friend" => {
                #[derive(Deserialize)]
                struct Args {
                    friend_id: i64,
                }
                let args: Args = serde_json::from_value(arguments)?;
                let friend = self.client.get_friend(args.friend_id).await?;
                Ok(serde_json::to_value(friend)?)
            }
            "add_friend" => {
                #[derive(Deserialize)]
                struct Args {
                    email: String,
                }
                let args: Args = serde_json::from_value(arguments)?;
                let friends = self.client.create_friend(args.email).await?;
                Ok(serde_json::to_value(friends)?)
            }
            // Utility tools
            "get_currencies" => {
                let currencies = self.client.get_currencies().await?;
                Ok(serde_json::to_value(currencies)?)
            }
            "get_categories" => {
                let categories = self.client.get_categories().await?;
                Ok(serde_json::to_value(categories)?)
            }
            _ => Err(anyhow::anyhow!("Unknown tool: {}", name)),
        }
    }
}