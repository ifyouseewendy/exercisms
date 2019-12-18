use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !valid(nucleotide) {
        return Err(nucleotide);
    }

    let mut counter = 0;
    for c in dna.chars() {
        if !valid(c) {
            return Err(c);
        } else if c == nucleotide {
            counter += 1;
        }
    }

    Ok(counter)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut h = HashMap::new();
    let set = "ACGT";

    for c in set.chars() {
        match count(c, dna) {
            Err(cc) => return Err(cc),
            Ok(counter) => { h.insert(c, counter); }
        }
    }

    Ok(h)
}

fn valid(nucleotide: char) -> bool {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false
    }
}
