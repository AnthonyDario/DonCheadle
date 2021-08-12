use std::str;

// URI syntax is the same as RFC-3986: https://datatracker.ietf.org/doc/html/rfc3986/
// URI grammer scheme
// URI          = scheme ":" hier-part [ "?" query ] [ "#" fragment ]
// hier-part    = "//" authority path-abempty
//                / path-abosolute
//                / path-rootless
//                / path-empty
// authority    = [ userinfo "@" ] host [ ":" port ]
// query        = *( pchar / "/" / "?")
// fragment     = *( pchar / "/" / "?")
pub fn parse(raw_uri: &[u8]) -> Result<(), String> {
    // first we percent decode the bytes
    decode(raw_uri)?;
    Ok(())
}

pub fn decode(raw_uri: &[u8]) -> Result<Vec<u8>, String> {
    let mut decoded_bytes = Vec::new();
    let mut i = 0;
    while i < raw_uri.len() {
        if i + 2 < raw_uri.len() && is_percent_encoded(&raw_uri[i..=i + 2]) {
            let hex_bytes = [raw_uri[i + 1], raw_uri[i + 2]];
            let hex = match str::from_utf8(&hex_bytes) {
                Ok(hex) => hex,
                Err(err) => {
                    return Err(format!("Error parsing the hex string: {}", err.to_string()))
                }
            };

            let decoded_byte = match u8::from_str_radix(hex, 16) {
                Ok(byte) => byte,
                Err(err) => {
                    return Err(format!(
                        "Error converting hex to a byte: {}",
                        err.to_string()
                    ))
                }
            };

            decoded_bytes.push(decoded_byte);
            i += 3;
        } else {
            decoded_bytes.push(raw_uri[i]);
            i += 1;
        }
    }
    Ok(decoded_bytes)
}

// Takes in three bytes and determines if it is a percent encoded sequence
fn is_percent_encoded(bytes: &[u8]) -> bool {
    if bytes[0] != b'%' {
        return false;
    }
    return bytes[1].is_ascii_hexdigit() && bytes[2].is_ascii_alphanumeric();
}

pub struct Uri<'a> {
    scheme: Option<&'a str>,
    host: &'a str,
    port: usize,
    path: Option<&'a str>,
    query: Option<&'a str>,
    fragment: Option<&'a str>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uri_decode_no_encodings() {
        let input = "foo://example.com:8042/over%%/there?name=ferret#nose%4";

        let decoded_bytes = decode(input.as_bytes()).unwrap();
        let decoded = str::from_utf8(&decoded_bytes).unwrap();

        assert_eq!(input, decoded);
    }

    #[test]
    fn some_encodings() {
        let input = "foo://ex%20ample.c%29om:8042/ove%23r/the%2cre?name=ferret#nose";
        let expected = "foo://ex ample.c)om:8042/ove#r/the,re?name=ferret#nose";

        let decoded_bytes = decode(input.as_bytes()).unwrap();
        let decoded = str::from_utf8(&decoded_bytes).unwrap();

        assert_eq!(expected, decoded);
    }
}
