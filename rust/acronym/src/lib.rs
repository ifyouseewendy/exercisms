pub fn abbreviate(phrase: &str) -> String {
    // The chaining solution has such bad performance
    //
    // phrase.chars().filter(|&c| c.is_ascii_alphabetic() || c.is_ascii_whitespace()).collect::<String>()
    //     .split_ascii_whitespace()
    //     .map(|s| s.chars().next().map(|c| c.to_ascii_uppercase()))
    //     .collect::<Option<String>>()
    //     .unwrap_or_else(|| "".to_string())

    let mut v = vec![];
    let mut prev = '\0';

    for (i, c) in phrase.chars().enumerate() {
        if (i == 0)
            || (c.is_ascii_uppercase() && !prev.is_ascii_uppercase())
            || (c.is_ascii_alphabetic() && (prev == ' ' || prev == '-'))
        {
            v.push(c.to_ascii_uppercase());
        }
        prev = c;
    }

    v.into_iter().collect()
}
