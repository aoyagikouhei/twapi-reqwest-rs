# twapi-reqwest-rs

Twitter OAuth library used by reqwest.

[Documentation](https://docs.rs/twapi-reqwest)

## Features
- Async/Await
- Application Only Authentication
- User Authentication
- Oauth1.0 Authentication
- Oauth2.0 Authentication
- JSON support(ex. dm_event, welcome_message, media_metadata, etc.)
- Multipart support(ex. post_media_upload)

## Changes

### v0.2.1 (2021/09/14)
* oauth parse_oauth_body add original body to HashMap

### v0.2.0 (2021/03/26)
* updated reqwest 0.11
* add struct Client

## Example
```rust
use twapi_reqwest::*;
use std::env;
use std::io::{BufReader, Cursor, Read};

#[tokio::main]
async fn main() {
    // OAuth2.0 Authentication
    let consumer_key = env::var("CONSUMER_KEY").unwrap();
    let consumer_secret = env::var("CONSUMER_SECRET").unwrap();
    let bearer_token = oauth::get_bearer_token(&consumer_key, &consumer_secret).await.unwrap().unwrap();

    // search(Application Only Authentication)
    let res: serde_json::Value = v2::get(
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

    // OAuth1.0 Authentication
    let consumer_key = env::var("CONSUMER_KEY").unwrap();
    let consumer_secret = env::var("CONSUMER_SECRET").unwrap();
    let access_key = env::var("ACCESS_KEY").unwrap();
    let access_secret = env::var("ACCESS_SECRET").unwrap();

    // home_timeline
    let url = "https://api.twitter.com/1.1/statuses/home_timeline.json";
    let query_options = vec![("count", "2")];
    let res: serde_json::Value = v1::get(
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
    let res: serde_json::Value = v1::post(
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

    // direct_messages new(JSON support)
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
    let data: serde_json::Value = serde_json::from_str(data).unwrap();
    let res: serde_json::Value = v1::json(
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

    // media/upload(Multipart support)
    let metadata = std::fs::metadata("test.jpg").unwrap();
    let file_size = metadata.len();
    let f = std::fs::File::open("test.jpg").unwrap();
    let mut cursor = Cursor::new(vec![0; file_size as usize]);
    let mut reader = BufReader::new(f);
    reader.read(cursor.get_mut()).unwrap();

    let part = reqwest::multipart::Part::bytes(cursor.into_inner());
    let data = reqwest::multipart::Form::new().part("media", part);
    let url = "https://upload.twitter.com/1.1/media/upload.json";
    let res: serde_json::Value = v1::multipart(
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
}
```