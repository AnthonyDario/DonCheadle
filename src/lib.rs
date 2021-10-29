mod client;

use crate::client::visit_url;
use crate::client::Response;

pub fn go(url: &str) -> Result<Response, String> {
    return visit_url(url.to_string());
}
