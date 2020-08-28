pub fn factors(n: u64) -> Vec<u64> {
    let mut factor_vec: Vec<u64> = Vec::new();
    let mut curr_val = n;
    let mut curr_factor = 2;
    while curr_val > 1 {
        if curr_val % curr_factor == 0 {
            curr_val = curr_val / curr_factor;
            factor_vec.push(curr_factor);
        } else {
            curr_factor += 1;
        }
    }
    factor_vec
}
