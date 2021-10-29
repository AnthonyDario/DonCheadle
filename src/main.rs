mod client;

use crate::client::visit_url;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 1 {
        println!("Need to supply a URL");
        return;
    }
    let host = args[1].clone();

    let content = match visit_url(host) {
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
