use std::io::{stdin, stdout, Write};
use std::process::exit;
mod client;
use crate::client::visit_url;

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
                    _ => parse_input(input),
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                exit(0);
            }
        }
    }
}

fn parse_input(input: String) {

    let content = match visit_url(input) {
        Ok(content) => content,
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
}
