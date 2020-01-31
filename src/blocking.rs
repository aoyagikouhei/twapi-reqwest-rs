use reqwest::{
    blocking::{multipart::Form, Response},
    Error,
};
use serde_json::Value;
use twapi_oauth::{oauth1_authorization_header, oauth2_authorization_header};

pub fn get(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    consumer_key: &str,
    consumer_secret: &str,
    access_key: &str,
    access_secret: &str,
) -> Result<Response, Error> {
    let authorization = oauth1_authorization_header(
        consumer_key,
        consumer_secret,
        access_key,
        access_secret,
        "GET",
        url,
        &query_options,
    );
    raw_get(url, query_options, &authorization)
}

pub fn get_v2(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    bearer_token: &str,
) -> Result<Response, Error> {
    let authorization = oauth2_authorization_header(bearer_token);
    raw_get(url, query_options, &authorization)
}

fn raw_get(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    authorization: &str,
) -> Result<Response, Error> {
    let client = reqwest::blocking::Client::new();
    client
        .get(url)
        .header("Authorization", authorization)
        .query(query_options)
        .send()
}

pub fn post(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    form_options: &Vec<(&str, &str)>,
    consumer_key: &str,
    consumer_secret: &str,
    access_key: &str,
    access_secret: &str,
) -> Result<Response, Error> {
    let mut merged_options = query_options.clone();
    for option in form_options {
        merged_options.push(*option);
    }
    let authorization = oauth1_authorization_header(
        consumer_key,
        consumer_secret,
        access_key,
        access_secret,
        "POST",
        url,
        &merged_options,
    );
    raw_post(url, query_options, form_options, &authorization)
}

fn raw_post(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    form_options: &Vec<(&str, &str)>,
    authorization: &str,
) -> Result<Response, Error> {
    let client = reqwest::blocking::Client::new();
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

pub fn json(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    data: Value,
    consumer_key: &str,
    consumer_secret: &str,
    access_key: &str,
    access_secret: &str,
) -> Result<Response, Error> {
    let authorization = oauth1_authorization_header(
        consumer_key,
        consumer_secret,
        access_key,
        access_secret,
        "POST",
        url,
        &query_options,
    );
    raw_json(url, query_options, data, &authorization)
}

fn raw_json(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    data: Value,
    authorization: &str,
) -> Result<Response, Error> {
    let client = reqwest::blocking::Client::new();
    client
        .post(url)
        .header("Authorization", authorization)
        .header("Content-Type", "application/json")
        .query(query_options)
        .json(&data)
        .send()
}

pub fn put(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    consumer_key: &str,
    consumer_secret: &str,
    access_key: &str,
    access_secret: &str,
) -> Result<Response, Error> {
    let authorization = oauth1_authorization_header(
        consumer_key,
        consumer_secret,
        access_key,
        access_secret,
        "PUT",
        url,
        &query_options,
    );
    raw_put(url, query_options, &authorization)
}

fn raw_put(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    authorization: &str,
) -> Result<Response, Error> {
    let client = reqwest::blocking::Client::new();
    client
        .put(url)
        .header("Authorization", authorization)
        .query(query_options)
        .send()
}

pub fn delete(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    consumer_key: &str,
    consumer_secret: &str,
    access_key: &str,
    access_secret: &str,
) -> Result<Response, Error> {
    let authorization = oauth1_authorization_header(
        consumer_key,
        consumer_secret,
        access_key,
        access_secret,
        "DELETE",
        url,
        &query_options,
    );
    raw_delete(url, query_options, &authorization)
}

fn raw_delete(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    authorization: &str,
) -> Result<Response, Error> {
    let client = reqwest::blocking::Client::new();
    client
        .delete(url)
        .header("Authorization", authorization)
        .query(query_options)
        .send()
}

pub fn multipart(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    data: Form,
    consumer_key: &str,
    consumer_secret: &str,
    access_key: &str,
    access_secret: &str,
) -> Result<Response, Error> {
    let authorization = oauth1_authorization_header(
        consumer_key,
        consumer_secret,
        access_key,
        access_secret,
        "POST",
        url,
        &query_options,
    );
    raw_multipart(url, query_options, data, &authorization)
}

fn raw_multipart(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    data: Form,
    authorization: &str,
) -> Result<Response, Error> {
    let client = reqwest::blocking::Client::new();
    client
        .post(url)
        .header("Authorization", authorization)
        .query(query_options)
        .multipart(data)
        .send()
}

pub fn get_bearer_token_response(
    consumer_key: &str,
    consumer_secret: &str,
) -> Result<Response, Error> {
    let key = base64::encode(&format!("{}:{}", consumer_key, consumer_secret));
    let client = reqwest::blocking::Client::new();
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

#[cfg(test)]
mod tests {
    use crate::blocking::*;
    use serde_json::Value;
    use std::env;

    #[test]
    fn test_api() {
        let consumer_key = env::var("CONSUMER_KEY").unwrap();
        let consumer_secret = env::var("CONSUMER_SECRET").unwrap();
        let access_key = env::var("ACCESS_KEY").unwrap();
        let access_secret = env::var("ACCESS_SECRET").unwrap();
        let bearer_token = get_bearer_token(&consumer_key, &consumer_secret).unwrap();

        // search
        let res: Value = get_v2(
            "https://api.twitter.com/1.1/search/tweets.json",
            &vec![("q", "東京&埼玉"), ("count", "2")],
            &bearer_token,
        )
        .unwrap()
        .json()
        .unwrap();
        println!("{:?}", res);

        // home_timeline
        let url = "https://api.twitter.com/1.1/statuses/home_timeline.json";
        let query_options = vec![("count", "2")];
        let res: Value = get(
            url,
            &query_options,
            &consumer_key,
            &consumer_secret,
            &access_key,
            &access_secret,
        )
        .unwrap()
        .json()
        .unwrap();
        println!("{:?}", res);

        // statuses/update
        let url = "https://api.twitter.com/1.1/statuses/update.json";
        let form_options = vec![
            ("status", "!\"'#$%&\\()+,/:;<=>?@[\\]^`{|}~;-._* 全部"),
            ("in_reply_to_status_id", "1178811297455935488"),
        ];
        let res: Value = post(
            url,
            &vec![],
            &form_options,
            &consumer_key,
            &consumer_secret,
            &access_key,
            &access_secret,
        )
        .unwrap()
        .json()
        .unwrap();
        println!("{:?}", res);

        // direct_messages new
        let url = "https://api.twitter.com/1.1/direct_messages/events/new.json";
        let data = r#"{
                    "event": {
                        "type": "message_create",
                        "message_create": {
                            "target": {
                                "recipient_id": "19522946"
                            },
                            "message_data": {
                                "text": "予定表〜①ﾊﾝｶｸだ!"
                            }
                        }
                    }
                }"#;
        let data: Value = serde_json::from_str(data).unwrap();
        let res: Value = json(
            url,
            &vec![],
            data,
            &consumer_key,
            &consumer_secret,
            &access_key,
            &access_secret,
        )
        .unwrap()
        .json()
        .unwrap();
        println!("{:?}", res);

        // media/upload
        let data = reqwest::blocking::multipart::Form::new()
            .file("media", "test.jpg")
            .unwrap();
        let url = "https://upload.twitter.com/1.1/media/upload.json";
        let res: Value = multipart(
            url,
            &vec![],
            data,
            &consumer_key,
            &consumer_secret,
            &access_key,
            &access_secret,
        )
        .unwrap()
        .json()
        .unwrap();
        println!("{:?}", res);
    }
}
