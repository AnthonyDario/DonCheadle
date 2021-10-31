use std::io::{stdin, stdout, Write};
use std::process::exit;

mod client;
mod gemtext;

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
    let content = match visit_url(input.clone()) {
        Ok(content) => GemText::from_response(content),
        Err(e) => {
            println!("Error: {:?}", e);
            return Err(format!("Error visiting url {}: {}", input, e));
        }
    };
    return Ok(content);
}

fn display_response(gemtext: GemText) {
    println!("{}", gemtext.content);
}

fn exit_with_error(error_message: String) {
    println!("{}", error_message);
    exit(1);
}
