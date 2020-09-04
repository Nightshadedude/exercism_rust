pub fn brackets_are_balanced(string: &str) -> bool {
    let match_bracket: Vec<String> = vec!["[","]","{","}","(",")"];
    let mut bracket_vec: Vec<String> = string.chars()
        .iter()
        .map(|c| if c in match_bracket.iter(){c})
        .collect::<Vec<String>>();
    while bracket_vec.len() > 0{
        
    }
}
