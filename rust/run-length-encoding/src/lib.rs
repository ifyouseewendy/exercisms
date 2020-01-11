pub fn encode(source: &str) -> String {
    let mut chars = vec![];
    let mut count = 0;
    let mut cur = '_';

    let mut iter = source.chars().peekable();

    while let Some(a) = iter.next() {
        if count == 0 {
            cur = a;
            count += 1;
        }

        match iter.peek() {
            Some(&b) if a == b => count += 1,
            _ => {
                if count > 1 {
                    chars.extend(count.to_string().chars().collect::<Vec<char>>());
                }
                chars.push(cur);
                count = 0;
            }
        }
    }

    chars.into_iter().collect()
}

pub fn decode(source: &str) -> String {
    let chars = source.chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut ret = vec![];
    let mut count = 1;

    while i < chars.len() {
        if chars[i].is_ascii_digit() {
            let mut j = i + 1;
            while chars[j].is_ascii_digit() {
                j += 1;
            }

            count = chars[i..j]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            i = j;
        } else {
            ret.extend([chars[i]].repeat(count));
            count = 1;
            i += 1;
        }
    }

    ret.into_iter().collect()
}
