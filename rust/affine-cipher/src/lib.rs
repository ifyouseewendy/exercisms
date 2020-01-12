/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const M: i32 = 26;

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if [2, 13].iter().any(|i| a % i == 0) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    Ok(plaintext
        .chars()
        .filter_map(|c| match c {
            c if c.is_ascii_alphabetic() => {
                let x = (c.to_ascii_lowercase() as u8 - b'a') as i32;
                Some(b'a' + ((a * x + b) % M) as u8)
            }
            c if c.is_numeric() => Some(c as u8),
            _ => None,
        })
        .collect::<Vec<u8>>()
        .chunks(5)
        .map(|chunk| String::from_utf8(chunk.to_owned()).unwrap())
        .collect::<Vec<String>>()
        .join(" "))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
/// D(y) = a^-1(y - b) mod m
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if [2, 13].iter().any(|i| a % i == 0) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let mmi = (1..).find(|&i| i * a % M as i32 == 1).unwrap();

    Ok(String::from_utf8(
        ciphertext
            .chars()
            .filter_map(|c| match c {
                c if c.is_ascii_alphabetic() => {
                    let y = (c.to_ascii_lowercase() as u8 - b'a') as i32;
                    Some(b'a' + ((mmi * (y - b)).rem_euclid(M)) as u8)
                }
                c if c.is_numeric() => Some(c as u8),
                _ => None,
            })
            .collect::<Vec<u8>>(),
    )
    .unwrap())
}
