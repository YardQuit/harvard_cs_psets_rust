use std::env;
use std::io::{self, Write};

fn main() {
    let substitution: String = get_substitution();
    let text: String = get_text();
    println!("ciphertext: {}", get_cipher(&text, &substitution));
}

fn get_substitution() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} key", &args[0]);
        std::process::exit(1);
    }
    if args[1].len() != 26 {
        println!("key must contain 26 characters");
        std::process::exit(1);
    } else {
        args[1].to_string()
    }
}

fn get_text() -> String {
    let mut prompt: String = String::new();
    print!("plaintext:  ");
    io::stdout().flush().expect("error: failed to flush stdout");
    io::stdin()
        .read_line(&mut prompt)
        .expect("error: faild to read from stdin");
    prompt.to_string()
}

fn get_cipher(text: &str, substitution: &str) -> String {
    let lower_substitution: Vec<char> = (substitution[0..])
        .chars()
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();
    let upper_substitution: Vec<char> = (substitution[0..])
        .chars()
        .map(|c| c.to_uppercase().next().unwrap())
        .collect();

    let mut cipher_text = String::new();

    for char in text.chars() {
        if char.is_ascii_lowercase() {
            let calc: u8 = (char as u8 - 97) % 26;
            cipher_text.push(lower_substitution[calc as usize]);
        } else if char.is_ascii_uppercase() {
            let calc: u8 = (char as u8 - 65) % 26;
            cipher_text.push(upper_substitution[calc as usize]);
        } else {
            cipher_text.push(char);
        }
    }
    cipher_text
}
