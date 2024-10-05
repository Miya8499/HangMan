mod assets;

use std::io;
use rand::Rng;
use crate::assets::HANGMAN_ASSETS;

static WORD_LIST: [&str; 4] = ["Apple", "Banana", "Grape", "Peach"];

fn main() {
    run_hangman(get_random_word(WORD_LIST));
}

fn run_hangman(word: &str) {
    let mut tries: i8 = 0;
    let mut remaining_attempts: i8 = 10;

    let mut current_guess: String = word.chars()
        .map(|char| if char == ' ' { char } else { '_' })
        .collect::<String>();

    loop {
        clear_console();

        println!("{}", HANGMAN_ASSETS.iter().nth(10 - remaining_attempts as usize).unwrap());

        if remaining_attempts <= 0 {
            println!("\nYou lost!\n");
            break;
        }

        if current_guess == word {
            println!("\nYou won with {tries} attempts!\n");
            return;
        }

        let mut input: String = String::new();

        println!("Remaining attempts: {remaining_attempts}\n", );
        println!("\n{current_guess}\n");
        println!("Insert a letter or a word: ");

        io::stdin().read_line(&mut input).expect("Read line failed.");
        input = input.trim().to_string().to_lowercase();

        tries += 1;

        if input.is_empty() {
            continue;
        }

        if (input.len()) == 1 && word.to_lowercase().contains(input.as_str()) {
            for char_index in get_char_indexes(input.chars().nth(0).unwrap(), word.to_lowercase().as_str()) {
                current_guess = replace_char_at_index(char_index, word.chars().collect::<Vec<char>>()[char_index], current_guess);
            }

            continue;
        }

        if input.len() == word.len() && input.to_lowercase() == word.to_lowercase() {
            current_guess = word.to_string();
            continue;
        }

        remaining_attempts -= 1;
    }
}

fn get_random_word(list: [&str; 4]) -> &str {
    let mut rng = rand::thread_rng();
    let n: usize = rng.gen_range(0..(list.len() - 1));

    list[n]
}

fn get_char_indexes(char_to_search: char, word: &str) -> Vec<usize> {
    word.chars()
        .enumerate()
        .filter(|(_, c)| *c == char_to_search)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>()
}

fn replace_char_at_index(index: usize, replacement_char: char, word: String) -> String {
    let mut word_chars: Vec<char> = word.chars().collect();

    word_chars[index] = replacement_char;
    word_chars.into_iter().collect()
}

fn clear_console() {
    print!("{esc}c", esc = 27 as char);
}