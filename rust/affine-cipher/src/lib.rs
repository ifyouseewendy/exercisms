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
    if [2, 13, 26].iter().any(|i| a % i == 0) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    Ok(plaintext
        .chars()
        .filter(|&c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase() as u8)
        .map(|byte| match byte {
            b'a'..=b'z' => b'a' + ((a * (byte - b'a') as i32 + b) % M) as u8,
            _ => byte,
        })
        .collect::<Vec<u8>>()
        .chunks(5)
        .map(|c| String::from_utf8(c.to_owned()).unwrap())
        .collect::<Vec<String>>()
        .join(" "))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
/// D(y) = a^-1(y - b) mod m
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if [2, 13, 26].iter().any(|i| a % i == 0) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let mmi = find_mmi(a);

    Ok(String::from_utf8(
        ciphertext
            .chars()
            .filter(|&c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase() as u8)
            .map(|byte| match byte {
                b'a'..=b'z' => b'a' + ((mmi * ((byte - b'a') as i32 - b)).rem_euclid(M)) as u8,
                _ => byte,
            })
            .collect::<Vec<u8>>(),
    )
    .unwrap())
}

fn find_mmi(a: i32) -> i32 {
    (1..).find(|&i| i * a % M as i32 == 1).unwrap()
}
