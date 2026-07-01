/// Validates whether all brackets, braces, and parentheses in the input string
/// are correctly nested and matched.
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut bracket_stack: Vec<char> = Vec::new();

    for character in string.chars() {
        match character {
            '[' => bracket_stack.push(']'),
            '{' => bracket_stack.push('}'),
            '(' => bracket_stack.push(')'),
            ']' | '}' | ')' => {
                if bracket_stack.pop() != Some(character) {
                    return false;
                }
            }
            _ => {}
        }
    }

    bracket_stack.is_empty()
}
