#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let sum: u64 = resolve(num).iter().sum();

    match sum.cmp(&num) {
        std::cmp::Ordering::Equal => Some(Classification::Perfect),
        std::cmp::Ordering::Greater => Some(Classification::Abundant),
        _ => Some(Classification::Deficient),
    }
}

fn resolve(num: u64) -> Vec<u64> {
    if num == 1 {
        return Vec::new();
    }

    let sqrt = (num as f64).sqrt() as u64;
    let mut ret = vec![];

    for i in 1..=sqrt {
        if num % i != 0 {
            continue;
        }

        ret.push(i);
        if i != 1 && !ret.contains(&(num / i)) {
            ret.push(num / i)
        }
    }

    ret
}
