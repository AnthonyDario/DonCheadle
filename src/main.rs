use std::io::{Read, Write};
use std::net::{ToSocketAddrs, TcpStream};
use native_tls::TlsConnector;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url = args[1].clone();
    let url_port = format!("{}:1965", url);

    let mut builder = TlsConnector::builder();
    builder.danger_accept_invalid_hostnames(true);
    builder.danger_accept_invalid_certs(true);
    let connector = builder.build().unwrap();

    match url_port.to_socket_addrs() {
        Ok(addr_iter) => addr_iter.for_each(|addr| {
            let stream = TcpStream::connect(addr).unwrap();

            let tls_stream = connector.connect(&url, stream);
            match tls_stream {
                Ok(mut stream) => {
                    stream.write_all(format!("gemini://{}\r\n", url).as_bytes()).unwrap();
                    let mut response = String::new();
                    stream.read_to_string(&mut response).unwrap();
                    println!("buf: {}", response);
                }
                Err(e) => println!("Error with the TLS connector: {:?}", e)
            }
        }),
        Err(e) => println!("Error: {}", e),
    }
}
