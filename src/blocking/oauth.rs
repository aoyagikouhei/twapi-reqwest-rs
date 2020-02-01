use reqwest::{
    blocking::{Client, Response},
    Error,
};
use serde_json::Value;
use twapi_oauth::calc_oauth_header;

pub fn get_bearer_token_response(
    consumer_key: &str,
    consumer_secret: &str,
) -> Result<Response, Error> {
    let key = base64::encode(&format!("{}:{}", consumer_key, consumer_secret));
    let client = Client::new();
    client
        .post("https://api.twitter.com/oauth2/token")
        .header(
            "Content-Type",
            "application/x-www-form-urlencoded;charset=UTF-8",
        )
        .header("Authorization", &format!("Basic {}", key))
        .body("grant_type=client_credentials")
        .send()
}

pub fn get_bearer_token(consumer_key: &str, consumer_secret: &str) -> Option<String> {
    match get_bearer_token_response(consumer_key, consumer_secret) {
        Ok(response) => match response.json::<Value>() {
            Ok(json) => match json["access_token"].as_str() {
                Some(access_token) => Some(access_token.to_string()),
                None => None,
            },
            Err(_) => None,
        },
        Err(_) => None,
    }
}

pub fn request_token_response(
    consumer_key: &str,
    consumer_secret: &str,
    oauth_callback: &str,
    x_auth_access_type: Option<&str>,
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
    let client = Client::new();
    client
        .post(uri)
        .header("Authorization", &format!("OAuth {}", signed))
        .send()
}

pub fn request_token(
    consumer_key: &str,
    consumer_secret: &str,
    oauth_callback: &str,
    x_auth_access_type: Option<&str>,
) -> Option<Vec<(String, String)>> {
    match request_token_response(consumer_key, consumer_secret, oauth_callback, x_auth_access_type) {
        Ok(response) => parse_oauth_body(response),
        Err(_) => None,
    }
}

pub fn access_token_response(
    consumer_key: &str,
    consumer_secret: &str,
    oauth_token: &str,
    oauth_token_secret: &str,
    oauth_verifier: &str,
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
    let client = Client::new();
    client
        .post(uri)
        .header("Authorization", &format!("OAuth {}", signed))
        .send()
}

pub fn access_token(
    consumer_key: &str,
    consumer_secret: &str,
    oauth_token: &str,
    oauth_token_secret: &str,
    oauth_verifier: &str,
) -> Option<Vec<(String, String)>> {
    match access_token_response(consumer_key, consumer_secret, oauth_token, oauth_token_secret, oauth_verifier) {
        Ok(response) => parse_oauth_body(response),
        Err(_) => None,
    }
}

fn parse_oauth_body(response: Response) -> Option<Vec<(String, String)>> {
    match response.text() {
        Ok(body) => {
            Some(body.split("&").map(|it| {
                let mut pair = it.split("=");
                (pair.next().unwrap().to_string(), pair.next().unwrap().to_string())
            }).collect())
        },
        Err(_) => None,
    }
}