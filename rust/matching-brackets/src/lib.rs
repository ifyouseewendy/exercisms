pub fn brackets_are_balanced(string: &str) -> bool {
    let mut s = string.to_string();
    s.retain(is_target);

    let mut stack = vec![];

    for c in s.chars() {
        if stack.is_empty() {
            stack.push(c);
        } else {
            match stack.last() {
                Some('[') => {
                    if c == ']' { stack.pop(); } else { stack.push(c); }
                },
                Some('(') => {
                    if c == ')' { stack.pop(); } else { stack.push(c); }
                },
                Some('{') => {
                    if c == '}' { stack.pop(); } else { stack.push(c); }
                },
                Some(_) => (),
                None => (),
            }
        }
    }

    stack.is_empty()
}

fn is_target(c: char) -> bool {
    c == '[' || c == ']' || c == '(' || c == ')' || c == '{' || c == '}'
}
