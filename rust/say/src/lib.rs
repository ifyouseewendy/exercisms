pub fn encode(n: u64) -> String {
    // 1234567890
    //
    // 1,234,567,890
    //
    // for each part
    //   if len == 3
    //     x hundred + spell
    //   else
    //     spell
    //
    // join each part with, billion, million, thousand
    //   special case for 1_000_000_000

    let mut parts = vec![];
    let mut m = n;
    while m >= 1000 {
        parts.push(m % 1000);
        m /= 1000;
    }
    parts.push(m % 1000);

    let mut ret = vec![];

    if let Some(&num) = parts.get(3) {
        if num != 0 {
            ret.push(spell(num));
            ret.push("billion".to_string());
        }
    }
    if let Some(&num) = parts.get(2) {
        if num != 0 {
            ret.push(spell(num));
            ret.push("million".to_string());
        }
    }
    if let Some(&num) = parts.get(1) {
        if num != 0 {
            ret.push(spell(num));
            ret.push("thousand".to_string());
        }
    }
    if let Some(&num) = parts.get(0) {
        if num != 0 {
            ret.push(spell(num));
        } else if num == 0 && parts.len() == 1 {
            ret.push("zero".to_string());
        }
    }

    ret.join(" ")
}

fn spell(n: u64) -> String {
    if n >= 1_000 { panic!() }
    if n == 0 { return "zero".to_string() }

    let mut v = vec![];
    if n >= 100 {
        let a = n / 100;
        let bc = n % 100;

        v.push(spell(a));
        v.push("hundred".to_string());

        if bc != 0 { v.push(spell(bc)); }
    } else {
        v.push(match n {
            1 => "one".to_string(),
            2 => "two".to_string(),
            3 => "three".to_string(),
            4 => "four".to_string(),
            5 => "five".to_string(),
            6 => "six".to_string(),
            7 => "seven".to_string(),
            8 => "eight".to_string(),
            9 => "nine".to_string(),
            10 => "ten".to_string(),
            11 => "eleven".to_string(),
            12 => "twelve".to_string(),
            13 => "thirteen".to_string(),
            14 => "fourteen".to_string(),
            15 => "fifteen".to_string(),
            16 => "sixteen".to_string(),
            17 => "seventeen".to_string(),
            18 => "eighteen".to_string(),
            19 => "nineteen".to_string(),
            20 => "twenty".to_string(),
            21..=29 => format!("twenty-{}", spell(n%10)),
            30 => "thirty".to_string(),
            31..=39 => format!("thirty-{}", spell(n%10)),
            40 => "forty".to_string(),
            41..=49 => format!("forty-{}", spell(n%10)),
            50 => "fifty".to_string(),
            51..=59 => format!("fifty-{}", spell(n%10)),
            60 => "sixty".to_string(),
            61..=69 => format!("sixty-{}", spell(n%10)),
            70 => "seventy".to_string(),
            71..=79 => format!("seventy-{}", spell(n%10)),
            80 => "eighty".to_string(),
            81..=89 => format!("eighty-{}", spell(n%10)),
            90 => "ninety".to_string(),
            91..=99 => format!("ninety-{}", spell(n%10)),
            _ => panic!(),
        })
    }

    v.join(" ")
}
