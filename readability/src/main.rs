use std::io::{self, Write};

fn main() {
    let text: String = get_text();
    let letters: u32 = count_letters(&text);
    let words: u32 = count_words(&text);
    let sentences: u32 = count_sentences(&text) - 1;
    println!("{}", grade(letters, words, sentences));
}

fn get_text() -> String {
    let mut prompt: String = String::new();
    print!("Text: ");
    io::stdout().flush().expect("error: failed to flush stdout");
    io::stdin()
        .read_line(&mut prompt)
        .expect("error: failed to read stdin");
    format!("{}", prompt)
}

fn count_letters(text: &String) -> u32 {
    text.chars().filter(|c| c.is_ascii_alphabetic()).count() as u32
}

fn count_words(text: &String) -> u32 {
    text.split_whitespace().count().try_into().unwrap()
}

fn count_sentences(text: &str) -> u32 {
    let mut counter: u32 = 1;
    for c in text.chars() {
        match c {
            '.' => counter += 1,
            '!' => counter += 1,
            '?' => counter += 1,
            _ => continue,
        }
    }
    return counter;
}

fn grade(letters: u32, words: u32, sentences: u32) -> String {
    let l: f32 = letters as f32 / words as f32 * 100.;
    let s: f32 = sentences as f32 / words as f32 * 100.;
    let i: f32 = 0.0588 * l - 0.296 * s - 15.8;
    let i: u32 = i.round() as u32;

    if i <= 1 {
        "Before Grade 1".to_string()
    } else if i >= 16 {
        "Grade 16+".to_string()
    } else {
        format!("Grade {}", i)
    }
}
