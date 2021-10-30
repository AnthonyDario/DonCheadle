use std::io::{stdin, stdout, Write};
use std::process::exit;

mod client;

use crate::client::{Response, visit_url};

fn main() {
    println!("Enter q to quit at any time");
    println!("Enter a gemini URL to visit that page");
    loop {
        print!("Enter URL: ");
        let mut input = String::new();
        stdout().flush();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim() {
                    "q" => exit(1),
                    _ => { 
                        match parse_input(input) {
                            Ok(gem_text) => display_input(gem_text),
                            Err(e) => exit(0),
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                exit(0);
            }
        }
    }
}

fn parse_input(input: String) -> Result<Response, String> {

    let content = match visit_url(input.clone()) {
        Ok(content) => content,
        Err(e) => {
            println!("Error: {:?}", e);
            return Err(format!("Error visiting url {}: {}", input, e));
        }
    };
    return Ok(content);
}

fn display_input(input: Response) {
    input
        .body
        .expect("Empty Response")
        .iter()
        .for_each(|line| println!("{}", line))
}
