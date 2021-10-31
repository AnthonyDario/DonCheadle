use std::io::{stdin, stdout, Write};
use std::process::exit;

mod client;

use crate::client::{Response, visit_url};

fn main() -> std::io::Result<()> {
    println!("Enter (q) to quit at any time\n\
              Enter a gemini URL to visit that page");
    loop {
        print!("Enter URL: ");
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;
        match input.trim() {
            "q" => exit(0),
            _ => { 
                match get_content(input) {
                    Ok(gem_text) => display_response(gem_text),
                    Err(e) => exit_with_error(format!("Error retrieving content: {}", e)),
                }
            }
        }
    }
}

fn get_content(input: String) -> Result<Response, String> {
    let content = match visit_url(input.clone()) {
        Ok(content) => content,
        Err(e) => {
            println!("Error: {:?}", e);
            return Err(format!("Error visiting url {}: {}", input, e));
        }
    };
    return Ok(content);
}

fn display_response(response: Response) {
    response
        .body
        .expect("Empty Response")
        .iter()
        .for_each(|line| println!("{}", line))
}

fn exit_with_error(error_message: String) {
    println!("{}", error_message);
    exit(1);
}
