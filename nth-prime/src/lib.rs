pub fn nth(n: u32) -> u32 {
    let m: usize = n as usize;
    //collect primes to allow for quicker calcs
    let mut found_prime: Vec<u32> = vec![2,3];
    let mut check_prime = 3; // start at 3 to allow increment by 2
    while found_prime.len() <= m {
        check_prime += 2; // 1/2 run time incrementing by 2
        let mut factor_count = 0;
        for i in 0..found_prime.len(){
            if check_prime % found_prime[i] == 0 {
                factor_count += 1;
                break; // break so as to not make subsequent checks
            }
        }
        if factor_count == 0 {
            found_prime.push(check_prime);
        } 
    }
    found_prime[m]
}
