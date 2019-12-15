pub fn reverse(input: &str) -> String {
    let mut v = input.chars().collect::<Vec<char>>();
    v.reverse();
    v.into_iter().collect()
}
