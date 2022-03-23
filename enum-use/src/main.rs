use std::fmt;

pub enum Color<T> {
    Red(T),
    Blue(T),
    Green(T),
}

impl<T> fmt::Display for Color<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Red(text) => write!(f, "\x1B[1;31m{}\x1B[0m", text),
            Self::Blue(text) => write!(f, "\x1B[1;34m{}\x1B[0m", text),
            Self::Green(text) => write!(f, "\x1B[1;32m{}\x1B[0m", text),
        }
    }
}

use Color::*;

fn main() {
    println!("this is an {} but in red", Red("hello, world"));
    println!("this is an {} but in blue", Blue("hello, world"));
    println!("this is an {} but in green", Green("hello, world"));
}