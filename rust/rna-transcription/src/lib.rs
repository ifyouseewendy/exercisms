#[derive(Debug, PartialEq)]
pub struct DNA(String);

#[derive(Debug, PartialEq)]
pub struct RNA(String);

const VALID_DNA: [char; 4] = ['A', 'T', 'C', 'G'];
const VALID_RNA: [char; 4] = ['A', 'U', 'C', 'G'];

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        for (i, c) in dna.chars().enumerate() {
            if !VALID_DNA.contains(&c) { return Err(i) }
        }
        Ok(Self(dna.to_string()))
    }

    pub fn into_rna(self) -> RNA {
        RNA(
            self.0.chars().map(|c| {
                match c {
                    'A' => Some('U'),
                    'T' => Some('A'),
                    'C' => Some('G'),
                    'G' => Some('C'),
                    _ => None,
                }
            }).collect::<Option<String>>().unwrap()
        )
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        for (i, c) in rna.chars().enumerate() {
            if !VALID_RNA.contains(&c) { return Err(i) }
        }
        Ok(Self(rna.to_string()))
    }
}
