pub mod oauth;
pub(crate) mod raw;
pub mod v1;
pub mod v2;

pub use reqwest;
pub use serde_json;

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
