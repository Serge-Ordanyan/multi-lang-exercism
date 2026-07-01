/// Validates whether all brackets, braces, and parentheses in the input string
/// are correctly nested and matched.
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut bracket_stack: Vec<char> = Vec::new();

    for character in string.chars() {
        match character {
            // Push expected closing tokens onto our tracking stack trace
            '[' => bracket_stack.push(']'),
            '{' => bracket_stack.push('}'),
            '(' => bracket_stack.push(')'),

            // Validate closing conditions against our current stack checkpoint
            ']' | '}' | ')' => {
                if bracket_stack.pop() != Some(character) {
                    return false; // Mismatched or out-of-order bracket isolated
                }
            }

            // Ignore non-bracket programmatic syntax entirely
            _ => {}
        }
    }

    // The syntax is fully balanced only if the stack vector is perfectly empty
    bracket_stack.is_empty()
}
