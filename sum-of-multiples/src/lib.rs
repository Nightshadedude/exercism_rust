use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: HashSet<u32> = HashSet::new();
    multiples.insert(0);
    for x in 1..limit{
        for fac in factors{
            match *fac {
                0 => {},
                _ => {match x % fac {
                        0 => {multiples.insert(x);},
                        _ => {},
                    }
                },
            }
        }
    }
    multiples.into_iter().sum()
}
