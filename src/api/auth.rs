use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomerLoginResponse {
    pub jwt_token: Option<String>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountLoginResponse {
    pub jwt_token: Option<String>,
    pub error: Option<String>,
}

#[server(CustomerLogin, "/api")]
pub async fn customer_login(access_code: String) -> Result<CustomerLoginResponse, ServerFnError> {
    use axum::http::{header, HeaderValue};
    use leptos_axum::ResponseOptions;

    let client = reqwest::Client::new();
    let res = client
        .post(crate::config::configuration::core_url("/oidc/token"))
        .form(&[
            ("grant_type", "urn:photos.network:access_code"),
            ("access_code", access_code.as_str()),
        ])
        .send()
        .await
        .map_err(|_| ServerFnError::new("The server could not be reached"))?;

    if res.status().is_success() {
        let body: serde_json::Value = res
            .json()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        let token = body
            .get("access_token")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        if let Some(ref jwt) = token {
            let response_options = expect_context::<ResponseOptions>();
            let cookie = format!("customer_token={}; Path=/; SameSite=Lax; Max-Age=86400", jwt);
            if let Ok(val) = HeaderValue::from_str(&cookie) {
                response_options.append_header(header::SET_COOKIE, val);
            }
        }

        Ok(CustomerLoginResponse {
            jwt_token: token,
            error: None,
        })
    } else {
        Ok(CustomerLoginResponse {
            jwt_token: None,
            error: Some("Invalid access code.".to_string()),
        })
    }
}

#[server(AccountLogin, "/api")]
pub async fn account_login(email: String, password: String) -> Result<AccountLoginResponse, ServerFnError> {
    use axum::http::{header, HeaderValue};
    use leptos_axum::ResponseOptions;

    let client = reqwest::Client::new();
    let res = client
        .post(crate::config::configuration::core_url("/oidc/token"))
        .form(&[
            ("grant_type", "password"),
            ("username", email.as_str()),
            ("password", password.as_str()),
        ])
        .send()
        .await
        .map_err(|_| ServerFnError::new("The server could not be reached"))?;

    if res.status().is_success() {
        let body: serde_json::Value = res
            .json()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        let token = body
            .get("access_token")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        if let Some(ref jwt) = token {
            let response_options = expect_context::<ResponseOptions>();
            let cookie = format!("customer_token={}; Path=/; SameSite=Lax; Max-Age=86400", jwt);
            if let Ok(val) = HeaderValue::from_str(&cookie) {
                response_options.append_header(header::SET_COOKIE, val);
            }
        }

        Ok(AccountLoginResponse {
            jwt_token: token,
            error: None,
        })
    } else {
        Ok(AccountLoginResponse {
            jwt_token: None,
            error: Some("Invalid email or password.".to_string()),
        })
    }
}
