use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound <= 1 { return vec![]; }

    let mut set = HashSet::new();

    for i in 2..=upper_bound {
        if set.contains(&i) { continue; }

        let mut num = i + i;
        while num <= upper_bound {
            set.insert(num);
            num += i;
        }
    }

    (2..=upper_bound).filter(|i| set.get(i).is_none()).collect()
}
