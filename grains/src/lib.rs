pub fn square(s: u32) -> u64 {
    let sq:u64 = 2;
    match s {
        1..=64 => sq.pow(s-1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    (1..=64).into_iter().map(|s| square(s)).sum()
}
