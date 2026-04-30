use leptos::prelude::*;

#[server(CheckAlbumOwnership, "/api", "Cbor")]
pub async fn check_album_ownership(album_id: String) -> Result<bool, ServerFnError> {
    use axum::http::HeaderMap;
    use leptos_axum::extract;
    use crate::auth::parse_cookie_token;

    let headers: HeaderMap = extract().await.unwrap_or_default();
    let token = match parse_cookie_token(&headers) {
        Some(t) => t,
        None => return Ok(false),
    };

    let client = reqwest::Client::new();
    let res = client
        .get(crate::config::configuration::core_url(&format!("/albums/{}/codes", album_id)))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(res.status().is_success())
}

#[server(DeleteAlbumMedia, "/api", "Cbor")]
pub async fn delete_album_media(album_id: String, media_id: String) -> Result<(), ServerFnError> {
    use axum::http::HeaderMap;
    use leptos_axum::extract;
    use crate::auth::parse_cookie_token;

    let headers: HeaderMap = extract().await.unwrap_or_default();
    let token = parse_cookie_token(&headers)
        .ok_or_else(|| ServerFnError::new("No session"))?;

    let client = reqwest::Client::new();
    let res = client
        .delete(crate::config::configuration::core_url(&format!("/albums/{}/media/{}", album_id, media_id)))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if !res.status().is_success() {
        let body = res.text().await.unwrap_or_default();
        return Err(ServerFnError::new(body));
    }

    Ok(())
}
