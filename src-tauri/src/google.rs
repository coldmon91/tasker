use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::db::Database;
use std::env;

const REDIRECT_URI: &str = "http://localhost:14123"; 
const AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const SCOPE: &str = "https://www.googleapis.com/auth/tasks https://www.googleapis.com/auth/calendar https://www.googleapis.com/auth/userinfo.email https://www.googleapis.com/auth/userinfo.profile";

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    access_token: String,
    expires_in: i64,
    refresh_token: Option<String>,
    scope: String,
    token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleUser {
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskList {
    pub id: String,
    pub title: String,
    pub updated: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskListResponse {
    pub items: Option<Vec<TaskList>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTask {
    pub id: String,
    pub title: String,
    pub updated: String,
    #[serde(rename = "selfLink")]
    pub self_link: Option<String>,
    pub position: Option<String>,
    pub status: String,
    pub due: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
    pub items: Option<Vec<GoogleTask>>,
}

fn get_client_id() -> String {
    env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set in .env")
}

fn get_client_secret() -> String {
    env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set in .env")
}

pub fn get_auth_url() -> String {
    format!(
        "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&access_type=offline&prompt=consent",
        AUTH_URL, get_client_id(), REDIRECT_URI, SCOPE
    )
}

pub async fn get_user_profile(db: &Database) -> Result<GoogleUser, String> {
    let token = get_access_token(db).await?;
    let client = Client::new();
    
    let res = client.get("https://www.googleapis.com/oauth2/v2/userinfo")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("API Error: {:?}", res.text().await));
    }

    let user: GoogleUser = res.json().await.map_err(|e| e.to_string())?;
    Ok(user)
}

pub async fn exchange_code(code: &str, db: &Database) -> Result<GoogleUser, String> {
    let client = Client::new();
    let mut params = HashMap::new();
    params.insert("code", code.to_string());
    params.insert("client_id", get_client_id());
    params.insert("client_secret", get_client_secret());
    params.insert("redirect_uri", REDIRECT_URI.to_string());
    params.insert("grant_type", "authorization_code".to_string());

    let res = client.post(TOKEN_URL)
        .form(&params)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("Failed to exchange code: {:?}", res.text().await));
    }

    let token_res: TokenResponse = res.json().await.map_err(|e| e.to_string())?;

    // Save tokens
    db.set_setting("google_access_token", &token_res.access_token).map_err(|e| e.to_string())?;
    if let Some(refresh_token) = token_res.refresh_token {
        db.set_setting("google_refresh_token", &refresh_token).map_err(|e| e.to_string())?;
    }
    
    // Fetch and return user profile
    get_user_profile(db).await
}

async fn get_access_token(db: &Database) -> Result<String, String> {
    if let Some(token) = db.get_setting("google_access_token").map_err(|e| e.to_string())? {
        Ok(token)
    } else {
        Err("No access token found".to_string())
    }
}

pub async fn fetch_task_lists(db: &Database) -> Result<Vec<TaskList>, String> {
    let token = get_access_token(db).await?;
    let client = Client::new();
    
    let res = client.get("https://tasks.googleapis.com/tasks/v1/users/@me/lists")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("API Error: {:?}", res.text().await));
    }

    let list_res: TaskListResponse = res.json().await.map_err(|e| e.to_string())?;
    Ok(list_res.items.unwrap_or_default())
}

pub async fn fetch_tasks(db: &Database, tasklist_id: &str) -> Result<Vec<GoogleTask>, String> {
    let token = get_access_token(db).await?;
    let client = Client::new();
    
    let res = client.get(&format!("https://tasks.googleapis.com/tasks/v1/lists/{}/tasks", tasklist_id))
        .header("Authorization", format!("Bearer {}", token))
        .query(&[("showCompleted", "true"), ("showHidden", "true")])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("API Error: {:?}", res.text().await));
    }

    let task_res: TaskResponse = res.json().await.map_err(|e| e.to_string())?;
    Ok(task_res.items.unwrap_or_default())
}
