use std::io::{self, Write};
use std::env;

fn main() {
    let key = get_key();
    let text = get_text();
    println!("{} {}", key, text);
}

fn get_text() -> String {
    let mut prompt: String = String::new();
    print!("plaintext  : ");
    io::stdout().flush().expect("error: failed to flush stdout");
    io::stdin()
        .read_line(&mut prompt)
        .expect("error: failed to read from stdin");
    prompt.to_string()
}

fn get_key() -> String {
    let args: Vec::<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <cipher-key>", &args[0]);
        std::process::exit(1);
        } else {
        (args[1]).to_string()
    }

}