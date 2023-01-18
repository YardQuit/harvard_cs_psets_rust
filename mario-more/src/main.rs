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
            .expect("error: failed to read form stdin");

        // convert string to u8 integer
        let height: u8 = match height.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        // check input integer be in a valid range
        if (1..=8).contains(&height) {
            return height;
        } else {
            continue;
        };
    }
}

fn print_block(height: &u8) {
    println!("hello");
    for i in 0..*height {
        for _ in 0..height - i - 1 {
            print!(" ");
        }
        for _ in 0..i + 1 {
            print!("#");
        }
        print!(" ");
        for _ in 0..i + 1 {
            print!("#");
        }
        println!();
    }
}
