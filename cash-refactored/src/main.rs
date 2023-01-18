use std::io::{self, Write};

fn main() {
    let cents: u32 = get_cents();
    let (quarters, dimes, nickels, pennies) = calculate_coins(&cents);
    let coins: u32 = quarters + dimes + nickels + pennies;

    println!("{} coins", coins);
}

fn get_cents() -> u32 {
    loop {
        // prompt for user input
        let mut prompt_cents = String::new();
        print!("Number of cents: ");
        io::stdout()
            .flush()
            .expect("error: faild to flush stdout");
        io::stdin()
            .read_line(&mut prompt_cents)
            .expect("error: failed to read from stdin");

        // convert user input from string to u32 integer
        let cents: u32 = match prompt_cents.trim().parse::<u32>() {
            Ok(parsed) => parsed,
            Err(_) => continue,
        };
        if cents > 0 && cents < 100 {
            return cents;
        } else {
            continue;
        }
     }
}

fn calculate_coins(cents: &u32) -> (u32, u32, u32, u32) {
    let quarters: u32 = cents / 25;
    let remaining_cents: u32 = cents % 25;

    let dimes: u32 = remaining_cents / 10;
    let remaining_cents: u32 = remaining_cents % 10;

    let nickels = remaining_cents / 5;
    let remaining_cents: u32 = remaining_cents % 5;

    let pennies: u32 = remaining_cents;

    (quarters, dimes, nickels, pennies)
}
