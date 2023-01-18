use std::io::{self, Write};

fn main() {
    let height: u8 = get_height();
    print_block(&height);
}

fn get_height() -> u8 {
    loop {
        // get user input
        let mut height = String::new();
        print!("Height? ");
        io::stdout().flush().expect("error: failed to flush stdout");
        io::stdin()
            .read_line(&mut height)
            .expect("error: failed to read from stdin");

        // convert input string to a u8 integer
        let height: u8 = match height.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        // check for valid range of integers
        if (1..=8).contains(&height) {
            return height;
        } else {
            // continue to loop
            continue;
       }
    }
}

fn print_block(height: &u8) {
    for i in 0..*height {
        for _ in 0..height - i - 1 {
            print!(" ");
        }
        for _ in 0..i + 1 {
            print!("#");
        }
        println!();
    }
}
