pub fn factors(n: u64) -> Vec<u64> {
    let mut v = vec![];
    let mut num = n;
    let mut candidates = 2..;

    while num > 1 {
        let x = candidates.next().unwrap();

        while num % x == 0 {
            v.push(x);
            num /= x;
        }
    }
    v
}
