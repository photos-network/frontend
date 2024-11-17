use leptos::Serializable;
use serde::{Deserialize, Serialize};

pub fn user(path: &str) -> String {
	format!("http://127.0.0.1:7777/oidc/authorize?response_type=id_token%20token&client_id=mobile-app&state=12345&code_challenge=47DEQp
       │ j8HBSa-_TImW-5JCeuQeRkm5NMpJWZG3hSuFU&code_challenge_method=S256&redirect_uri=photosapp://authenticate&scope=openid%20profile%
       │ 20email%20phone%20library:read&nonce=ABCD")
}

pub async fn fetch_api<T>(path: &str) -> Option<T>
where
    T: Serializable,
{
    let json = reqwest::get(path)
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()?
        .text()
        .await
        .ok()?;
    T::de(&json).map_err(|e| log::error!("{e}")).ok()
}

