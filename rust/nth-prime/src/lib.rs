pub fn nth(n: u32) -> u32 {
    let mut count = 0;

    for i in 2.. {
        if is_prime(i) {
            if count == n {
                return i;
            }
            count += 1;
        }
    }

    return 0;
}

fn is_prime(n: u32) -> bool {
    !(2..n).any(|x| n % x == 0)
}
