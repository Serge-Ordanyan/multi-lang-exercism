pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec![String::new(); digits.len() + 1];
    }

    if len > digits.len() {
        return Vec::new();
    }

    let chars: Vec<char> = digits.chars().collect();

    chars
        .windows(len)
        .map(|window| window.iter().collect())
        .collect()
}
