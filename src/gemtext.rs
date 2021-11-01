use crate::client::Response;
use crate::url::parse_url;
use std::collections::HashMap;
use url::Url;

static EMPTY_BODY: &str = "No Body";

pub struct GemText {
    pub content: String,
    pub links: HashMap<u32, Url>,
}

impl GemText {
    pub fn from_response(response: Response) -> Result<GemText, String> {
        let mut link_counter: u32 = 0;
        let mut links = HashMap::new();

        let content = match response.body {
            Some(body) => body.iter().fold(String::from(""), |acc, s| {
                if is_gemtext_link(s) {
                    match GemTextLink::parse(s.to_string()) {
                        Ok(link) => {
                            links.insert(link_counter += 1, link.url);
                            format!("{}{}\n", acc, link.label)
                        }
                        Err(_) => format!("{}{}\n", acc, s) // TODO: probably need to log this error
                    }
                } else {
                    format!("{}{}\n", acc, s)
                }
            }),
            None => EMPTY_BODY.to_string(),
        };

        return Ok(GemText {
            content: content,
            links: HashMap::new(),
        });
    }
}

struct GemTextLink {
    pub url: Url,
    pub label: String,
}

impl GemTextLink {

    // Link Format:
    // =>   gemini://circumlunar.space    gemini project page
    //    | |---------url------------|  |  |------label------|
    //    |                             |
    // optional whitespace          whitespace
    fn parse(line: String) -> Result<GemTextLink, String> {
        if !line.starts_with("=>") { return Err("Link that didn't start with =>".to_string()) };

        let mut parts = line.strip_prefix("=>").unwrap().split_whitespace();
        let url = match parts.next() {
            None => return Err("Link without a URL".to_string()),
            Some(url) => parse_url(&url.to_string())?,
        };

        let mut label = String::new();
        parts.for_each(|word| label.push_str(&format!("{} ", word)));

        Ok(GemTextLink {
            url: url,
            label: label,
        })
    }
}

fn is_gemtext_link(line: &String) -> bool {
    return line.starts_with("=>");
}
