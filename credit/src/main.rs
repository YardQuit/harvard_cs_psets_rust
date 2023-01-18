use std::io::{self, Write};

fn main() {
    let (cardnumber, length): (u64, u8) = get_cardnumber();

    if checksum(cardnumber) {
        println!("checksum ok cardno: {} length: {}", cardnumber, length);
    } else {
        println!("Creditcard is invalid");
    }

    println!("{}", issuer_validation(cardnumber, &length));
}

fn get_cardnumber() -> (u64, u8) {
    loop {
        // get user input
        let mut prompt: String = String::new();
        print!("Enter Card-Number: ");
        io::stdout().flush().expect("error: failed to flush stdout");
        io::stdin()
            .read_line(&mut prompt)
            .expect("error: failed to read from stdin");

        // get length of string as integer
        let length: u8 = prompt.trim().len().try_into().unwrap();

        // convert string to integer
        match prompt.trim().parse::<u64>() {
            Ok(parsed) => return (parsed, length),
            Err(_) => continue,
        };
    }
}

fn checksum(cardnumber: u64) -> bool {
    // luhn's algorithm to check validity
    let mut cardnumber: u64 = cardnumber;
    let mut cardnumber_vec = Vec::new();

    // create a Vector of the cardnumber
    while cardnumber > 0 {
        let digit = cardnumber % 10;
        cardnumber_vec.push(digit);
        cardnumber /= 10;
    }
    cardnumber_vec.reverse();

    // multiply every other digit starting with the number second-to-last by 2.
    // if result is 10 or more, split value into two digits, e.g. 12 = 1 and 2.
    let mut algo_multi = 0;
    for (i, x) in cardnumber_vec.iter().enumerate() {
        if i % 2 == 0 {
            if x * 2 > 9 {
                let left = ((x * 2) / 10) % 10; // e.g. (12 / 10) % 10 = 1.2
                let right = (x * 2) % 10; // e.g. 12 % 10 = 2
                algo_multi += left + right;
            } else {
                algo_multi += x * 2;
            }
        }
    }

    // add every other digit starting with the first to last by 2.
    let mut algo_added = 0;
    for (i, x) in cardnumber_vec.iter().enumerate() {
        if i % 2 != 0 {
            algo_added += x;
        }
    }

    // verify checksum
    let algo_checksum = (algo_multi + algo_added) % 10;
    if algo_checksum != 0 {
        return false;
    }

    // pass checksum validation
    true
}

fn issuer_validation(cardnumber: u64, length: &u8) -> String {
    let visa_short = cardnumber / 1_000_000_000_000;
    let visa_long = cardnumber / 1_000_000_000_000_000;
    let mastercard = cardnumber / 100_000_000_000_000;
    let americanex = cardnumber / 10_000_000_000_000;

    match *length {
        13u8 => match visa_short {
            4 => "This is a valid Visa Card.".to_string(),
            _ => "Unknown creditcard issuer.".to_string(),
        },
        15u8 => match americanex {
            34 | 37 => "This is a valid American Express.".to_string(),
            _ => "Unknown creditcard issuer.".to_string(),
        },
        16u8 => match mastercard {
            50..=55 | 22..=27 => "This is a valid MasterCard.".to_string(),
            _ => match visa_long {
                4 => "This is a valid Visa Card.".to_string(),
                _ => "Unknown creditcard issuer.".to_string(),
            },
        },
        _ => "Unknowd creditcard issuer.".to_string(),
    }
}
