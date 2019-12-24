use std::collections::{HashMap, HashSet};
use permutohedron::Heap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut parts: Vec<&str> = input.split(char::is_whitespace).collect();
    parts.retain(|&x| x != "==" && x != "+");

    let rhs = parts.pop().unwrap();
    let lhs = parts;
    println!("lhs {:?} rhs {:?}", lhs, rhs);

    let set = alpha_set(input);
    println!("Set: {:?}", set);
    if set.len() > 10 { return None }

    let mut data: Vec<u8> = (0..10).collect();
    let mut heap = Heap::new(&mut data);

    while let Some(permutation) = heap.next_permutation() {
        let map = set.iter().zip(permutation.iter()).collect::<HashMap<&char, &u8>>();
        if lhs.iter().map(|s| s.chars().next().unwrap()).any(|c| *map.get(&c).unwrap() == &0) { continue; }
        if rhs.chars().next().and_then(|c| map.get(&c)).unwrap() == &&0 { continue; }

        let lhs_sum = lhs.iter().fold(0, |accum, elem| accum + calculate(elem, &map));
        let rhs_sum = calculate(rhs, &map);

        if lhs_sum == rhs_sum { return Some(map.iter().map(|(&k, &v)| (*k, *v)).collect()) }
    }

    None
}

fn alpha_set(input: &str) -> HashSet<char> {
    input.chars().filter(|&c| char::is_uppercase(c)).collect()
}

fn calculate(part: &str, map: &HashMap<&char, &u8>) -> u64 {
    part.chars().rev().enumerate().fold(0, |accum, (i, c)|
        accum + (**map.get(&c).unwrap() as u64) * 10_u64.pow(i as u32)
    )
}
