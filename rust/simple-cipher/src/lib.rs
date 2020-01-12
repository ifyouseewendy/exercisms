use rand::{thread_rng, Rng};
pub fn encode(key: &str, s: &str) -> Option<String> {
    println!("{:?}, {:?}", key, s);
    if key.is_empty() {
        return None;
    }
    if [key, s]
        .iter()
        .any(|&s| s.chars().any(|c| !c.is_ascii_lowercase()))
    {
        return None;
    }

    String::from_utf8(
        s.bytes()
            .enumerate()
            .map(|(i, byte)| {
                key.as_bytes()
                    .get(i)
                    .map(|v| forward(byte, v - b'a'))
                    .unwrap_or(byte)
            })
            .collect::<Vec<u8>>(),
    )
    .ok()
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    println!("{:?}, {:?}", key, s);
    if key.is_empty() {
        return None;
    }
    if [key, s]
        .iter()
        .any(|&s| s.chars().any(|c| !c.is_ascii_lowercase()))
    {
        return None;
    }

    String::from_utf8(
        s.bytes()
            .enumerate()
            .map(|(i, byte)| {
                key.as_bytes()
                    .get(i)
                    .map(|v| backward(byte, v - b'a'))
                    .unwrap_or(byte)
            })
            .collect::<Vec<u8>>(),
    )
    .ok()
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = thread_rng();
    let key: String = String::from_utf8(
        (0..100)
            .map(|_| rng.gen_range(0, 26) + b'a')
            .collect::<Vec<u8>>(),
    )
    .unwrap();
    (key.clone(), encode(&key, s).unwrap())
}

const LENGTH: u8 = 26;
fn forward(byte: u8, distance: u8) -> u8 {
    b'a' + (byte - b'a' + distance) % LENGTH
}

fn backward(byte: u8, distance: u8) -> u8 {
    let step = if byte - b'a' >= distance {
        byte - b'a' - distance
    } else {
        byte - b'a' + LENGTH - distance
    };

    b'a' + step
}
