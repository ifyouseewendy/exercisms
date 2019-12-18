use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    validate(nucleotide).and_then(|nc|
        dna.chars()
            .map(validate)
            .collect::<Result<Vec<_>, char>>()
            .map(|vc| vc.iter().filter(|&&c| c == nc).count() )
    )
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    "ACGT".chars()
        .map(|c| count(c, dna).map(|num| (c, num)))
        .collect()
}

fn validate(nucleotide: char) -> Result<char, char> {
    if "ACGT".chars().any(|c| c == nucleotide) {
        Ok(nucleotide)
    } else {
        Err(nucleotide)
    }
}
