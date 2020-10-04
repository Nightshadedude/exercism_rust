use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    println!("nuc: {}, dna: {}", nucleotide, dna);
    let mut count = 0usize;
    match nucleotide {
        'A' => (),
        'T' => (),
        'G' => (),
        'C' => (),
        _ => return Err(nucleotide),
    }
    count = dna.matches(nucleotide).count();
    match nucleotide_counts(dna) {
        Ok(_) => Ok(count),
        Err(e) => Err(e),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut nuc_counts: HashMap<char, usize> = HashMap::new();
    for ch in dna.chars().into_iter() {
        nuc_counts.insert(ch, dna.matches(ch).count());
    }
    for hash in nuc_counts.iter() {
        if hash.0 == &'A' ||
            hash.0 == &'T' ||
            hash.0 == &'G' ||
            hash.0 == &'C' 
            {}
        else{
            return Err(*hash.0);
        }
    }
    Ok(nuc_counts)
}
