use reqwest::{
    Error,
    blocking::{Response},
};
use twapi_oauth::{oauth2_authorization_header};

pub fn get(
    url: &str,
    query_options: &Vec<(&str, &str)>,
    bearer_token: &str,
) -> Result<Response, Error> {
    let authorization = oauth2_authorization_header(bearer_token);
    crate::blocking::raw::get(url, query_options, &authorization)
}

#[cfg(testx)]
mod tests {
    use crate::blocking::*;
    use std::env;
    use serde_json::Value;

    #[test]
    fn test_api() {
        let consumer_key = env::var("CONSUMER_KEY").unwrap();
        let consumer_secret = env::var("CONSUMER_SECRET").unwrap();
        let bearer_token = oauth::get_bearer_token(&consumer_key, &consumer_secret)
            .unwrap();

        // search
        let res: Value = v2::get(
            "https://api.twitter.com/1.1/search/tweets.json",
            &vec![("q", "東京&埼玉"), ("count", "2")],
            &bearer_token,
        )
        .unwrap()
        .json()
        .unwrap();
        println!("{:?}", res);
    }
}