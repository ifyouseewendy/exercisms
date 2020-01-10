#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    // implement your palindrome type here
    factors: Vec<(u64, u64)>,
    value: u64,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {
            factors: vec!((a, b)),
            value: a*b,
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factors.push((a, b));
    }
}

fn find_factors(num: u64, min: u64, max: u64) -> Palindrome {
    let mut factors = vec!();

    for i in min..=max {
        if num % i == 0 && num/i <= max {
            factors.push((i, num/i))
        }
        if i*i >= num { break }
    }

    println!("factors: {:?}", factors);
    if factors.is_empty() { panic!("factors is empty for {}, {}, {}", num, min, max) }

    let (a, b) = factors[0];
    let mut p = Palindrome::new(a, b);

    let mut i = 1;
    while i < factors.len() {
        let (a, b) = factors[i];
        p.insert(a, b);

        i += 1;
    }

    p
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max { return None; }

    let candidates = find_candidates(min, max);

    let (min_p, max_p) = find_palindrome(&candidates)?;

    println!("min_p: {}, max_p: {}", min_p, max_p);

    Some((find_factors(min_p, min, max), find_factors(max_p, min, max)))
}

fn find_candidates(min: u64, max: u64) -> Vec<u64> {
    let mut candidates = vec!();

    let numbers: Vec<u64> = (min..=max).collect();
    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            candidates.push(numbers[i]*numbers[j]);
        }
    }

    candidates.sort();
    candidates.dedup();

    candidates
}

fn find_palindrome(candidates: &[u64]) -> Option<(u64, u64)> {
    let min = candidates.iter().find(|&&x| is_palindrome(x));
    let max = candidates.iter().rev().find(|&&x| is_palindrome(x));

    if min.is_none() || max.is_none() { return None }

    Some((*min.unwrap(), *max.unwrap()))
}

fn is_palindrome(num: u64) -> bool {
    let chars: Vec<char> = num.to_string().chars().collect();

    let mut i = 0;
    let mut j = chars.len() - 1;

    while i <= j {
        if chars[i] != chars[j] { return false; }

        i += 1;
        if j > 1 { j -= 1; }
    }

    true
}
