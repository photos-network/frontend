#![recursion_limit = "256"]

include!(concat!(env!("OUT_DIR"), "/i18n/mod.rs"));

pub mod api;
pub mod app;
pub mod auth;
pub mod config;
pub mod components;
pub mod error_template;
pub mod footer;
pub mod header;
pub mod pages;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
