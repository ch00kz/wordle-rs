use std::collections::HashMap;
use std::io;
use std::iter::zip;

pub mod words;
pub mod types;

use crate::words::{get_random_word, validate_word, WordValidation};
use crate::types::{Guess, display_guesses};

fn main() {
    let mut unsolved = true;
    let solution = get_random_word();
    println!("Start guessing...");
    while unsolved {
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                let cleaned_guess_str = user_input.trim().to_lowercase();
                match validate_word(&cleaned_guess_str) {
                    WordValidation::NotFiveChars => println!("Word must be five characters."),
                    WordValidation::NotInDict => println!("Word is not in dictionary."),
                    WordValidation::Valid(guess) => {
                        let guesses = check_guess(&guess, &solution);
                        println!("{}", display_guesses(&guesses));
                        if guesses.iter().all(|guess| guess.is_correct()) {
                            unsolved = false;
                        }
                    }
                }
            }
            Err(err) => println!("{:?}", err),
        }
    }
    println!("\n{}", String::from("🎉🎉🎉 YOU DID IT! 🎉🎉🎉"))
}

fn check_guess(guess: &str, solution: &str) -> Vec<Guess> {
    let mut looking_for = HashMap::<char, u32>::new();
    let mut score = Vec::<Guess>::new();
    // First Pass: Mark correct guesses and wrong guesses. Also make note of what we're looking for.
    for (guess_char, solution_char) in zip(
        guess.to_lowercase().chars(),
        solution.to_lowercase().chars(),
    ) {
        // first pass, marking correct characters
        if guess_char == solution_char {
            score.push(Guess::Correct(guess_char))
        } else {
            *looking_for.entry(solution_char).or_insert(0) += 1;
            score.push(Guess::Wrong(guess_char))
        }
    }

    // Second Pass: If we're looking for a character, mark it as InTheWord
    for guess in score.iter_mut() {
        if guess.is_wrong() {
            let guess_char = guess.get_char();
            if let Some(looking_for_amount) = looking_for.get_mut(&guess_char) {
                if *looking_for_amount > 0 {
                    *guess = Guess::InTheWord(guess_char);
                    *looking_for_amount -= 1;
                }
            }
        }
    }

    score
}