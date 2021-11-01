use native_tls::{TlsConnector, TlsStream};
use std::fmt;
use std::io::{Read, Write};
use std::iter::Iterator;
use std::net::TcpStream;
use url::Url;

use crate::url::parse_url;

pub fn visit_url(mut url_string: String) -> Result<Response, String> {
    let mut content: Response;

    loop {
        content = get_content(parse_url(&url_string)?)?;
        if content.header.code < 30 || content.header.code > 39 {
            break;
        }
        url_string = content.header.meta;
    }

    return Ok(content);
}

fn get_content(url: Url) -> Result<Response, String> {
    let mut builder = TlsConnector::builder();
    builder.danger_accept_invalid_hostnames(true);
    builder.danger_accept_invalid_certs(true);
    let connector = builder.build().unwrap();

    match url.socket_addrs(|| Some(1965)) {
        Ok(addr_vec) => {
            let stream = match TcpStream::connect(addr_vec[0]) {
                Ok(s) => s,
                Err(e) => return Err(format!("Error with the TCP connection: {:?}", e)),
            };

            let tls_stream = connector.connect(&url.host_str().unwrap(), stream);
            match tls_stream {
                Ok(mut stream) => {
                    stream.write_all(format!("{}\r\n", url).as_bytes()).unwrap();
                    Response::from(stream)
                }
                Err(e) => Err(format!("Error with the TLS connector: {:?}", e)),
            }
        }
        Err(e) => Err(format!("Error getting socket addresses: {:?}", e)),
    }
}

// Gemini response details can be found in the gemini spec:
// https://gemini.circumlunar.space/docs/specification.gmi
pub struct Response {
    pub header: Header,
    pub body: Option<Vec<String>>,
}

impl Response {
    pub fn from(mut stream: TlsStream<TcpStream>) -> Result<Response, String> {
        let mut content = String::new();
        stream
            .read_to_string(&mut content)
            .or_else(|err| Err(format!("{}", err)))?;
        let mut lines = content
            .split("\n")
            .map(String::from)
            .collect::<Vec<String>>();

        Ok(Response {
            header: Header::parse_header(lines.remove(0))?,
            body: Some(lines),
        })
    }
}

pub struct Header {
    pub name: HeaderName,
    pub code: u8,
    pub meta: String,
}

pub enum HeaderName {
    Input,                     // Input prompt
    Success,                   // MIME media type
    Redirect,                  // The Redirect Url
    TemporaryFailure,          // User facing error message
    PermanentFailure,          // User facing error message
    ClientCertificateRequired, // Error message
}

impl Header {
    // A gemini header is formatted as: <STATUS><SPACE><META><CR><LF>
    pub fn parse_header(header: String) -> Result<Header, String> {
        let mut header_iter = header.split_whitespace();
        let status_code = header_iter
            .next()
            .ok_or("No status code found in header")?
            .parse::<u8>()
            .or_else(|err| Err(format!("{}", err)))?;
        let meta = header_iter
            .next()
            .ok_or("No meta found in header")?
            .to_string();

        let name = match status_code {
            10..=19 => HeaderName::Input,
            20..=29 => HeaderName::Success,
            30..=39 => HeaderName::Redirect,
            40..=49 => HeaderName::TemporaryFailure,
            50..=59 => HeaderName::PermanentFailure,
            60..=69 => HeaderName::ClientCertificateRequired,
            _ => return Err(format!("Unexpected status code: {}", status_code)),
        };

        Ok(Header {
            name: name,
            meta: meta,
            code: status_code,
        })
    }
}

impl fmt::Display for Header {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}\r\n", self.code, self.meta)
    }
}
