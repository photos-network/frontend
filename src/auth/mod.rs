pub mod guards;

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionKind {
    None,
    Customer,
    Account,
    Admin,
}

/// Extracts `customer_token` from the `Cookie` header. SSR-only.
#[cfg(feature = "ssr")]
pub(crate) fn parse_cookie_token(headers: &axum::http::HeaderMap) -> Option<String> {
    let cookie_str = headers.get("cookie")?.to_str().ok()?;
    for pair in cookie_str.split(';') {
        let mut parts = pair.trim().splitn(2, '=');
        if parts.next()?.trim() == "customer_token" {
            return Some(parts.next()?.trim().to_string());
        }
    }
    None
}

#[server]
pub async fn check_session_kind() -> Result<SessionKind, ServerFnError> {
    use axum::http::HeaderMap;
    use leptos_axum::extract;

    let headers: HeaderMap = extract().await.unwrap_or_default();
    match parse_cookie_token(&headers) {
        Some(t) => Ok(decode_session_kind(&t)),
        None => Ok(SessionKind::None),
    }
}

#[cfg(feature = "ssr")]
fn decode_session_kind(token: &str) -> SessionKind {
    use base64::Engine;

    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return SessionKind::Customer;
    }

    let b64 = parts[1].replace('-', "+").replace('_', "/");
    let padding = (4 - b64.len() % 4) % 4;
    let padded = format!("{}{}", b64, "=".repeat(padding));

    let decoded = match base64::engine::general_purpose::STANDARD.decode(&padded) {
        Ok(b) => b,
        Err(_) => return SessionKind::Customer,
    };
    let payload: serde_json::Value = match serde_json::from_slice(&decoded) {
        Ok(v) => v,
        Err(_) => return SessionKind::Customer,
    };

    let role = payload["role"].as_str().unwrap_or("");
    let is_admin = payload["is_admin"].as_bool().unwrap_or(false);

    match (role, is_admin) {
        ("account", true) => SessionKind::Admin,
        ("account", false) => SessionKind::Account,
        _ => SessionKind::Customer,
    }
}
