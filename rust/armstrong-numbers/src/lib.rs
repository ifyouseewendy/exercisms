pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 { return true; }

    let num_str = num.to_string();
    let len = num_str.len() as u32;

    num == num_str.chars().map(|c| c.to_digit(10).unwrap().pow(len)).sum()
}
