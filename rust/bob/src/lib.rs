pub fn reply(message: &str) -> &str {
    let msg = message.trim();

    if msg.is_empty() {
        return "Fine. Be that way!";
    }

    let is_question = msg.ends_with('?');
    let has_letters = msg.chars().any(|c| c.is_alphabetic());
    let is_yelling = has_letters && msg == msg.to_uppercase();

    match (is_yelling, is_question) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Whoa, chill out!",
        (false, true) => "Sure.",
        _ => "Whatever.",
    }
}
