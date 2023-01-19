use std::env;
use std::io::{self, Write};

fn main() {
    let key: u32 = get_key();
    let text: String = get_text();
    println!("ciphertext: {}", get_cipher(&text, key));
}

fn get_text() -> String {
    let mut prompt: String = String::new();
    print!("plaintext:  ");
    io::stdout().flush().expect("error: failed to flush stdout");
    io::stdin()
        .read_line(&mut prompt)
        .expect("error: failed to read from stdin");
    prompt.to_string()
}

fn get_key() -> u32 {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <cipher-key>", &args[0]);
        std::process::exit(1);
    } else {
        match args[1].trim().parse::<u32>() {
            Ok(value) => value,
            Err(_) => {
                println!("usage: {} <cipher-key>", &args[0]);
                std::process::exit(1);
            }
        }
    }
}

fn get_cipher(text: &str, key: u32) -> String {
    let mut cipher_text: String = String::new();
    let lower_az = vec!(
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    );
    let upper_az = vec!(
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
    );

    for char in text.chars() {
        if char.is_ascii_lowercase() {
            let calc = ((char as u32 - 97) + key) % 26;
            cipher_text.push(lower_az[calc as usize]);
        } else if char.is_ascii_uppercase() {
            let calc = ((char as u32 - 65) + key) % 26;
            cipher_text.push(upper_az[calc as usize]);
        } else {
            cipher_text.push(char);
        }
    }
    cipher_text
}
