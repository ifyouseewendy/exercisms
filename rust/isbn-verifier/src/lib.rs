/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let chars = isbn
        .chars()
        .enumerate()
        .skip_while(|&(_, c)| c == '-')
        .filter(|&(i, c)| {
            if i != isbn.len() - 1 {
                c.is_ascii_digit()
            } else {
                c.is_ascii_digit() || c == 'X'
            }
        })
        .map(|(_, c)| c)
        .collect::<Vec<char>>();

    if chars.len() != 10 {
        return false;
    }

    let sum = chars.iter().enumerate().fold(0, |acc, (i, c)| {
        acc + c.to_digit(10).unwrap_or(10) as usize * (10 - i)
    });

    sum % 11 == 0
}
