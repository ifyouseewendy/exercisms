#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref PRIMES: Vec<u64> = {
        calc_prime(MAXIMA)
    };
}
const MAXIMA: u64 = 93_819_012_551;

pub fn factors(n: u64) -> Vec<u64> {
    let primes = &PRIMES;

    let mut v = vec![];
    let mut num = n;

    while num != 1 {
        let factor = smallest_factor(num, primes).unwrap();
        v.push(factor);
        num /= factor;
    }

    v
}

fn smallest_factor(n: u64, primes: &[u64]) -> Option<u64> {
    if n < 2 { return None }

    for i in primes {
        if n % i == 0 { return Some(*i) }
    }

    Some(n)
}
fn calc_prime(n: u64) -> Vec<u64> {
    if n < 2    { return vec![] }
    if n == 2   { return vec![2] }

    let upper = n / 2;

    let mut v = vec![];
    for i in 2..=upper {
        if is_prime(i) { v.push(i); }
    }
    v
}

fn is_prime(num: u64) -> bool {
    if num == 0 || num == 1 { return false }
    if num == 2             { return true }
    if num % 2 == 0         { return false }
    if num % 3 == 0         { return false }
    if num % 5 == 0         { return false }

    let mut i = 3;
    let bound = (num as f64).sqrt() as u64;

    while i <= bound {
        if num % i == 0 { return false }
        i += 2;
    }

    true
}

