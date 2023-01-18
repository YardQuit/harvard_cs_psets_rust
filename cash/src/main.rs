use std::io::{self, Write};

fn main() {
    let mut cents: u32 = get_cents();

    let quarters: u32 = calculate_quarters(&cents);
    cents -= quarters * 25;

    let dimes: u32 = calculate_dimes(&cents);
    cents -= dimes * 10;

    let nickels: u32 = calculate_nickels(&cents);
    cents -= nickels * 5;

    let pennies: u32 = calculate_pennies(&cents);

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
            .expect("error: failed to flush output");
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
fn calculate_quarters(cents: &u32) -> u32 {
    cents / 25
}

fn calculate_dimes(cents: &u32) -> u32 {
    cents / 10
}

fn calculate_nickels(cents: &u32) -> u32 {
    cents / 5
}

fn calculate_pennies(cents: &u32) -> u32 {
    *cents
}
