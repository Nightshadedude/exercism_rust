struct WordCheck{
    word: String,
    has_punc: Option<usize>,
    all_caps: bool,
    camel_case: bool,
}

impl WordCheck {
    fn new(s: &str) -> Self {
        WordCheck {
            word: String::from(s),
            has_punc: has_punc(s),
            all_caps: s == s.to_uppercase(),
            camel_case: camel_case(s),
        }
    }

    fn caps(&self) -> bool {
        self.all_caps
    }

    fn camel(&self) -> bool {
        self.camel_case
    }

    fn abbr(&self) -> String {
        let mut ret = String::from("");
        match self.has_punc {
            None => {
                match self.caps() {
                    true => {
                        ret.push(self.word
                                 .to_uppercase()
                                 .chars()
                                 .next()
                                 .unwrap());
                    },
                    false => {
                        match self.camel() {
                            true => {
                                ret.push_str(&self.word
                                         .chars()
                                         .map(|c| if c.is_uppercase() {c.to_string()}
                                              else {' '.to_string()})
                                         .collect::<Vec<String>>()
                                         .join("")
                                         .split_whitespace()
                                         .collect::<Vec<&str>>()
                                         .join(""));
                            },
                            false => {
                                ret.push(self.word
                                    .to_uppercase()
                                    .chars()
                                    .next()
                                    .unwrap());

                            },
                        }
                    },
                }
            },
            Some(i) => {
                let mut spl = String::from("");
                spl.push(self.word.chars().next().unwrap());
                match self.word.chars().nth(i+1) {
                    Some(ch) => spl.push(ch),
                    None => (),
                }
                ret.push_str(&spl.to_uppercase());
            },
        }
        ret
    }
}

pub fn abbreviate(phrase: &str) -> String {
    phrase.split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| WordCheck::new(s))
        .collect::<Vec<WordCheck>>()
        .iter()
        .map(|wc| wc.abbr())
        .collect::<Vec<String>>()
        .join("")
}

fn has_punc(word: &str) -> Option<usize> {
    let mut ret = None;
    for i in (0..word.len()).rev() {
        match word.chars().nth(i).unwrap().is_ascii_punctuation() {
            true => ret = Some(i),
            false => (),
        }
    }
    ret
}

fn camel_case(word: &str) -> bool {
    let max_count = word.len();
    let count = word.chars()
        .map(|c| if c.is_uppercase() {1usize} else {0usize}).sum();
    match count {
        max_count => false,
        0..=1 => false,
        _ => true,
    }
}
