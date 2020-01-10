pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut v = factors.iter().take_while(|&&v| v != 0).flat_map(|&v| (v..limit).step_by(v as usize)).collect::<Vec<u32>>();

    v.sort();
    v.dedup();

    v.iter().sum()
}
