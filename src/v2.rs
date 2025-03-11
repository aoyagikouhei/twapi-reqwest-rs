use reqwest::{multipart::Form, Error, Response};
use serde_json::Value;
use std::time::Duration;
use twapi_oauth::oauth2_authorization_header;

pub struct Client {
    bearer_token: String,
    timeout_sec: Option<Duration>,
}

impl Client {
    pub fn new(bearer_token: &str, timeout_sec: Option<Duration>) -> Self {
        Self {
            bearer_token: bearer_token.to_owned(),
            timeout_sec,
        }
    }

    pub async fn new_from_key(
        consumer_key: &str,
        consumer_secret: &str,
        timeout_sec: Option<Duration>,
    ) -> Result<Option<Self>, Error> {
        Ok(
            crate::oauth::get_bearer_token(&consumer_key, &consumer_secret, timeout_sec)
                .await?
                .map(|bearer_token| Self::new(&bearer_token, None)),
        )
    }

    pub async fn new_by_env(timeout_sec: Option<Duration>) -> Result<Option<Self>, Error> {
        let consumer_key = match std::env::var("CONSUMER_KEY") {
            Ok(consumer_key) => consumer_key,
            Err(_) => return Ok(None),
        };
        let consumer_secret = match std::env::var("CONSUMER_SECRET") {
            Ok(consumer_key) => consumer_key,
            Err(_) => return Ok(None),
        };
        Self::new_from_key(&consumer_key, &consumer_secret, timeout_sec).await
    }

    fn make_header(&self) -> String {
        oauth2_authorization_header(&self.bearer_token)
    }

    pub async fn get(
        &self,
        url: &str,
        query_options: &Vec<(&str, &str)>,
    ) -> Result<Response, Error> {
        crate::raw::get(url, query_options, &self.make_header(), self.timeout_sec).await
    }

    pub async fn post(
        &self,
        url: &str,
        query_options: &Vec<(&str, &str)>,
        form_options: &Vec<(&str, &str)>,
    ) -> Result<Response, Error> {
        crate::raw::post(
            url,
            query_options,
            form_options,
            &self.make_header(),
            self.timeout_sec,
        )
        .await
    }

    pub async fn json(
        &self,
        url: &str,
        query_options: &Vec<(&str, &str)>,
        data: &Value,
    ) -> Result<Response, Error> {
        crate::raw::json(
            url,
            query_options,
            data,
            &self.make_header(),
            self.timeout_sec,
        )
        .await
    }

    pub async fn put(
        &self,
        url: &str,
        query_options: &Vec<(&str, &str)>,
    ) -> Result<Response, Error> {
        crate::raw::put(url, query_options, &self.make_header(), self.timeout_sec).await
    }

    pub async fn delete(
        &self,
        url: &str,
        query_options: &Vec<(&str, &str)>,
    ) -> Result<Response, Error> {
        crate::raw::delete(url, query_options, &self.make_header(), self.timeout_sec).await
    }

    pub async fn multipart(
        &self,
        url: &str,
        query_options: &Vec<(&str, &str)>,
        data: Form,
    ) -> Result<Response, Error> {
        crate::raw::multipart(
            url,
            query_options,
            data,
            &self.make_header(),
            self.timeout_sec,
        )
        .await
    }
}

pub async fn get(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    bearer_token: &str,
    timeout_sec: Option<Duration>,
) -> Result<Response, Error> {
    let client = Client::new(bearer_token, timeout_sec);
    client.get(url, query_options).await
}

pub async fn post(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    form_options: &Vec<(&str, &str)>,
    bearer_token: &str,
    timeout_sec: Option<Duration>,
) -> Result<Response, Error> {
    let client = Client::new(bearer_token, timeout_sec);
    client.post(url, query_options, form_options).await
}

pub async fn json(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    data: &Value,
    bearer_token: &str,
    timeout_sec: Option<Duration>,
) -> Result<Response, Error> {
    let client = Client::new(bearer_token, timeout_sec);
    client.json(url, query_options, data).await
}

pub async fn put(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    bearer_token: &str,
    timeout_sec: Option<Duration>,
) -> Result<Response, Error> {
    let client = Client::new(bearer_token, timeout_sec);
    client.put(url, query_options).await
}

pub async fn delete(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    bearer_token: &str,
    timeout_sec: Option<Duration>,
) -> Result<Response, Error> {
    let client = Client::new(bearer_token, timeout_sec);
    client.delete(url, query_options).await
}

pub async fn multipart(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    data: Form,
    bearer_token: &str,
    timeout_sec: Option<Duration>,
) -> Result<Response, Error> {
    let client = Client::new(bearer_token, timeout_sec);
    client.multipart(url, query_options, data).await
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
        let bearer_token = oauth::get_bearer_token(&consumer_key, &consumer_secret, None)
            .await
            .unwrap()
            .unwrap();

        // search
        let res: Value = v2::get(
            "https://api.twitter.com/1.1/search/tweets.json",
            &vec![("q", "*abc"), ("count", "2")],
            &bearer_token,
            None,
        )
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
        println!("{:?}", res);
    }
}
