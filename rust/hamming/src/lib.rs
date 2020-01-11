/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
const DNA: [char; 4] = ['C', 'A', 'G', 'T'];
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    if [s1, s2]
        .iter()
        .any(|&s| s.chars().any(|c| !DNA.contains(&c)))
    {
        return None;
    }

    Some(s1.chars().zip(s2.chars()).filter(|&(a, b)| a != b).count())
}
