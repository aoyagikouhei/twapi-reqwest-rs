pub mod oauth;
pub(crate) mod raw;
pub mod v1;
pub mod v2;

pub use reqwest;
use reqwest::Client;
pub use serde_json;
use std::time::Duration;

pub(crate) fn make_body(form_options: &Vec<(&str, &str)>) -> String {
    match serde_urlencoded::to_string(form_options) {
        Ok(body) => body
            .replace('+', "%20")
            .replace('*', "%2A")
            .replace("%7E", "~")
            .into(),
        Err(_) => String::from(""),
    }
}

pub(crate) fn build_client(timeout_sec: Option<Duration>) -> Client {
    match timeout_sec {
        Some(value) => Client::builder().timeout(value),
        None => Client::builder(),
    }
    .build()
    .unwrap()
}
