use std::io::{stdin, stdout, Write};
use std::process::exit;

mod gemtext;
mod client;
mod url;

use crate::client::visit_url;
use crate::gemtext::GemText;

fn main() -> std::io::Result<()> {
    println!(
        "Enter (q) to quit at any time\n\
         Enter a gemini URL to visit that page"
    );
    loop {
        print!("Enter URL: ");
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;
        match input.trim() {
            "q" => exit(0),
            _ => match get_content(input) {
                Ok(gemtext) => display_response(gemtext),
                Err(e) => exit_with_error(format!("Error retrieving content: {}", e)),
            },
        }
    }
}

fn get_content(input: String) -> Result<GemText, String> {
    let response = visit_url(input.clone())?;
    return Ok(GemText::from_response(response)?);
}

fn display_response(gemtext: GemText) {
    println!("{}", gemtext.content);
}

fn exit_with_error(error_message: String) {
    println!("{}", error_message);
    exit(1);
}
