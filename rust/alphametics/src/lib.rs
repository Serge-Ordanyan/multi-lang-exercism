use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // Parse the alphanumeric words from the formula
    let words: Vec<&str> = input
        .split(|c: char| !c.is_alphabetic())
        .filter(|s| !s.is_empty())
        .collect();
    if words.len() < 2 {
        return None;
    }

    let summands = &words[0..words.len() - 1];
    let result_word = words[words.len() - 1];

    // Collect all unique characters
    let mut chars: Vec<char> = input
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    if chars.len() > 10 {
        return None; // There are only 10 unique digits (0-9)
    }
    chars.sort();

    // Identify characters that cannot be zero (leading letters of words)
    let mut leading_chars = HashSet::new();
    for word in &words {
        if let Some(first) = word.chars().next() {
            leading_chars.insert(first);
        }
    }

    let mut assigned = HashMap::new();
    let mut used_digits = [false; 10];

    if backtrack(0, &chars, &mut assigned, &mut used_digits, &leading_chars, summands, result_word) {
        Some(assigned)
    } else {
        None
    }
}

fn backtrack(
    idx: usize,
    chars: &[char],
    assigned: &mut HashMap<char, u8>,
    used_digits: &mut [bool; 10],
    leading_chars: &HashSet<char>,
    summands: &[&str],
    result_word: &str,
) -> bool {
    if idx == chars.len() {
        return evaluate(summands, result_word, assigned);
    }

    let c = chars[idx];
    let start_digit = if leading_chars.contains(&c) { 1 } else { 0 };

    for digit in start_digit..=9 {
        if !used_digits[digit as usize] {
            used_digits[digit as usize] = true;
            assigned.insert(c, digit);

            if backtrack(idx + 1, chars, assigned, used_digits, leading_chars, summands, result_word) {
                return true;
            }

            assigned.remove(&c);
            used_digits[digit as usize] = false;
        }
    }
    false
}

fn evaluate(summands: &[&str], result_word: &str, assigned: &HashMap<char, u8>) -> bool {
    let to_val = |word: &str| -> u64 {
        word.chars().fold(0, |acc, c| acc * 10 + (*assigned.get(&c).unwrap() as u64))
    };

    let sum: u64 = summands.iter().map(|&w| to_val(w)).sum();
    sum == to_val(result_word)
}
