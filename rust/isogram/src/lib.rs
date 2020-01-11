use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let chars = candidate
        .to_lowercase()
        .chars()
        .filter(|&c| c.is_ascii_alphabetic())
        .collect::<Vec<char>>();

    chars.len() == chars.iter().copied().collect::<HashSet<char>>().len()
}
