use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // Normalize the target word: lowercased, chars sorted
    let target_lower = word.to_lowercase();
    let mut target_chars: Vec<char> = target_lower.chars().collect();
    target_chars.sort_unstable();

    // For quick equality check (to skip identical words ignoring case)
    let target_lower_str = &*target_lower;

    possible_anagrams
        .iter()
        .filter_map(|&candidate| {
            let candidate_lower = candidate.to_lowercase();
            // skip if identical word (case-insensitive)
            if candidate_lower == target_lower_str {
                return None;
            }
            // normalize candidate: chars sorted
            let mut cand_chars: Vec<char> = candidate_lower.chars().collect();
            cand_chars.sort_unstable();

            if cand_chars == target_chars {
                Some(candidate)
            } else {
                None
            }
        })
        .collect()
}
