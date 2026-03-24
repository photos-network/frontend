use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessCodeEntry {
    pub access_code: String,
    pub display_name: Option<String>,
    pub customer_id: String,
}

#[server(GetAlbumCodes, "/api", "Cbor")]
pub async fn get_album_codes(album_id: String) -> Result<Vec<AccessCodeEntry>, ServerFnError> {
    use axum::http::HeaderMap;
    use leptos_axum::extract;
    use crate::auth::parse_cookie_token;

    let headers: HeaderMap = extract().await.unwrap_or_default();
    let token = parse_cookie_token(&headers)
        .ok_or_else(|| ServerFnError::new("No session"))?;

    let client = reqwest::Client::new();
    let res = client
        .get(crate::config::configuration::core_url(&format!("/albums/{}/codes", album_id)))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if !res.status().is_success() {
        return Err(ServerFnError::new(format!("Backend error {}", res.status())));
    }

    res.json::<Vec<AccessCodeEntry>>().await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(GenerateAlbumCode, "/api", "Cbor")]
pub async fn generate_album_code(album_id: String, display_name: String) -> Result<String, ServerFnError> {
    use axum::http::HeaderMap;
    use leptos_axum::extract;
    use crate::auth::parse_cookie_token;

    let headers: HeaderMap = extract().await.unwrap_or_default();
    let token = parse_cookie_token(&headers)
        .ok_or_else(|| ServerFnError::new("No session"))?;

    let client = reqwest::Client::new();
    let res = client
        .post(crate::config::configuration::core_url(&format!("/albums/{}/codes/generate", album_id)))
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({ "display_name": display_name }))
        .send()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if !res.status().is_success() {
        let body = res.text().await.unwrap_or_default();
        return Err(ServerFnError::new(body));
    }

    let json: serde_json::Value = res.json().await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(json["access_code"].as_str().unwrap_or("").to_string())
}

#[server(RemoveAlbumCode, "/api", "Cbor")]
pub async fn remove_album_code(album_id: String, access_code: String) -> Result<(), ServerFnError> {
    use axum::http::HeaderMap;
    use leptos_axum::extract;
    use crate::auth::parse_cookie_token;

    let headers: HeaderMap = extract().await.unwrap_or_default();
    let token = parse_cookie_token(&headers)
        .ok_or_else(|| ServerFnError::new("No session"))?;

    let client = reqwest::Client::new();
    let res = client
        .delete(crate::config::configuration::core_url(&format!("/albums/{}/codes/{}", album_id, access_code)))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if !res.status().is_success() {
        return Err(ServerFnError::new(format!("Backend error {}", res.status())));
    }

    Ok(())
}
