use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum AlphaError {
    NonUniqueDigit,
    LeadingZero,
}

struct CharContainer {
    ch: char,
    
    zero_elig: bool,
}

impl CharContainer {
    fn new() -> Self{

    }
}

struct Alphametric {
    left: Vec<Vec<char>>,
    right: Vec<char>,
    search_space: HashSet<CharContainer>,
}

impl Alphametric {
    fn new(input: &str) -> Self {

    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {

}


