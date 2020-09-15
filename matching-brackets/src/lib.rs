pub fn brackets_are_balanced(string: &str) -> bool {
    let match_bracket: Vec<(String, String)> = vec![
        ("[".to_string(), "]".to_string()),
        ("{".to_string(), "}".to_string()),
        ("(".to_string(), ")".to_string())
    ];
    
    let mut bracket_str = vec![];
    //strip non-brackets
    for c in string.chars() {
        for mb in match_bracket.iter() {
            if c.to_string() == mb.0 ||
                c.to_string() == mb.1 {
                bracket_str.push(c);
            }
        }
    }

    let mut matched = true;
    let mut remaining = bracket_str.len();

    while matched && bracket_str.len() > 0 {
        for mb in match_bracket.iter() {
            println!("checking match bracket: {:#?}", mb);
            println!("checking brackets: {:#?}", &bracket_str);
            match bracket_str.iter().position(|bs| bs.to_string() == mb.1) {
                Some(x) if x > 0 => {
                    if bracket_str[x-1].to_string() == mb.0 {
                        bracket_str.remove(x);
                        bracket_str.remove(x-1);
                    }
                },
                _ => {},
            }
        }
        if remaining == bracket_str.len() {
            matched = false;
        } else {
            remaining = bracket_str.len();
            matched = true;
        }
    }
    matched
}
