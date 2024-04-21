use std::collections::HashMap;
use std::io;
use std::iter::zip;

pub mod words;
pub mod types;

use crate::words::{get_random_word, is_in_dictionary};
use crate::types::{Guess, Score};

fn main() {
    let mut unsolved = true;
    let solution = get_random_word();
    println!("Start guessing...");
    while unsolved {
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(_) => {
                let cleaned_guess = guess.trim().to_lowercase();
                if is_in_dictionary(&cleaned_guess) {
                    let score = get_score(&cleaned_guess, &solution);
                    if score.iter().all(|guess| guess.is_correct()) {
                        unsolved = false;
                    }
                    for guess in score {
                        print!(" {} ", guess);
                    }
                    println!("")
                } else {
                    println!("Word not in dictionary")
                }
               
            }
            Err(err) => println!("{:?}", err),
        }
    }
    println!("{}", String::from("🎉🎉🎉 YOU DID IT! 🎉🎉🎉"))
}

fn get_score(guess: &str, solution: &str) -> Score {
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
            if let Some(missing) = looking_for.get_mut(&solution_char) {
                *missing += 1;
            } else {
                looking_for.insert(solution_char, 1);
            }
            score.push(Guess::Wrong(guess_char))
        }
    }

    // Second Pass: If we're looking for a character, mark it as InWord
    for guess in score.iter_mut() {
        if guess.is_wrong() {
            let guess_char = guess.get_char();
            if let Some(looking_for_amount) = looking_for.get_mut(&guess_char) {
                if *looking_for_amount > 0 {
                    *guess = Guess::InWord(guess_char);
                    *looking_for_amount -= 1;
                }
            }
        }
    }

    score
}