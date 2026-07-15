
pub fn abbreviate(phrase: &str) -> String {
    phrase
        // Split words by spaces and hyphens
        .split(|c: char| c == ' ' || c == '-')
        // Map each word to its starting letters (handling CamelCase transitions)
        .flat_map(|word| {
            let clean_word: String = word.chars().filter(|c| c.is_alphabetic()).collect();
            if clean_word.is_empty() {
                return vec![];
            }

            let mut letters = Vec::new();
            let chars: Vec<char> = clean_word.chars().collect();

            // If the word is entirely uppercase (e.g., PNG, LCD), just take the first letter
            if clean_word.chars().all(|c| c.is_uppercase()) {
                letters.push(chars[0].to_ascii_uppercase());
            } else {
                // Otherwise, take the first letter and any subsequent uppercase letters (CamelCase)
                letters.push(chars[0].to_ascii_uppercase());
                for i in 1..chars.len() {
                    if chars[i].is_uppercase() && chars[i - 1].is_lowercase() {
                        letters.push(chars[i]);
                    }
                }
            }
            letters
        })
        .collect()
}
