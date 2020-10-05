use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if nucleotide == 'A' ||
        nucleotide == 'T' ||
        nucleotide == 'G' ||
        nucleotide == 'C' {
        } else {
            return Err(nucleotide);
        }
 
    let res = match nucleotide_counts(dna) {
        Ok(counts) => {
            let count = counts
                .iter()
                .map(|pair| if pair.0 == &nucleotide { pair.1 } else { &0usize })
                .sum();
            Ok(count)
        },
        Err(e) => Err(e),
    };
    res
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut nuc_counts: HashMap<char, usize> = HashMap::new();
    nuc_counts.insert('A', 0);
    nuc_counts.insert('T', 0);
    nuc_counts.insert('G', 0);
    nuc_counts.insert('C', 0);
    for ch in dna.chars().into_iter() {
        if ch == 'A' ||
           ch == 'T' ||
           ch == 'G' ||
           ch == 'C' {
               nuc_counts.insert(ch, dna.matches(ch).count());
        } else {
            return Err(ch);
        }
    }

    if nuc_counts.len() == 0 {
        let mut ret = HashMap::new();
        ret.insert(' ', 0);
        return Ok(ret);
    } 
    Ok(nuc_counts)
}
