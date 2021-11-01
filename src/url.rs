use url::Url;

pub fn parse_url(url_string: &String) -> Result<Url, String> {
    let parse_string = if url_string.contains("gemini://") {
        url_string.clone()
    } else {
        format!("gemini://{}", url_string)
    };

    match Url::parse(parse_string.as_str()) {
        Ok(mut url) => {
            if url.set_port(Some(1965)).is_err() {
                return Err("Error setting the port".to_string());
            }
            Ok(url)
        }
        Err(e) => Err(format!("Error parsing Url '{}': {:?}", url_string, e)),
    }
}

