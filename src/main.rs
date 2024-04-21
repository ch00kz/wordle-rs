use std::collections::HashMap;
use std::io;
use std::iter::zip;

pub mod words;
pub mod types;

use crate::words::{get_random_word, validate_word, WordValidation};
use crate::types::{Guess, display_guesses, DisplayMode};

fn main() {
    let solution = get_random_word();
    let mut history = Vec::<Vec<Guess>>::new();
    let mut round = 1;
    println!("Start guessing...");
    while round <= 6 {
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                let guess_str = user_input.trim().to_lowercase();
                match validate_word(&guess_str) {
                    WordValidation::NotFiveChars => println!("✋ Try again. Word must be five characters."),
                    WordValidation::NotInDict => println!("📖 Try again. Word is not in dictionary."),
                    WordValidation::Valid(word) => {
                        let guesses = make_guesses(&word, &solution);
                        history.push(guesses.clone());
                        println!("{}", display_guesses(&guesses, DisplayMode::Letters));
                        if guesses.iter().all(|guess| guess.is_correct()) {
                            break
                        } else {
                            round += 1;
                        }
                    }
                }
            }
            Err(err) => println!("{:?}", err),
        }
    }

    // Print History
    println!("\nWordle-RS {}/6", if round <= 6 { round.to_string() } else { String::from("X") });
    for guesses in history {
        println!("{}", display_guesses(&guesses, DisplayMode::Emojis));
    }
}

fn make_guesses(guess: &str, solution: &str) -> Vec<Guess> {
    let mut looking_for = HashMap::<char, u32>::new();
    let mut guesses = Vec::<Guess>::new();
    // First Pass: Mark correct guesses and wrong guesses. Also make note of what we're looking for.
    for (guess_char, solution_char) in zip(
        guess.to_lowercase().chars(),
        solution.to_lowercase().chars(),
    ) {
        if guess_char == solution_char {
            guesses.push(Guess::Correct(guess_char))
        } else {
            *looking_for.entry(solution_char).or_insert(0) += 1;
            guesses.push(Guess::Wrong(guess_char))
        }
    }

    // Second Pass: Check if we're looking for any of the wrong guesses, and mark as InTheWord
    for guess in guesses.iter_mut().filter(|g| g.is_wrong()) {
        let letter = guess.get_char();
        if let Some(amount_looking_for) = looking_for.get_mut(&letter) {
            if *amount_looking_for > 0 {
                *guess = Guess::InTheWord(letter);
                *amount_looking_for -= 1;
            }
        }
    }

    guesses
}