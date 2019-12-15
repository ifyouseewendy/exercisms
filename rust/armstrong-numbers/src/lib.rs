pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 { return true; }

    let str_num = num.to_string();
    let len = str_num.len() as u32;

    num == str_num.as_bytes().iter().map(|c| (((c - b'0') as u32).pow(len) as u32)).sum()
}
