use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlbumSummary {
    pub album_id: String,
    pub name: String,
    pub description: Option<String>,
    pub cover_media_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumStats {
    pub album_id: String,
    pub total_views: i64,
    pub unique_viewers: i64,
    pub total_downloads: i64,
    pub viewers: Vec<ViewerEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewerEntry {
    pub viewer_id: String,
    pub viewer_role: String,
    pub view_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumRef {
    pub album_id: String,
    pub album_name: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountWithAlbums {
    pub account_id: String,
    pub email: String,
    pub display_name: Option<String>,
    pub last_login_at: Option<String>,
    pub is_admin: bool,
    pub albums: Vec<AlbumRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaItemSummary {
    pub uuid: String,
    pub name: String,
}

#[server(GetCustomerAlbums, "/api", "Cbor")]
pub async fn get_customer_albums() -> Result<Vec<AlbumSummary>, ServerFnError> {
    use axum::http::HeaderMap;
    use leptos_axum::extract;
    use crate::auth::parse_cookie_token;

    let headers: HeaderMap = extract().await.unwrap_or_default();
    let token = parse_cookie_token(&headers)
        .ok_or_else(|| ServerFnError::new("No session cookie found – please sign in"))?;

    let client = reqwest::Client::new();
    let res = client
        .get(crate::config::configuration::core_url("/auth/customer/albums"))
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

    res.json::<Vec<AlbumSummary>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to parse album list: {}", e)))
}

#[server(GetAlbumMedia, "/api", "Cbor")]
pub async fn get_album_media(album_id: String) -> Result<Vec<MediaItemSummary>, ServerFnError> {
    use axum::http::HeaderMap;
    use leptos_axum::extract;
    use crate::auth::parse_cookie_token;

    let headers: HeaderMap = extract().await.unwrap_or_default();
    let token = parse_cookie_token(&headers)
        .ok_or_else(|| ServerFnError::new("Not authenticated"))?;

    let client = reqwest::Client::new();
    let res = client
        .get(crate::config::configuration::core_url(&format!("/auth/customer/albums/{}/media", album_id)))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if !res.status().is_success() {
        return Err(ServerFnError::new("Failed to fetch media"));
    }

    res.json::<Vec<MediaItemSummary>>().await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(GetDashboardAlbums, "/api", "Cbor")]
pub async fn get_dashboard_albums() -> Result<Vec<AlbumSummary>, ServerFnError> {
    use axum::http::HeaderMap;
    use leptos_axum::extract;
    use crate::auth::parse_cookie_token;

    let headers: HeaderMap = extract().await.unwrap_or_default();
    let token = parse_cookie_token(&headers)
        .ok_or_else(|| ServerFnError::new("No session cookie found – please sign in"))?;

    let client = reqwest::Client::new();
    let res = client
        .get(crate::config::configuration::core_url("/auth/customer/albums"))
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

    res.json::<Vec<AlbumSummary>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to parse album list: {}", e)))
}

#[server(GetOwnedAlbumStats, "/api", "Cbor")]
pub async fn get_owned_album_stats() -> Result<Vec<AlbumStats>, ServerFnError> {
    use axum::http::HeaderMap;
    use leptos_axum::extract;
    use crate::auth::parse_cookie_token;

    let headers: HeaderMap = extract().await.unwrap_or_default();
    let token = match parse_cookie_token(&headers) {
        Some(t) => t,
        None => return Ok(vec![]),
    };

    let client = reqwest::Client::new();
    let res = match client
        .get(crate::config::configuration::core_url("/albums/stats"))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
    {
        Ok(r) => r,
        Err(_) => return Ok(vec![]),
    };

    if !res.status().is_success() {
        return Ok(vec![]);
    }

    match res.json::<Vec<AlbumStats>>().await {
        Ok(s) => Ok(s),
        Err(_) => Ok(vec![]),
    }
}

#[server(GetAdminUsersDetailed, "/api", "Cbor")]
pub async fn get_admin_users_detailed() -> Result<Vec<AccountWithAlbums>, ServerFnError> {
    use axum::http::HeaderMap;
    use leptos_axum::extract;
    use crate::auth::parse_cookie_token;

    let headers: HeaderMap = extract().await.unwrap_or_default();
    let token = match parse_cookie_token(&headers) {
        Some(t) => t,
        None => return Ok(vec![]),
    };

    let client = reqwest::Client::new();
    let res = match client
        .get(crate::config::configuration::core_url("/admin/users/detailed"))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
    {
        Ok(r) => r,
        Err(_) => return Ok(vec![]),
    };

    if !res.status().is_success() {
        return Ok(vec![]);
    }

    match res.json::<Vec<AccountWithAlbums>>().await {
        Ok(u) => Ok(u),
        Err(_) => Ok(vec![]),
    }
}
