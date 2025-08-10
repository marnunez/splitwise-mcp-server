use anyhow::{Context, Result};
use reqwest::{Client, Response};
use serde_json::json;
use std::collections::HashMap;

use crate::types::*;

const BASE_URL: &str = "https://secure.splitwise.com/api/v3.0";

pub struct SplitwiseClient {
    client: Client,
    api_key: String,
}

impl SplitwiseClient {
    pub fn new(api_key: String) -> Result<Self> {
        let client = Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    format!("Bearer {}", api_key).parse()?,
                );
                headers.insert(
                    reqwest::header::CONTENT_TYPE,
                    "application/json".parse()?,
                );
                headers
            })
            .build()?;

        Ok(Self { client, api_key })
    }

    async fn get<T: for<'de> serde::Deserialize<'de>>(&self, endpoint: &str) -> Result<T> {
        let url = format!("{}{}", BASE_URL, endpoint);
        let response = self.client.get(&url).send().await?;
        self.handle_response(response).await
    }

    async fn get_with_params<T: for<'de> serde::Deserialize<'de>>(
        &self,
        endpoint: &str,
        params: &[(&str, String)],
    ) -> Result<T> {
        let url = format!("{}{}", BASE_URL, endpoint);
        let response = self.client.get(&url).query(params).send().await?;
        self.handle_response(response).await
    }

    async fn post<T: for<'de> serde::Deserialize<'de>>(
        &self,
        endpoint: &str,
        body: serde_json::Value,
    ) -> Result<T> {
        let url = format!("{}{}", BASE_URL, endpoint);
        let response = self.client.post(&url).json(&body).send().await?;
        self.handle_response(response).await
    }

    async fn delete<T: for<'de> serde::Deserialize<'de>>(&self, endpoint: &str) -> Result<T> {
        let url = format!("{}{}", BASE_URL, endpoint);
        let response = self.client.delete(&url).send().await?;
        self.handle_response(response).await
    }

    async fn handle_response<T: for<'de> serde::Deserialize<'de>>(
        &self,
        response: Response,
    ) -> Result<T> {
        let status = response.status();
        let text = response.text().await?;

        if status.is_success() {
            serde_json::from_str(&text).with_context(|| {
                format!("Failed to parse response. Status: {}, Length: {}, First 500 chars: {}", 
                    status, 
                    text.len(),
                    &text.chars().take(500).collect::<String>())
            })
        } else {
            let error: ApiError = serde_json::from_str(&text).unwrap_or_else(|_| ApiError {
                errors: {
                    let mut map = HashMap::new();
                    map.insert("base".to_string(), vec![text.clone()]);
                    map
                },
            });
            anyhow::bail!("API error ({}): {:?}", status, error.errors)
        }
    }

    // User endpoints
    pub async fn get_current_user(&self) -> Result<User> {
        #[derive(serde::Deserialize)]
        struct Response {
            user: User,
        }
        let response: Response = self.get("/get_current_user").await?;
        Ok(response.user)
    }

    pub async fn get_user(&self, id: i64) -> Result<User> {
        #[derive(serde::Deserialize)]
        struct Response {
            user: User,
        }
        let response: Response = self.get(&format!("/get_user/{}", id)).await?;
        Ok(response.user)
    }

    pub async fn update_user(&self, id: i64, update: UpdateUserRequest) -> Result<User> {
        let response: User = self
            .post(&format!("/update_user/{}", id), serde_json::to_value(update)?)
            .await?;
        Ok(response)
    }

    // Group endpoints
    pub async fn get_groups(&self) -> Result<Vec<Group>> {
        #[derive(serde::Deserialize)]
        struct Response {
            groups: Vec<Group>,
        }
        let response: Response = self.get("/get_groups").await?;
        Ok(response.groups)
    }

    pub async fn get_group(&self, id: i64) -> Result<Group> {
        #[derive(serde::Deserialize)]
        struct Response {
            group: Group,
        }
        let response: Response = self.get(&format!("/get_group/{}", id)).await?;
        Ok(response.group)
    }

    pub async fn create_group(&self, request: CreateGroupRequest) -> Result<Group> {
        // Convert the request to the flattened format expected by API
        let mut body = json!({
            "name": request.name,
        });

        if let Some(group_type) = request.group_type {
            body["group_type"] = json!(group_type);
        }
        if let Some(simplify) = request.simplify_by_default {
            body["simplify_by_default"] = json!(simplify);
        }

        // Flatten users into users__{index}__{property} format
        for (i, user) in request.users.iter().enumerate() {
            if let Some(user_id) = user.user_id {
                body[format!("users__{}__user_id", i)] = json!(user_id);
            }
            if let Some(ref first_name) = user.first_name {
                body[format!("users__{}__first_name", i)] = json!(first_name);
            }
            if let Some(ref last_name) = user.last_name {
                body[format!("users__{}__last_name", i)] = json!(last_name);
            }
            if let Some(ref email) = user.email {
                body[format!("users__{}__email", i)] = json!(email);
            }
        }

        #[derive(serde::Deserialize)]
        struct Response {
            group: Group,
        }
        let response: Response = self.post("/create_group", body).await?;
        Ok(response.group)
    }

    pub async fn delete_group(&self, id: i64) -> Result<bool> {
        let response: SuccessResponse = self
            .post(&format!("/delete_group/{}", id), json!({}))
            .await?;
        Ok(response.success)
    }

    pub async fn add_user_to_group(
        &self,
        group_id: i64,
        user: GroupUserInput,
    ) -> Result<User> {
        let mut body = json!({
            "group_id": group_id,
        });

        if let Some(user_id) = user.user_id {
            body["user_id"] = json!(user_id);
        } else {
            body["first_name"] = json!(user.first_name);
            body["last_name"] = json!(user.last_name);
            body["email"] = json!(user.email);
        }

        #[derive(serde::Deserialize)]
        struct Response {
            success: bool,
            user: Option<User>,
            errors: Option<serde_json::Value>,
        }
        let response: Response = self.post("/add_user_to_group", body).await?;
        
        if response.success {
            response.user.context("User not returned despite success")
        } else {
            anyhow::bail!("Failed to add user to group: {:?}", response.errors)
        }
    }

    pub async fn remove_user_from_group(&self, group_id: i64, user_id: i64) -> Result<bool> {
        let body = json!({
            "group_id": group_id,
            "user_id": user_id,
        });

        let response: SuccessResponse = self.post("/remove_user_from_group", body).await?;
        Ok(response.success)
    }

    // Expense endpoints
    pub async fn get_expenses(&self, params: ListExpensesParams) -> Result<Vec<Expense>> {
        let mut query_params = vec![];
        
        if let Some(group_id) = params.group_id {
            query_params.push(("group_id", group_id.to_string()));
        }
        if let Some(friend_id) = params.friend_id {
            query_params.push(("friend_id", friend_id.to_string()));
        }
        if let Some(ref dated_after) = params.dated_after {
            query_params.push(("dated_after", dated_after.clone()));
        }
        if let Some(ref dated_before) = params.dated_before {
            query_params.push(("dated_before", dated_before.clone()));
        }
        if let Some(ref updated_after) = params.updated_after {
            query_params.push(("updated_after", updated_after.clone()));
        }
        if let Some(ref updated_before) = params.updated_before {
            query_params.push(("updated_before", updated_before.clone()));
        }
        if let Some(limit) = params.limit {
            query_params.push(("limit", limit.to_string()));
        }
        if let Some(offset) = params.offset {
            query_params.push(("offset", offset.to_string()));
        }

        #[derive(serde::Deserialize)]
        struct Response {
            expenses: Vec<Expense>,
        }
        
        let response: Response = if query_params.is_empty() {
            self.get("/get_expenses").await?
        } else {
            self.get_with_params("/get_expenses", &query_params).await?
        };
        
        Ok(response.expenses)
    }

    pub async fn get_expense(&self, id: i64) -> Result<Expense> {
        #[derive(serde::Deserialize)]
        struct Response {
            expense: Expense,
        }
        let response: Response = self.get(&format!("/get_expense/{}", id)).await?;
        Ok(response.expense)
    }

    pub async fn create_expense(&self, request: CreateExpenseRequest) -> Result<Vec<Expense>> {
        let mut body = json!({
            "cost": request.cost,
            "description": request.description,
        });

        if let Some(currency_code) = request.currency_code {
            body["currency_code"] = json!(currency_code);
        }
        if let Some(category_id) = request.category_id {
            body["category_id"] = json!(category_id);
        }
        if let Some(date) = request.date {
            body["date"] = json!(date);
        }
        if let Some(details) = request.details {
            body["details"] = json!(details);
        }
        if let Some(payment) = request.payment {
            body["payment"] = json!(payment);
        }

        // Handle split type
        if let Some(group_id) = request.group_id {
            body["group_id"] = json!(group_id);
            if request.split_equally.unwrap_or(true) {
                body["split_equally"] = json!(true);
            }
        }

        // Handle custom shares
        if let Some(shares) = request.split_by_shares {
            for (i, share) in shares.iter().enumerate() {
                if let Some(user_id) = share.user_id {
                    body[format!("users__{}__user_id", i)] = json!(user_id);
                } else {
                    if let Some(ref email) = share.email {
                        body[format!("users__{}__email", i)] = json!(email);
                    }
                    if let Some(ref first_name) = share.first_name {
                        body[format!("users__{}__first_name", i)] = json!(first_name);
                    }
                    if let Some(ref last_name) = share.last_name {
                        body[format!("users__{}__last_name", i)] = json!(last_name);
                    }
                }
                body[format!("users__{}__paid_share", i)] = json!(share.paid_share);
                body[format!("users__{}__owed_share", i)] = json!(share.owed_share);
            }
        }

        #[derive(serde::Deserialize)]
        struct Response {
            expenses: Vec<Expense>,
            errors: Option<serde_json::Value>,
        }
        let response: Response = self.post("/create_expense", body).await?;
        
        if let Some(errors) = response.errors {
            if !errors.is_null() && errors.as_object().map_or(false, |o| !o.is_empty()) {
                anyhow::bail!("Failed to create expense: {:?}", errors)
            }
        }
        
        Ok(response.expenses)
    }

    pub async fn update_expense(
        &self,
        id: i64,
        request: UpdateExpenseRequest,
    ) -> Result<Vec<Expense>> {
        // Similar to create_expense but for update endpoint
        let mut body = json!({});

        if let Some(cost) = request.cost {
            body["cost"] = json!(cost);
        }
        if let Some(description) = request.description {
            body["description"] = json!(description);
        }

        if let Some(currency_code) = request.currency_code {
            body["currency_code"] = json!(currency_code);
        }
        if let Some(category_id) = request.category_id {
            body["category_id"] = json!(category_id);
        }
        if let Some(date) = request.date {
            body["date"] = json!(date);
        }
        
        // Handle split information - required when changing cost
        if let Some(split_equally) = request.split_equally {
            body["split_equally"] = json!(split_equally);
        }
        
        // Handle custom split shares - convert to flattened format for API
        if let Some(shares) = request.split_by_shares {
            for (index, share) in shares.iter().enumerate() {
                if let Some(user_id) = share.user_id {
                    body[format!("users__{}__user_id", index)] = json!(user_id);
                }
                if let Some(email) = &share.email {
                    body[format!("users__{}__email", index)] = json!(email);
                }
                body[format!("users__{}__paid_share", index)] = json!(share.paid_share);
                body[format!("users__{}__owed_share", index)] = json!(share.owed_share);
            }
        }

        #[derive(serde::Deserialize)]
        struct Response {
            expenses: Vec<Expense>,
            errors: Option<serde_json::Value>,
        }
        let response: Response = self
            .post(&format!("/update_expense/{}", id), body)
            .await?;
        
        if let Some(errors) = response.errors {
            if !errors.is_null() && errors.as_object().map_or(false, |o| !o.is_empty()) {
                anyhow::bail!("Failed to update expense: {:?}", errors)
            }
        }
        
        Ok(response.expenses)
    }

    pub async fn delete_expense(&self, id: i64) -> Result<bool> {
        #[derive(serde::Deserialize)]
        struct DeleteResponse {
            success: bool,
            errors: serde_json::Value, // Can be {} or object with actual errors
        }
        let response: DeleteResponse = self
            .post(&format!("/delete_expense/{}", id), json!({}))
            .await?;
        Ok(response.success)
    }

    // Friend endpoints
    pub async fn get_friends(&self) -> Result<Vec<Friend>> {
        #[derive(serde::Deserialize)]
        struct Response {
            friends: Vec<Friend>,
        }
        let response: Response = self.get("/get_friends").await?;
        Ok(response.friends)
    }

    pub async fn get_friend(&self, id: i64) -> Result<Friend> {
        #[derive(serde::Deserialize)]
        struct Response {
            friend: Friend,
        }
        let response: Response = self.get(&format!("/get_friend/{}", id)).await?;
        Ok(response.friend)
    }

    pub async fn create_friend(&self, email: String) -> Result<Vec<Friend>> {
        let body = json!({
            "user_email": email,
        });

        #[derive(serde::Deserialize)]
        struct Response {
            friends: Vec<Friend>,
        }
        let response: Response = self.post("/create_friend", body).await?;
        Ok(response.friends)
    }

    // Utility endpoints
    pub async fn get_currencies(&self) -> Result<Vec<Currency>> {
        #[derive(serde::Deserialize)]
        struct Response {
            currencies: Vec<Currency>,
        }
        let response: Response = self.get("/get_currencies").await?;
        Ok(response.currencies)
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>> {
        #[derive(serde::Deserialize)]
        struct Response {
            categories: Vec<Category>,
        }
        let response: Response = self.get("/get_categories").await?;
        Ok(response.categories)
    }
}