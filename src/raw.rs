use reqwest::{multipart::Form, Error, Response};
use serde_json::Value;
use std::time::Duration;
use twapi_oauth::encode;

use crate::build_client;

fn make_query(list: &Vec<(&str, &str)>, separator: &str) -> String {
    let mut result = String::from("");
    for item in list {
        if "" != result {
            result.push_str(separator);
        }
        result.push_str(&format!("{}={}", item.0, encode(item.1)));
    }
    result
}
pub(crate) async fn get(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    authorization: &str,
    timeout_sec: Option<Duration>,
) -> Result<Response, Error> {
    let url = if query_options.len() > 0 {
        format!("{}?{}", url, make_query(query_options, "&"))
    } else {
        url.to_owned()
    };
    let client = build_client(timeout_sec);
    client
        .get(&url)
        .header("Authorization", authorization)
        .send()
        .await
}

pub(crate) async fn post(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    form_options: &Vec<(&str, &str)>,
    authorization: &str,
    timeout_sec: Option<Duration>,
) -> Result<Response, Error> {
    let client = build_client(timeout_sec);
    client
        .post(url)
        .header("Authorization", authorization)
        .header(
            "Content-Type",
            "application/x-www-form-urlencoded;charset=UTF-8",
        )
        .query(query_options)
        .body(crate::make_body(form_options))
        .send()
        .await
}

pub(crate) async fn json(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    data: &Value,
    authorization: &str,
    timeout_sec: Option<Duration>,
) -> Result<Response, Error> {
    let client = build_client(timeout_sec);
    client
        .post(url)
        .header("Authorization", authorization)
        .header("Content-Type", "application/json")
        .query(query_options)
        .json(&data)
        .send()
        .await
}

pub(crate) async fn put(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    authorization: &str,
    timeout_sec: Option<Duration>,
) -> Result<Response, Error> {
    let client = build_client(timeout_sec);
    client
        .put(url)
        .header("Authorization", authorization)
        .query(query_options)
        .send()
        .await
}

pub(crate) async fn delete(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    authorization: &str,
    timeout_sec: Option<Duration>,
) -> Result<Response, Error> {
    let client = build_client(timeout_sec);
    client
        .delete(url)
        .header("Authorization", authorization)
        .query(query_options)
        .send()
        .await
}

pub(crate) async fn multipart(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    data: Form,
    authorization: &str,
    timeout_sec: Option<Duration>,
) -> Result<Response, Error> {
    let client = build_client(timeout_sec);
    client
        .post(url)
        .header("Authorization", authorization)
        .query(query_options)
        .multipart(data)
        .send()
        .await
}
