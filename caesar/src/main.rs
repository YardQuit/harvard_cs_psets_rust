use std::io::{self, Write};

fn main() {
    let text = get_text();
}

fn get_text() -> String {
    loop {
        let mut prompt_text: String = String::new();
        print!("Key: ");
        io::stdout().flush().expect("error: failed to flush stdout");
        io::stdin()
            .read_line(&mut prompt_text)
            .expect("error: failed to read from stdin");
    }
}
