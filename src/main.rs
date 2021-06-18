use native_tls::{TlsConnector, TlsStream};
use std::io::{Read, Write};
use std::iter::Iterator;
use std::net::{TcpStream, ToSocketAddrs}; // TODO: might be not too bad to make a TLS connector?

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // TODO: check the arg length so we don't panic
    let host = args[1].clone();

    let content = get_content(host).expect("NO CONTENT ARRRRG"); // TODO: better error handling
    println!("{} {}", content.status_code, content.meta);
    content
        .body
        .expect("test code")
        .iter()
        .for_each(|line| println!("{}", line));
    // TODO: handle the redirect
}

fn get_content(host: String) -> Result<Response, String> {
    let url = format!("{}:1965", host);
    // TODO: move the TLS stuff into a separate own method
    // TODO: trust certs on first use
    let mut builder = TlsConnector::builder();
    builder.danger_accept_invalid_hostnames(true);
    builder.danger_accept_invalid_certs(true);
    let connector = builder.build().unwrap();

    // There should probably only be one socket address returned here?
    // would want to assert on that
    match url.to_socket_addrs() {
        Ok(mut addr_iter) => match addr_iter.next() {
            Some(addr) => {
                let stream = TcpStream::connect(addr).unwrap(); // TODO: error handling
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
                    Err(e) => {
                        println!("Error with the TLS connector: {:?}", e);
                        Err(format!("Error with the TLS connector: {:?}", e))
                    }
                }
            }
            None => Err(String::from("No addresses found")),
        },
        Err(e) => {
            println!("Error getting socket addresses: {:?}", e);
            Err(format!("Error getting socket addresses: {:?}", e))
        }
    }
}

// A gemini response details can be found in the gemini spec:
// https://gemini.circumlunar.space/docs/specification.gmi
struct Response {
    status_code: u8,
    meta: String,
    body: Option<Vec<String>>,
}

impl Response {
    pub fn from(mut stream: TlsStream<TcpStream>) -> Result<Response, String> {
        let mut content = String::new();
        stream
            .read_to_string(&mut content)
            .or_else(|err| Err(format!("{}", err)))?;
        let lines = content
            .split("\r\n")
            .map(String::from)
            .collect::<Vec<String>>();

        // The first line of the response is the header with the following format:
        //<STATUS><SPACE><META><CR><LF>
        let mut header_iter = lines[0].split_whitespace();
        let status_code = header_iter
            .next()
            .ok_or("No status code found in header")?
            .parse::<u8>()
            .or_else(|err| Err(format!("{}", err)))?;
        let header_meta = header_iter
            .next()
            .ok_or("No meta found in header")?
            .to_string();

        Ok(Response {
            status_code: status_code,
            meta: header_meta,
            body: Some(lines[1..].to_vec()), // TODO: sometimes there won't be a body
        })
    }
}
