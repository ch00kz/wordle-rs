use colored::*;
use std::fmt;

pub enum Guess {
    Correct(char),
    InWord(char),
    Wrong(char),
}

impl Guess {
    pub fn get_char(&self) -> char {
        match *self {
            Guess::Correct(c) => c,
            Guess::InWord(c) => c,
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

pub type Score = Vec<Guess>;

impl fmt::Display for Guess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let guess = match self {
            Guess::Correct(c) => c.to_string().to_uppercase().green(),
            Guess::InWord(c) => c.to_string().to_uppercase().yellow(),
            Guess::Wrong(c) => c.to_string().to_uppercase().red(),
        };
        write!(f, "{}", guess)
    }
}
