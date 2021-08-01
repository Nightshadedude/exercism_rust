use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum AlphaError {
   FailedCalc,
   DuplicateNumber,
   LeadingZero,
   NoResultsFound,
}

//pub type AlphaResult<'a> = Result<(&'a Vec<char>, usize, &'a HashMap<char, u8>), AlphaError>;

#[derive(Debug)]
struct AlphaMetric {
    input: String,
    letters_to_assign: HashSet<char>,
    left_eq_columns: Vec<Vec<char>>,
    right_eq_columns: Vec<char>,
}

impl AlphaMetric {
    pub fn new(input: &str) -> Self {
        let input = String::from(input);
        let letters_to_assign = input.chars().filter(|c| c.is_ascii_uppercase()).collect::<HashSet<char>>();
        let split = input.split(" == ").collect::<Vec<_>>();
        let left = split[0].split(" + ").collect::<Vec<_>>();
        let left_eq_columns = Self::columnize(left);
        let right_eq_columns = split[1].chars().collect::<Vec<char>>();
        let letters = &letters_to_assign.iter().map(|&ch| ch).collect::<Vec<char>>();

        AlphaMetric {
            input,
            letters_to_assign,
            left_eq_columns,
            right_eq_columns,
        }
    }

    pub fn get_all_combos(&self) -> Option<HashMap<char, u8>> {
        let mut hash: HashMap<char, u8> = HashMap::new();
        let chars = self.letters_to_assign.iter().map(|&c| c).collect::<Vec<char>>();
        match self.build(&chars, 0, &mut hash) {
            Ok((_,_,h)) => {
                let ret = h.clone();
                Some(ret)
            },
            Err(_) => None,
        }
    }

    pub fn build<'a>(&self, chars: &'a Vec<char>, depth: usize, hash: &'a mut HashMap<char, u8>) -> Result<(&'a Vec<char>, usize, &'a mut HashMap<char, u8>), AlphaError> {    
        if !check_duplicate(hash) { return Err(AlphaError::DuplicateNumber); }
        match depth {
            _ if depth >= chars.len() => {
                if self.zero_check(hash) { return Err(AlphaError::LeadingZero); }
               if !self.calc(hash) { return Err(AlphaError::FailedCalc); }
               Ok((chars, depth, hash))
            },
            _ =>{
                for i in 0..=9 {
                    hash.insert(chars[depth], i);
                    let mut inner_hash = hash.clone();
                    match self.build(chars, depth+1, &mut inner_hash) {
                        Ok((_, d, _)) => {
                            return Ok((chars, d, hash));
                        },
                        _ => {},
                    }
                }
                Err(AlphaError::NoResultsFound)
            },
        }
    }

    pub fn columnize(vec: Vec<&str>) -> Vec<Vec<char>> {
        let mut ret_vec = vec![];
        let mut vec = vec.iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        while vec.len() > 0 {
            let mut char_vec = vec![];
            let mut temp_vec = vec![];
            for _ in 0..vec.len() {
                let mut current = vec.pop().unwrap();
                let ch = current.pop().unwrap();
                char_vec.push(ch);
                if current.len() > 0 { temp_vec.push(current) }
            }
            char_vec.reverse();
            ret_vec.push(char_vec);
            temp_vec.reverse();
            vec = temp_vec;
        }
        ret_vec.reverse();
        ret_vec
    }
    pub fn zero_check(&self, hash: &HashMap<char, u8>) -> bool {
        for c in self.left_eq_columns[0].iter(){
            if hash.get(c).unwrap() == &0 { return true; }
        }
        if hash.get(&self.right_eq_columns[0]).unwrap() == &0 { return true; }
        false
    }

    pub fn calc(&self, hash: &HashMap<char, u8>) -> bool {
        let mut left = self.left_eq_columns.clone();
        let mut right = self.right_eq_columns.clone();
        let mut carry = 0;
        while left.len() > 0 && right.len() > 0 {
            let l = left.pop().unwrap();
            let r = right.pop().unwrap();
            let l_sum = l.iter().map(|i| hash.get(i).unwrap()).sum::<u8>() + carry;
            let r_sum = hash.get(&r).unwrap();
            carry = (l_sum - (l_sum % 10)) / 10;
            if l_sum - carry - r_sum != 0 { return false; }
        }
        match right.len() {
            1 => {
                let r = right.pop().unwrap();
                let r_sum = hash.get(&r).unwrap();
                if r_sum == &carry { true }
                else { false }
            },
            0 => {
                if carry == 0 { true }
                else { false }
            },
            _ => {
                let mut temp = 0;
                right.reverse();
                for (e, i) in right.iter().enumerate() {
                    let i = hash.get(&i).unwrap();
                    temp += u8::pow(e as u8, 10) * i;
                }
                if temp == carry { true }
                else { false }
            },
        }
    }
}

pub fn char_to_u8_vec(hash: &HashMap<char, u8>, char_vec: &Vec<char>) -> Vec<u8> {
    let u8_vec = char_vec.iter().map(|c| *hash.get(c).unwrap()).collect::<Vec<u8>>();
    u8_vec
}

pub fn check_duplicate(hash: &HashMap<char, u8>) -> bool {
    if hash.len() == 0 { return true; }
    let mut temp_hash = HashMap::new();
    for (k,v) in hash.iter() {
        temp_hash.insert(v, k);
    }
    temp_hash.len() == hash.len()
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let test = AlphaMetric::new(input);
    test.get_all_combos()
}


