use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::db::Database;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

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

pub fn start_oauth_server() -> Result<String, String> {
    let auth_code = Arc::new(Mutex::new(None::<String>));
    let auth_code_clone = Arc::clone(&auth_code);

    let server = tiny_http::Server::http("127.0.0.1:14123")
        .map_err(|e| format!("Failed to start OAuth server: {}", e))?;

    thread::spawn(move || {
        if let Ok(Some(request)) = server.recv_timeout(std::time::Duration::from_secs(120)) {
            let url = request.url().to_string();
            let mut found_code = false;
            
            // Parse query parameters to extract code
            if let Some(query_start) = url.find('?') {
                let query = &url[query_start + 1..];
                for param in query.split('&') {
                    if let Some((key, value)) = param.split_once('=') {
                        if key == "code" {
                            let mut code_guard = auth_code_clone.lock().unwrap();
                            *code_guard = Some(value.to_string());
                            found_code = true;
                            break;
                        }
                    }
                }
            }
            
            // Send response to browser
            if found_code {
                let response = tiny_http::Response::from_string(
                    r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Authentication Successful</title>
    <style>
        body { font-family: system-ui; display: flex; align-items: center; justify-content: center; height: 100vh; margin: 0; background: #f5f5f5; }
        .card { background: white; padding: 2rem; border-radius: 12px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); text-align: center; max-width: 400px; }
        .success { color: #10b981; font-size: 3rem; margin-bottom: 1rem; }
        h1 { color: #1f2937; margin: 0 0 0.5rem 0; font-size: 1.5rem; }
        p { color: #6b7280; margin: 0; }
    </style>
</head>
<body>
    <div class="card">
        <div class="success">✓</div>
        <h1>Authentication Successful!</h1>
        <p>You can now close this window and return to Tasker.</p>
    </div>
</body>
</html>"#
                ).with_header(
                    tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap()
                );
                let _ = request.respond(response);
            } else {
                let response = tiny_http::Response::from_string(
                    r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Authentication Failed</title>
    <style>
        body { font-family: system-ui; display: flex; align-items: center; justify-content: center; height: 100vh; margin: 0; background: #f5f5f5; }
        .card { background: white; padding: 2rem; border-radius: 12px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); text-align: center; max-width: 400px; }
        .error { color: #ef4444; font-size: 3rem; margin-bottom: 1rem; }
        h1 { color: #1f2937; margin: 0 0 0.5rem 0; font-size: 1.5rem; }
        p { color: #6b7280; margin: 0; }
    </style>
</head>
<body>
    <div class="card">
        <div class="error">✗</div>
        <h1>Authentication Failed</h1>
        <p>No authorization code received. Please try again.</p>
    </div>
</body>
</html>"#
                ).with_header(
                    tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap()
                );
                let _ = request.respond(response);
            }
        }
    });

    // Wait for auth code with timeout
    for _ in 0..120 {
        thread::sleep(std::time::Duration::from_secs(1));
        let code_guard = auth_code.lock().unwrap();
        if let Some(code) = code_guard.as_ref() {
            return Ok(code.clone());
        }
    }

    Err("Timeout waiting for authorization code".to_string())
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
