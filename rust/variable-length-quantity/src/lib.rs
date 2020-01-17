#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|&v| to_byte(v)).collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    if bytes.is_empty() {
        return Ok(Vec::new());
    } else if !is_end_byte(*bytes.last().unwrap()) {
        return Err(Error::IncompleteNumber);
    }

    let mut grouped_bytes: Vec<Vec<u8>> = vec![];
    let mut iter = bytes.iter();

    loop {
        let mut v: Vec<u8> = vec![];
        while let Some(&bt) = iter.next() {
            v.push(bt);
            if is_end_byte(bt) {
                break;
            }
        }

        if v.is_empty() {
            break;
        }
        grouped_bytes.push(v);
    }

    println!("grouped_bytes: {:#?}", grouped_bytes);

    grouped_bytes
        .into_iter()
        .map(|gb| {
            let bs = gb
                .iter()
                .flat_map(|&u| {
                    let mut b = format!("{:b}", u).bytes().rev().collect::<Vec<u8>>();
                    while b.len() < 8 {
                        b.push(b'0');
                    }

                    b.iter()
                        .rev()
                        .skip(1)
                        .map(|&v| if v == b'1' { 1 } else { 0 })
                        .collect::<Vec<u8>>()
                })
                .rev()
                .collect::<Vec<u8>>();

            println!("bs: {:?}", bs);

            let sum: u64 = bs.iter().enumerate().fold(0, |acc, (i, &ele)| {
                acc + ((ele as u64) * 2_u64.pow(i as u32))
            });
            if sum > std::u32::MAX as u64 {
                Err(Error::Overflow)
            } else {
                Ok(sum as u32)
            }
        })
        .collect()
}

fn is_end_byte(v: u8) -> bool {
    v & 0b_1000_0000 == 0
}

fn to_byte(v: u32) -> Vec<u8> {
    let mut bytes = format!("{:b}", v).bytes().rev().collect::<Vec<u8>>();
    while bytes.len() % 7 != 0 {
        bytes.push(b'0');
    }
    bytes = bytes.iter().rev().copied().collect::<Vec<u8>>();
    println!("{:?}", String::from_utf8(bytes.clone()).unwrap());

    let len = bytes.len() / 7;
    bytes
        .chunks(7)
        .enumerate()
        .map(|(i, chunk)| {
            std::iter::once(if i == len - 1 { b'0' } else { b'1' }).chain(chunk.to_owned())
        })
        .map(|bs| {
            u8::from_str_radix(&String::from_utf8(bs.collect::<Vec<u8>>()).unwrap(), 2).unwrap()
        })
        .collect()
}
