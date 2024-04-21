use colored::*;
use std::fmt;

pub enum Guess {
    Correct(char),
    InTheWord(char),
    Wrong(char),
}

impl Guess {
    pub fn get_char(&self) -> char {
        match *self {
            Guess::Correct(c) => c,
            Guess::InTheWord(c) => c,
            Guess::Wrong(c) => c,
        }
    }
    pub fn is_correct(&self) -> bool {
        match *self {
            Guess::Correct(_) => true,
            _ => false,
        }
    }
    pub fn is_wrong(&self) -> bool {
        match *self {
            Guess::Wrong(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Guess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let guess = match self {
            Guess::Correct(c) => c.to_string().to_uppercase().green(),
            Guess::InTheWord(c) => c.to_string().to_uppercase().yellow(),
            Guess::Wrong(c) => c.to_string().to_uppercase().red(),
        };
        write!(f, " {} ", guess)
    }
}

pub fn display_guesses(guesses: &Vec<Guess>) -> String {
    guesses.iter().map(|g| g.to_string()).collect()
}
