#[derive(Clone)]
struct WordCheck{
    word: String,
    handle_punc: Option<String>,
    handle_camel: Option<String>,
}

impl WordCheck {
    fn new(s: &str) -> Self {
        WordCheck {
            word: String::from(s),
            handle_punc: handle_punc(s),
            handle_camel: handle_camel(s),
        }
    }

    fn abb(self) -> String {
        if self.handle_punc.is_some() {self.handle_punc.unwrap()}
        else if self.handle_camel.is_some() {self.handle_camel.unwrap()}
        else {self.word.to_uppercase().chars().next().unwrap().to_string()}
    }
}
fn handle_punc(s: &str) -> Option<String> {
    let len = s.len();
    for i in 0..len{
        match s.chars().nth(i).unwrap().is_ascii_punctuation() {
            true if len == 1 => {
                return Some("".to_string())
            },
            true if i == 0 => {
                match s.chars().nth(i+1) {
                    Some(ch) => return Some(ch.to_uppercase().to_string()),
                    None => return None,
                }
            }
            true if i == len-1 => {
                return None
            },
            true => {
                if s.chars().nth(i).unwrap() == '\'' {return None}
                match s.chars().nth(i+1) {
                    Some(ch) => {
                        let mut abb = String::from("");
                        abb.push(s.chars().nth(0).unwrap());
                        abb.push(ch);
                        abb = abb.to_uppercase();
                        return Some(abb)
                    },
                    None => return None,
                }
            }
            false => (),
        }
    }
    return None;
}

fn handle_camel(s: &str) -> Option<String> {
    if s == s.to_uppercase() {return None;}
    let max_count = s.len();
    let mut ret = String::from("");
    let count: usize = s.chars()
        .map(|c| if c.is_uppercase() {ret.push(c); 1usize} else {0usize}).sum();
    if count == max_count {return None;}
    else if count == 0 {return None;}
    else if count == 1 {return None;}
    else {return Some(ret);}
}

pub fn abbreviate(phrase: &str) -> String {
    phrase.split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| WordCheck::new(s))
        .collect::<Vec<WordCheck>>()
        .iter()
        .map(|wc| wc.clone().abb())
        .collect::<Vec<String>>()
        .join("")
    }
