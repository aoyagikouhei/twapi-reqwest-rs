use reqwest::{multipart::Form, Error, Response};
use serde_json::Value;
use twapi_oauth::oauth1_authorization_header;

pub async fn get(
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
    crate::raw::get(url, query_options, &authorization).await
}

pub async fn post(
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
    crate::raw::post(url, query_options, form_options, &authorization).await
}

pub async fn json(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    data: &Value,
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
    crate::raw::json(url, query_options, data, &authorization).await
}

pub async fn put(
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
    crate::raw::put(url, query_options, &authorization).await
}

pub async fn delete(
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
    crate::raw::delete(url, query_options, &authorization).await
}

pub async fn multipart(
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
    crate::raw::multipart(url, query_options, data, &authorization).await
}

#[cfg(test)]
mod tests {
    use crate::*;
    use serde_json::Value;
    use std::env;
    use std::io::{BufReader, Cursor, Read};

    #[tokio::test]
    async fn test_api() {
        let consumer_key = env::var("CONSUMER_KEY").unwrap();
        let consumer_secret = env::var("CONSUMER_SECRET").unwrap();
        let access_key = env::var("ACCESS_KEY").unwrap();
        let access_secret = env::var("ACCESS_SECRET").unwrap();

        // search
        let url = "https://api.twitter.com/1.1/search/tweets.json";
        let query_options = vec![("q", "*abc"), ("count", "2")];
        let res: Value = v1::get(
            url,
            &query_options,
            &consumer_key,
            &consumer_secret,
            &access_key,
            &access_secret,
        )
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
        println!("{:?}", res);

        // home_timeline
        let url = "https://api.twitter.com/1.1/statuses/home_timeline.json";
        let query_options = vec![("count", "2")];
        let res: Value = v1::get(
            url,
            &query_options,
            &consumer_key,
            &consumer_secret,
            &access_key,
            &access_secret,
        )
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
        println!("{:?}", res);

        // statuses/update
        let url = "https://api.twitter.com/1.1/statuses/update.json";
        let form_options = vec![
            ("status", "!\"'#$%&\\()+,/:;<=>?@[\\]^`{|}~;-._* 全部"),
            ("in_reply_to_status_id", "1178811297455935488"),
        ];
        let res: Value = v1::post(
            url,
            &vec![],
            &form_options,
            &consumer_key,
            &consumer_secret,
            &access_key,
            &access_secret,
        )
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
        println!("{:?}", res);
/*
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
        let res: Value = v1::json(
            url,
            &vec![],
            &data,
            &consumer_key,
            &consumer_secret,
            &access_key,
            &access_secret,
        )
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
        println!("{:?}", res);

        // media/upload
        let metadata = std::fs::metadata("test.jpg").unwrap();
        let file_size = metadata.len();
        let f = std::fs::File::open("test.jpg").unwrap();
        let mut cursor = Cursor::new(vec![0; file_size as usize]);
        let mut reader = BufReader::new(f);
        reader.read(cursor.get_mut()).unwrap();

        let part = reqwest::multipart::Part::bytes(cursor.into_inner());
        let data = reqwest::multipart::Form::new().part("media", part);
        let url = "https://upload.twitter.com/1.1/media/upload.json";
        let res: Value = v1::multipart(
            url,
            &vec![],
            data,
            &consumer_key,
            &consumer_secret,
            &access_key,
            &access_secret,
        )
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
        println!("{:?}", res);
    */
    }
}