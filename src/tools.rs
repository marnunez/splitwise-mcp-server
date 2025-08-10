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
                            "description": "Fields to include in response. If omitted, returns all fields. Options: id, description, cost, date, category, deleted_at, group_id",
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
                        }
                    },
                    "required": []
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
                            "description": "Fields to include in response. If omitted, returns all fields. Options: id, description, cost, date, category, deleted_at, group_id"
                        }
                    },
                    "required": ["expense_id"]
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
                    fields: Option<Vec<String>>,
                    search_text: Option<String>,
                    search_fields: Option<Vec<String>>,
                }
                let args: Args = serde_json::from_value(arguments)?;
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
                let mut expenses = self.client.get_expenses(params).await?;
                
                // Filter by search text if specified
                if let Some(search_text) = args.search_text {
                    let search_lower = search_text.to_lowercase();
                    let search_fields = args.search_fields.unwrap_or_else(|| {
                        vec!["description".to_string(), "details".to_string(), "category".to_string()]
                    });
                    
                    expenses.retain(|expense| {
                        for field in &search_fields {
                            match field.as_str() {
                                "description" => {
                                    if expense.description.to_lowercase().contains(&search_lower) {
                                        return true;
                                    }
                                },
                                "details" => {
                                    if expense.details.as_ref().map_or(false, |d| d.to_lowercase().contains(&search_lower)) {
                                        return true;
                                    }
                                },
                                "category" => {
                                    if expense.category.name.to_lowercase().contains(&search_lower) {
                                        return true;
                                    }
                                },
                                _ => {}
                            }
                        }
                        false
                    });
                }
                
                // If fields are specified, filter the response
                if let Some(fields) = args.fields {
                    let filtered: Vec<serde_json::Value> = expenses.into_iter().map(|exp| {
                        let mut obj = serde_json::Map::new();
                        for field in &fields {
                            match field.as_str() {
                                "id" => { obj.insert("id".to_string(), json!(exp.id)); },
                                "description" => { obj.insert("description".to_string(), json!(exp.description)); },
                                "cost" => { obj.insert("cost".to_string(), json!(exp.cost)); },
                                "date" => { obj.insert("date".to_string(), json!(exp.date)); },
                                "category" => { 
                                    obj.insert("category".to_string(), json!({"id": exp.category.id, "name": exp.category.name}));
                                },
                                "deleted_at" => { obj.insert("deleted_at".to_string(), json!(exp.deleted_at)); },
                                "group_id" => { obj.insert("group_id".to_string(), json!(exp.group_id)); },
                                _ => {}
                            }
                        }
                        serde_json::Value::Object(obj)
                    }).collect();
                    Ok(serde_json::Value::Array(filtered))
                } else {
                    Ok(serde_json::to_value(expenses)?)
                }
            }
            "get_expense" => {
                #[derive(Deserialize)]
                struct Args {
                    expense_id: i64,
                    fields: Option<Vec<String>>,
                }
                let args: Args = serde_json::from_value(arguments)?;
                let expense = self.client.get_expense(args.expense_id).await?;
                
                // If fields are specified, filter the response
                if let Some(fields) = args.fields {
                    let mut obj = serde_json::Map::new();
                    for field in &fields {
                        match field.as_str() {
                            "id" => { obj.insert("id".to_string(), json!(expense.id)); },
                            "description" => { obj.insert("description".to_string(), json!(expense.description)); },
                            "cost" => { obj.insert("cost".to_string(), json!(expense.cost)); },
                            "date" => { obj.insert("date".to_string(), json!(expense.date)); },
                            "category" => { 
                                obj.insert("category".to_string(), json!({"id": expense.category.id, "name": expense.category.name}));
                            },
                            "deleted_at" => { obj.insert("deleted_at".to_string(), json!(expense.deleted_at)); },
                            "group_id" => { obj.insert("group_id".to_string(), json!(expense.group_id)); },
                            _ => {}
                        }
                    }
                    Ok(serde_json::Value::Object(obj))
                } else {
                    Ok(serde_json::to_value(expense)?)
                }
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