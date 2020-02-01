use reqwest::{multipart::Form, Error, Response};
use serde_json::Value;
use twapi_oauth::oauth2_authorization_header;

pub async fn get(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    bearer_token: &str,
) -> Result<Response, Error> {
    let authorization = oauth2_authorization_header(bearer_token);
    crate::raw::get(url, query_options, &authorization).await
}

pub async fn post(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    form_options: &Vec<(&str, &str)>,
    bearer_token: &str,
) -> Result<Response, Error> {
    let authorization = oauth2_authorization_header(bearer_token);
    crate::raw::post(url, query_options, form_options, &authorization).await
}

pub async fn json(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    data: &Value,
    bearer_token: &str,
) -> Result<Response, Error> {
    let authorization = oauth2_authorization_header(bearer_token);
    crate::raw::json(url, query_options, data, &authorization).await
}

pub async fn put(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    bearer_token: &str,
) -> Result<Response, Error> {
    let authorization = oauth2_authorization_header(bearer_token);
    crate::raw::put(url, query_options, &authorization).await
}

pub async fn delete(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    bearer_token: &str,
) -> Result<Response, Error> {
    let authorization = oauth2_authorization_header(bearer_token);
    crate::raw::delete(url, query_options, &authorization).await
}

pub async fn multipart(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    data: Form,
    bearer_token: &str,
) -> Result<Response, Error> {
    let authorization = oauth2_authorization_header(bearer_token);
    crate::raw::multipart(url, query_options, data, &authorization).await
}

#[cfg(test)]
mod tests {
    use crate::*;
    use serde_json::Value;
    use std::env;

    #[tokio::test]
    async fn test_api() {
        let consumer_key = env::var("CONSUMER_KEY").unwrap();
        let consumer_secret = env::var("CONSUMER_SECRET").unwrap();
        let bearer_token = oauth::get_bearer_token(&consumer_key, &consumer_secret)
            .await
            .unwrap();

        // search
        let res: Value = v2::get(
            "https://api.twitter.com/1.1/search/tweets.json",
            &vec![("q", "東京&埼玉"), ("count", "2")],
            &bearer_token,
        )
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
        println!("{:?}", res);
    }
}
