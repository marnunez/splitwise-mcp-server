use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: String,
    pub registration_status: Option<String>,
    pub picture: Option<Picture>,
    pub default_currency: Option<String>,
    pub locale: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Picture {
    pub small: Option<String>,
    pub medium: Option<String>,
    pub large: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub group_type: Option<String>,
    pub updated_at: String,
    pub simplify_by_default: bool,
    pub members: Vec<GroupMember>,
    pub original_debts: Vec<Debt>,
    pub simplified_debts: Vec<Debt>,
    pub whiteboard: Option<serde_json::Value>,
    pub group_reminders: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMember {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub registration_status: Option<String>,
    pub picture: Option<Picture>,
    pub balance: Vec<Balance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub currency_code: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Debt {
    pub from: i64,
    pub to: i64,
    pub amount: String,
    pub currency_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: i64,
    pub group_id: Option<i64>,
    pub friendship_id: Option<i64>,
    pub expense_bundle_id: Option<i64>,
    pub description: String,
    pub repeats: bool,
    pub repeat_interval: Option<String>,
    pub email_reminder: bool,
    pub email_reminder_in_advance: Option<i32>,
    pub next_repeat: Option<String>,
    pub details: Option<String>,
    pub comments_count: i32,
    pub payment: bool,
    pub creation_method: Option<String>,
    pub transaction_method: Option<String>,
    pub transaction_confirmed: bool,
    pub transaction_id: Option<String>,
    pub transaction_status: Option<String>,
    pub cost: String,
    pub currency_code: String,
    pub repayments: Vec<Repayment>,
    pub date: String,
    pub created_at: String,
    pub created_by: UserReference,
    pub updated_at: String,
    pub updated_by: Option<UserReference>,
    pub deleted_at: Option<String>,
    pub deleted_by: Option<UserReference>,
    pub category: Category,
    pub receipt: Receipt,
    pub users: Vec<ExpenseUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserReference {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub picture: Option<Picture>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub icon: Option<String>,
    pub subcategories: Option<Vec<Subcategory>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subcategory {
    pub id: i64,
    pub name: String,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub original: Option<String>,
    pub large: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseUser {
    pub user_id: i64,
    pub user: Option<UserReference>,
    pub paid_share: String,
    pub owed_share: String,
    pub net_balance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repayment {
    pub from: i64,
    pub to: i64,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Friend {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub registration_status: Option<String>,
    pub picture: Option<Picture>,
    pub balance: Vec<Balance>,
    pub groups: Vec<FriendGroup>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendGroup {
    pub group_id: i64,
    pub balance: Vec<Balance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    pub currency_code: String,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExpenseRequest {
    pub cost: String,
    pub description: String,
    pub currency_code: Option<String>,
    pub category_id: Option<i64>,
    pub date: Option<String>,
    pub repeat_interval: Option<String>,
    pub details: Option<String>,
    pub payment: Option<bool>,
    pub group_id: Option<i64>,
    pub split_equally: Option<bool>,
    pub split_by_shares: Option<Vec<ExpenseShare>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateExpenseRequest {
    pub cost: Option<String>,
    pub description: Option<String>,
    pub currency_code: Option<String>,
    pub category_id: Option<i64>,
    pub date: Option<String>,
    pub details: Option<String>,
    pub payment: Option<bool>,
    pub group_id: Option<i64>,
    pub split_equally: Option<bool>,
    pub split_by_shares: Option<Vec<ExpenseShare>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseShare {
    pub user_id: Option<i64>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub paid_share: String,
    pub owed_share: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
    pub group_type: Option<String>,
    pub simplify_by_default: Option<bool>,
    pub users: Vec<GroupUserInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupUserInput {
    pub user_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub locale: Option<String>,
    pub default_currency: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListExpensesParams {
    pub group_id: Option<i64>,
    pub friend_id: Option<i64>,
    pub dated_after: Option<String>,
    pub dated_before: Option<String>,
    pub updated_after: Option<String>,
    pub updated_before: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub errors: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub errors: Option<Vec<String>>,
}