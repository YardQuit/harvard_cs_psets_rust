use std::io::{self, Write};

fn main() {
    let text: String = get_text();
    println!("{}", get_bulbs(&text));
}

fn get_text() -> String {
    let mut prompt = String::new();
    print!("message: ");
    io::stdout().flush().expect("error: failed to flush stdout");
    io::stdin()
        .read_line(&mut prompt)
        .expect("error: failed to read from stdin");
    prompt.trim().to_string()
}

fn get_bulbs(text: &str) -> String {
    let binary = text.as_bytes();
    let zero_emoji = "⚫";
    let one_emoji = "🟡";
    let mut bulbs: String = String::new();

    for b in binary {
        let binary_raw = format!("{:08b}", b);
        let binary_zero = binary_raw.replace('0', zero_emoji);
        let binary_zero_one = binary_zero.replace('1', one_emoji);
        bulbs = format!("{}\n{}", bulbs, binary_zero_one);
    }
    bulbs
}