pub fn reply(message: &str) -> &str {
    if message.trim().is_empty() {
        return "Fine. Be that way!";
    }

    let is_question = message.trim().ends_with('?');

    let m: String = message.chars().filter(char::is_ascii_alphabetic).collect();
    let all_uppercase = !m.is_empty() && m.chars().all(|c| char::is_ascii_uppercase(&c));

    if is_question && all_uppercase {
        "Calm down, I know what I'm doing!"
    } else if is_question {
        "Sure."
    } else if all_uppercase {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
