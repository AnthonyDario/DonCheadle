use crate::client::Response;
use std::collections::HashMap;
use url::Url;

static EMPTY_BODY: &str = "No Body";

pub struct GemText {
    pub content: String,
    pub links: HashMap<u32, Url>,
}

impl GemText {
    pub fn from_response(response: Response) -> GemText {
        let content = match response.body {
            Some(body) => body
                .iter()
                .fold(String::from(""), |acc, s| format!("{}{}", acc, s)),
            None => EMPTY_BODY.to_string(),
        };

        return GemText {
            content: content,
            links: HashMap::new(),
        };
    }
}
