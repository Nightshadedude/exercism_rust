pub fn raindrops(n: u32) -> String {
    let sounds: Vec<(u32, &str)> = vec![(3, "Pling"),
        (5, "Plang"),
        (7, "Plong")];
    let mut drop = String::from("");
    for (factor, sound) in sounds {
        if n % factor == 0 { drop.push_str(sound); }
    }
    match drop.len() {
        0 => format!("{}",n),
        _ => drop,
    }
}
