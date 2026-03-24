use std::sync::Arc;

use frontend::config::configuration::Configuration;
use tracing::debug;

const CONFIG_PATH: &str = "./config/frontend.json";

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::routing::get;
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use frontend::app::*;

    // read leptos configuration
    let conf = get_configuration(None).unwrap();

    // read frontend configuration
    let configuration =
        Arc::new(Configuration::new(CONFIG_PATH).expect("Could not parse configuration!"));
    debug!("Configuration: {}", configuration);


    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/media/{media_id}", get(proxy_media_file))
        .route("/albums/{album_id}/upload", axum::routing::post(proxy_album_upload))
        .route("/albums/{album_id}/download", get(proxy_album_download))
        .route("/albums/{album_id}/media/{media_id}", axum::routing::delete(proxy_album_media_delete))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(feature = "ssr")]
async fn proxy_media_file(
    axum::extract::Path(media_id): axum::extract::Path<String>,
    headers: axum::http::HeaderMap,
) -> axum::response::Response {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    let token = extract_customer_token(&headers);

    let url = frontend::config::configuration::core_url(&format!("/auth/customer/media/{}/file", media_id));

    let client = reqwest::Client::new();
    let mut req = client.get(&url);
    if let Some(t) = token {
        req = req.header("Authorization", format!("Bearer {}", t));
    }

    match req.send().await {
        Ok(res) => {
            let status =
                StatusCode::from_u16(res.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
            let ct = res
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("application/octet-stream")
                .to_string();
            let bytes = res.bytes().await.unwrap_or_default();
            (status, [(axum::http::header::CONTENT_TYPE, ct)], bytes).into_response()
        }
        Err(_) => StatusCode::BAD_GATEWAY.into_response(),
    }
}

#[cfg(feature = "ssr")]
fn extract_customer_token(headers: &axum::http::HeaderMap) -> Option<String> {
    let cookie_str = headers.get("cookie")?.to_str().ok()?;
    for pair in cookie_str.split(';') {
        let mut parts = pair.trim().splitn(2, '=');
        if parts.next()?.trim() == "customer_token" {
            return Some(parts.next()?.trim().to_string());
        }
    }
    None
}

#[cfg(feature = "ssr")]
async fn proxy_album_upload(
    axum::extract::Path(album_id): axum::extract::Path<String>,
    headers: axum::http::HeaderMap,
    mut multipart: axum::extract::Multipart,
) -> axum::response::Response {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    let token = extract_customer_token(&headers);
    // Redirect back to wherever the form was submitted from (dashboard or album detail).
    let redirect_url = headers
        .get("referer")
        .and_then(|v| v.to_str().ok())
        .and_then(|r| r.splitn(4, '/').nth(3).map(|p| format!("/{}", p)))
        .unwrap_or_else(|| format!("/albums/{}", album_id));
    let backend_url = frontend::config::configuration::core_url(&format!("/albums/{}/media", album_id));

    let client = reqwest::Client::new();

    // Stream each file field to the backend as a multipart upload
    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name() != Some("file") {
            continue;
        }
        let filename = match field.file_name() {
            Some(n) if !n.is_empty() => n.to_string(),
            _ => continue,
        };
        let bytes = match field.bytes().await {
            Ok(b) if !b.is_empty() => b,
            _ => continue,
        };

        let part = reqwest::multipart::Part::bytes(bytes.to_vec())
            .file_name(filename.clone())
            .mime_str("application/octet-stream")
            .unwrap_or_else(|_| reqwest::multipart::Part::bytes(vec![]));
        let form = reqwest::multipart::Form::new().part("file", part);

        let mut req = client.post(&backend_url).multipart(form);
        if let Some(ref t) = token {
            req = req.header("Authorization", format!("Bearer {}", t));
        }
        match req.send().await {
            Err(_) => return (StatusCode::BAD_GATEWAY, "Upload failed").into_response(),
            Ok(res) if !res.status().is_success() => {
                let status = axum::http::StatusCode::from_u16(res.status().as_u16())
                    .unwrap_or(StatusCode::BAD_GATEWAY);
                let body = res.bytes().await.unwrap_or_default();
                return (status, body).into_response();
            }
            Ok(_) => {}
        }
    }

    axum::response::Redirect::to(&redirect_url).into_response()
}

#[cfg(feature = "ssr")]
async fn proxy_album_media_delete(
    axum::extract::Path((album_id, media_id)): axum::extract::Path<(String, String)>,
    headers: axum::http::HeaderMap,
) -> axum::response::Response {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    let token = extract_customer_token(&headers);
    let url = frontend::config::configuration::core_url(&format!("/albums/{}/media/{}", album_id, media_id));
    let client = reqwest::Client::new();
    let mut req = client.delete(&url);
    if let Some(t) = token {
        req = req.header("Authorization", format!("Bearer {}", t));
    }

    match req.send().await {
        Ok(res) => {
            let status = axum::http::StatusCode::from_u16(res.status().as_u16())
                .unwrap_or(StatusCode::BAD_GATEWAY);
            let body = res.bytes().await.unwrap_or_default();
            (status, body).into_response()
        }
        Err(_) => StatusCode::BAD_GATEWAY.into_response(),
    }
}

#[cfg(feature = "ssr")]
async fn proxy_album_download(
    axum::extract::Path(album_id): axum::extract::Path<String>,
    headers: axum::http::HeaderMap,
) -> axum::response::Response {
    use axum::body::Body;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    let token = extract_customer_token(&headers);
    let url = frontend::config::configuration::core_url(&format!("/albums/{}/download", album_id));
    let client = reqwest::Client::new();
    let mut req = client.get(&url);
    if let Some(t) = token {
        req = req.header("Authorization", format!("Bearer {}", t));
    }

    // Forward Range header so the backend can serve partial content (206).
    if let Some(range) = headers.get("range") {
        req = req.header("Range", range);
    }

    match req.send().await {
        Ok(res) => {
            let status =
                StatusCode::from_u16(res.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
            let get_hdr = |name: &str| -> Option<String> {
                res.headers().get(name).and_then(|v| v.to_str().ok()).map(|s| s.to_string())
            };
            let ct = get_hdr("content-type").unwrap_or_else(|| "application/zip".to_string());
            let cd = get_hdr("content-disposition")
                .unwrap_or_else(|| "attachment; filename=\"album.zip\"".to_string());

            let mut builder = axum::response::Response::builder()
                .status(status.as_u16())
                .header(axum::http::header::CONTENT_TYPE, ct)
                .header(axum::http::header::CONTENT_DISPOSITION, cd);

            // Pass through range-related headers for resumable download support.
            for hdr in &["accept-ranges", "content-range", "content-length"] {
                if let Some(val) = get_hdr(hdr) {
                    builder = builder.header(*hdr, val);
                }
            }

            let stream = res.bytes_stream();
            builder
                .body(Body::from_stream(stream))
                .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
        Err(_) => StatusCode::BAD_GATEWAY.into_response(),
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // see lib.rs for hydration function instead
}
