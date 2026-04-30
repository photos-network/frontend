use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAccountEntry {
    pub account_id: String,
    pub email: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAlbumEntry {
    pub album_id: String,
    pub owner: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumAccessEntry {
    pub account_id: String,
    pub album_id: String,
    pub role: String,
}

#[server(GetAdminUsers, "/api", "Cbor")]
pub async fn get_admin_users() -> Result<Vec<AdminAccountEntry>, ServerFnError> {
    use axum::http::HeaderMap;
    use leptos_axum::extract;
    use crate::auth::parse_cookie_token;

    let headers: HeaderMap = extract().await.unwrap_or_default();
    let token = parse_cookie_token(&headers)
        .ok_or_else(|| ServerFnError::new("No session cookie found – please sign in"))?;

    let client = reqwest::Client::new();
    let res = client
        .get(crate::config::configuration::core_url("/admin/users"))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Backend unreachable: {}", e)))?;

    let status = res.status();
    if status == reqwest::StatusCode::UNAUTHORIZED {
        return Err(ServerFnError::new("Token rejected by backend (401)"));
    }
    if !status.is_success() {
        let body = res.text().await.unwrap_or_default();
        return Err(ServerFnError::new(format!("Backend error {}: {}", status, body)));
    }

    res.json::<Vec<AdminAccountEntry>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to parse user list: {}", e)))
}

#[server(GetAdminAlbums, "/api", "Cbor")]
pub async fn get_admin_albums() -> Result<Vec<AdminAlbumEntry>, ServerFnError> {
    use axum::http::HeaderMap;
    use leptos_axum::extract;
    use crate::auth::parse_cookie_token;

    let headers: HeaderMap = extract().await.unwrap_or_default();
    let token = parse_cookie_token(&headers)
        .ok_or_else(|| ServerFnError::new("No session cookie found – please sign in"))?;

    let client = reqwest::Client::new();
    let res = client
        .get(crate::config::configuration::core_url("/admin/albums"))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Backend unreachable: {}", e)))?;

    let status = res.status();
    if status == reqwest::StatusCode::UNAUTHORIZED {
        return Err(ServerFnError::new("Token rejected by backend (401)"));
    }
    if !status.is_success() {
        let body = res.text().await.unwrap_or_default();
        return Err(ServerFnError::new(format!("Backend error {}: {}", status, body)));
    }

    res.json::<Vec<AdminAlbumEntry>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to parse album list: {}", e)))
}

#[server(GetAlbumAccessList, "/api", "Cbor")]
pub async fn get_album_access_list(album_id: String) -> Result<Vec<AlbumAccessEntry>, ServerFnError> {
    use axum::http::HeaderMap;
    use leptos_axum::extract;
    use crate::auth::parse_cookie_token;

    let headers: HeaderMap = extract().await.unwrap_or_default();
    let token = parse_cookie_token(&headers)
        .ok_or_else(|| ServerFnError::new("No session cookie found – please sign in"))?;

    let client = reqwest::Client::new();
    let res = client
        .get(crate::config::configuration::core_url(&format!("/albums/{}/access", album_id)))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Backend unreachable: {}", e)))?;

    let status = res.status();
    if status == reqwest::StatusCode::UNAUTHORIZED {
        return Err(ServerFnError::new("Token rejected by backend (401)"));
    }
    if !status.is_success() {
        let body = res.text().await.unwrap_or_default();
        return Err(ServerFnError::new(format!("Backend error {}: {}", status, body)));
    }

    res.json::<Vec<AlbumAccessEntry>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to parse access list: {}", e)))
}

#[server(GrantAlbumAccess, "/api", "Cbor")]
pub async fn grant_album_access(album_id: String, account_id: String, role: String) -> Result<(), ServerFnError> {
    use axum::http::HeaderMap;
    use leptos_axum::extract;
    use crate::auth::parse_cookie_token;

    let headers: HeaderMap = extract().await.unwrap_or_default();
    let token = parse_cookie_token(&headers)
        .ok_or_else(|| ServerFnError::new("No session cookie found – please sign in"))?;

    let client = reqwest::Client::new();
    let res = client
        .post(crate::config::configuration::core_url(&format!("/albums/{}/access", album_id)))
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({ "account_id": account_id, "role": role }))
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Backend unreachable: {}", e)))?;

    let status = res.status();
    if status == reqwest::StatusCode::UNAUTHORIZED {
        return Err(ServerFnError::new("Token rejected by backend (401)"));
    }
    if !status.is_success() {
        let body = res.text().await.unwrap_or_default();
        return Err(ServerFnError::new(format!("Backend error {}: {}", status, body)));
    }

    Ok(())
}

#[server(RevokeAlbumAccess, "/api", "Cbor")]
pub async fn revoke_album_access(album_id: String, account_id: String) -> Result<(), ServerFnError> {
    use axum::http::HeaderMap;
    use leptos_axum::extract;
    use crate::auth::parse_cookie_token;

    let headers: HeaderMap = extract().await.unwrap_or_default();
    let token = parse_cookie_token(&headers)
        .ok_or_else(|| ServerFnError::new("No session cookie found – please sign in"))?;

    let client = reqwest::Client::new();
    let res = client
        .delete(crate::config::configuration::core_url(&format!("/albums/{}/access/{}", album_id, account_id)))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Backend unreachable: {}", e)))?;

    let status = res.status();
    if status == reqwest::StatusCode::UNAUTHORIZED {
        return Err(ServerFnError::new("Token rejected by backend (401)"));
    }
    if !status.is_success() {
        let body = res.text().await.unwrap_or_default();
        return Err(ServerFnError::new(format!("Backend error {}: {}", status, body)));
    }

    Ok(())
}
