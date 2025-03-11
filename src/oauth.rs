use reqwest::{Error, Response};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use twapi_oauth::calc_oauth_header;

use crate::build_client;

pub async fn get_bearer_token_response(
    consumer_key: &str,
    consumer_secret: &str,
    timeout_sec: Option<Duration>,
) -> Result<Response, Error> {
    let key = base64::encode(&format!("{}:{}", consumer_key, consumer_secret));
    let client = build_client(timeout_sec);
    client
        .post("https://api.twitter.com/oauth2/token")
        .header(
            "Content-Type",
            "application/x-www-form-urlencoded;charset=UTF-8",
        )
        .header("Authorization", &format!("Basic {}", key))
        .body("grant_type=client_credentials")
        .send()
        .await
}

pub async fn get_bearer_token(
    consumer_key: &str,
    consumer_secret: &str,
    timeout_sec: Option<Duration>,
) -> Result<Option<String>, Error> {
    let json: Value = get_bearer_token_response(consumer_key, consumer_secret, timeout_sec)
        .await?
        .json()
        .await?;
    match json["access_token"].as_str() {
        Some(access_token) => Ok(Some(access_token.to_string())),
        None => Ok(None),
    }
}

pub async fn request_token_response(
    consumer_key: &str,
    consumer_secret: &str,
    oauth_callback: &str,
    x_auth_access_type: Option<&str>,
    timeout_sec: Option<Duration>,
) -> Result<Response, Error> {
    let uri = "https://api.twitter.com/oauth/request_token";
    let mut header_options = vec![("oauth_callback", oauth_callback)];
    if let Some(x_auth_access_type) = x_auth_access_type {
        header_options.push(("x_auth_access_type", x_auth_access_type));
    }
    let signed = calc_oauth_header(
        &format!("{}&", consumer_secret),
        consumer_key,
        &header_options,
        "POST",
        uri,
        &vec![],
    );
    let client = build_client(timeout_sec);
    client
        .post(uri)
        .header("Authorization", &format!("OAuth {}", signed))
        .send()
        .await
}

pub async fn request_token(
    consumer_key: &str,
    consumer_secret: &str,
    oauth_callback: &str,
    x_auth_access_type: Option<&str>,
    timeout_sec: Option<Duration>,
) -> Result<HashMap<String, String>, Error> {
    let response = request_token_response(
        consumer_key,
        consumer_secret,
        oauth_callback,
        x_auth_access_type,
        timeout_sec,
    )
    .await?;
    Ok(parse_oauth_body(response).await)
}

pub async fn access_token_response(
    consumer_key: &str,
    consumer_secret: &str,
    oauth_token: &str,
    oauth_token_secret: &str,
    oauth_verifier: &str,
    timeout_sec: Option<Duration>,
) -> Result<Response, Error> {
    let uri = "https://api.twitter.com/oauth/access_token";
    let signed = calc_oauth_header(
        &format!("{}&{}", consumer_secret, oauth_token_secret),
        consumer_key,
        &vec![
            ("oauth_token", oauth_token),
            ("oauth_verifier", oauth_verifier),
        ],
        "POST",
        uri,
        &vec![],
    );
    let client = build_client(timeout_sec);
    client
        .post(uri)
        .header("Authorization", &format!("OAuth {}", signed))
        .send()
        .await
}

pub async fn access_token(
    consumer_key: &str,
    consumer_secret: &str,
    oauth_token: &str,
    oauth_token_secret: &str,
    oauth_verifier: &str,
    timeout_sec: Option<Duration>,
) -> Result<HashMap<String, String>, Error> {
    let response = access_token_response(
        consumer_key,
        consumer_secret,
        oauth_token,
        oauth_token_secret,
        oauth_verifier,
        timeout_sec,
    )
    .await?;
    Ok(parse_oauth_body(response).await)
}

async fn parse_oauth_body(response: Response) -> HashMap<String, String> {
    let status = response.status();
    let mut result = HashMap::new();
    match response.text().await {
        Ok(body) => {
            if status.is_success() {
                for item in body.split("&") {
                    let mut pair = item.split("=");
                    if let Some(key) = pair.next() {
                        result.insert(key.to_owned(), pair.next().unwrap_or("").to_owned());
                    }
                }
            } else {
                result.insert("twapi_request_body".to_owned(), body.clone());
                result.insert(
                    "twapi_request_status_code".to_owned(),
                    status.as_str().to_owned(),
                );
            }
        }
        Err(_) => {}
    }
    result
}
