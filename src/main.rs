use native_tls::{TlsConnector, TlsStream};
use std::fmt;
use std::io::{Read, Write};
use std::iter::Iterator;
use std::net::{TcpStream, ToSocketAddrs}; // TODO: might be not too bad to make a TLS connector?

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 1 {
        println!("Need to supply a URL");
        return;
    }
    let host = args[1].clone();

    let content = match get_content(host) {
        Ok(c) => c,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };

    println!("{}", content.header);
    content
        .body
        .expect("test code")
        .iter()
        .for_each(|line| println!("{}", line));
    // TODO: handle the redirect
}

fn get_content(host: String) -> Result<Response, String> {
    let url = format!("{}:1965", host);
    // TODO: move the TLS stuff into a separate method
    // TODO: trust certs on first use
    let mut builder = TlsConnector::builder();
    builder.danger_accept_invalid_hostnames(true);
    builder.danger_accept_invalid_certs(true);
    let connector = builder.build().unwrap();

    match url.to_socket_addrs() {
        Ok(mut addr_iter) => match addr_iter.next() {
            Some(addr) => {
                let stream = match TcpStream::connect(addr) {
                    Ok(s) => s,
                    Err(e) => return Err(format!("Error with the TCP connection: {:?}", e)),
                };
                println!("connected to a stream");

                let tls_stream = connector.connect(&host, stream);
                match tls_stream {
                    Ok(mut stream) => {
                        println!("tls connector connected");
                        stream
                            .write_all(format!("gemini://{}\r\n", url).as_bytes())
                            .unwrap();
                        Response::from(stream)
                    }
                    Err(e) => Err(format!("Error with the TLS connector: {:?}", e)),
                }
            }
            None => Err(String::from("No addresses found")),
        },
        Err(e) => Err(format!("Error getting socket addresses: {:?}", e)),
    }
}

// A gemini response details can be found in the gemini spec:
// https://gemini.circumlunar.space/docs/specification.gmi
struct Response {
    header: Header,
    body: Option<Vec<String>>,
}

impl Response {
    pub fn from(mut stream: TlsStream<TcpStream>) -> Result<Response, String> {
        let mut content = String::new();
        stream
            .read_to_string(&mut content)
            .or_else(|err| Err(format!("{}", err)))?;
        let mut lines = content
            .split("\r\n")
            .map(String::from)
            .collect::<Vec<String>>();

        Ok(Response {
            header: Header::parse_header(lines.remove(0))?, // TODO: no clone? we should be able to move this?
            body: Some(lines),                              // TODO: sometimes there won't be a body
        })
    }
}

enum Header {
    Input(u8, String),                     // Input prompt
    Success(u8, String),                   // MIME media type
    Redirect(u8, String),                  // The Redirect Url
    TemporaryFailure(u8, String),          // User facing error message
    PermanentFailure(u8, String),          // User facing error message
    ClientCertificateRequired(u8, String), // Error message
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

        match status_code {
            10..=19 => Ok(Header::Input(status_code, meta)),
            20..=29 => Ok(Header::Success(status_code, meta)),
            30..=39 => Ok(Header::Redirect(status_code, meta)),
            40..=49 => Ok(Header::TemporaryFailure(status_code, meta)),
            50..=59 => Ok(Header::PermanentFailure(status_code, meta)),
            60..=69 => Ok(Header::ClientCertificateRequired(status_code, meta)),
            _ => Err(format!("Unexpected status code: {}", status_code)),
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Is there really not a more succinct way to do this?
        // Perhaps this means a struct would be better suited than an Enum...
        match self {
            Header::Input(code, meta) => write!(f, "{} {}\r\n", code, meta),
            Header::Success(code, meta) => write!(f, "{} {}\r\n", code, meta),
            Header::Redirect(code, meta) => write!(f, "{} {}\r\n", code, meta),
            Header::TemporaryFailure(code, meta) => write!(f, "{} {}\r\n", code, meta),
            Header::PermanentFailure(code, meta) => write!(f, "{} {}\r\n", code, meta),
            Header::ClientCertificateRequired(code, meta) => write!(f, "{} {}\r\n", code, meta),
        }
    }
}
