use std::collections::HashMap;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound <= 1 { return vec![]; }

    let mut map = HashMap::new();

    for i in 2..=upper_bound {
        if map.get(&i).is_some() { continue; }

        let mut num = i + i;
        while num <= upper_bound {
            map.entry(num).or_insert(1);
            num += i;
        }
    }

    (2..=upper_bound).filter(|i| map.get(i).is_none()).collect()
}
