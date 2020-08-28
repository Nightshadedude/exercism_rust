pub fn verse(n: u32) -> String {
    let fill: (String, String, String, String, String, String, String) = match n {
        0 => ("No more".to_string(),
            "s".to_string(),
            "no more".to_string(), 
            "s".to_string(),
            "Go to the store and buy some more, ".to_string(),
            "99".to_string(),
            "s".to_string()),
        1 => ("1".to_string(),
            "".to_string(),
            "1".to_string(),
            "".to_string(),
            "Take it down and pass it around, ".to_string(),
            "no more".to_string(),
            "s".to_string()),
        2 => (n.to_string(),
              "s".to_string(),
              n.to_string(),
              "s".to_string(),
              "Take one down and pass it around, ".to_string(),
              (n-1).to_string(),
              "".to_string()),
        _ => (n.to_string(),
              "s".to_string(),
              n.to_string(),
              "s".to_string(),
              "Take one down and pass it around, ".to_string(),
              (n-1).to_string(),
              "s".to_string()),
    };

    format!("{} bottle{} of beer on the wall, {} bottle{} of beer.\n\
            {}{} bottle{} of beer on the wall.\n",
            fill.0,
            fill.1,
            fill.2,
            fill.3,
            fill.4,
            fill.5,
            fill.6)

}

pub fn sing(start: u32, end: u32) -> String {
    if end < start {
        (end..=start).rev().into_iter().map(|n| verse(n)).collect::<Vec<String>>().join("\n")
    }
    else {
        (start..=end).rev().into_iter().map(|n| verse(n)).collect::<Vec<String>>().join("\n")
    }
}
