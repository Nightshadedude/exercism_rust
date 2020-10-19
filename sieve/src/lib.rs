pub fn primes_up_to(n: u64) -> Vec<u64> {                                            
    let mut prime_sieve: Vec<u64> = vec![2,3];                                       
    let mut check_prime = 3;                                                         
    while prime_sieve.last().unwrap() <= &n && check_prime <= n{                     
        check_prime += 2;                                                            
        let mut factor_count = 0;                                                    
        for i in 0..prime_sieve.len(){                                               
            if check_prime % prime_sieve[i] == 0 {                                   
                factor_count += 1;                                                   
                break;                                                               
            }                                                                        
        }                                                                            
        if factor_count == 0 && check_prime <= n {                                   
            prime_sieve.push(check_prime);                                           
        }                                                                            
    }                                                                                
    match n {                                                                        
        _ if n <= 1 => vec![],                                                       
        2 => vec![2],                                                                
        _ => prime_sieve,                                                            
    }                                                                                
} 
