use reqwest::{
    blocking::{Client, multipart::Form, Response},
    Error,
};
use serde_json::Value;

pub(crate) fn get(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    authorization: &str,
) -> Result<Response, Error> {
    let client = Client::new();
    client
        .get(url)
        .header("Authorization", authorization)
        .query(query_options)
        .send()
}

pub(crate) fn post(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    form_options: &Vec<(&str, &str)>,
    authorization: &str,
) -> Result<Response, Error> {
    let client = Client::new();
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
}

pub(crate) fn json(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    data: Value,
    authorization: &str,
) -> Result<Response, Error> {
    let client = Client::new();
    client
        .post(url)
        .header("Authorization", authorization)
        .header("Content-Type", "application/json")
        .query(query_options)
        .json(&data)
        .send()
}

pub(crate) fn put(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    authorization: &str,
) -> Result<Response, Error> {
    let client = Client::new();
    client
        .put(url)
        .header("Authorization", authorization)
        .query(query_options)
        .send()
}

pub(crate) fn delete(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    authorization: &str,
) -> Result<Response, Error> {
    let client = Client::new();
    client
        .delete(url)
        .header("Authorization", authorization)
        .query(query_options)
        .send()
}

pub(crate) fn multipart(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    data: Form,
    authorization: &str,
) -> Result<Response, Error> {
    let client = Client::new();
    client
        .post(url)
        .header("Authorization", authorization)
        .query(query_options)
        .multipart(data)
        .send()
}