use matching_brackets::brackets_are_balanced;

#[test]
fn paired_square_brackets() {
    assert!(brackets_are_balanced("[]"));
}

#[test]
fn empty_string() {
    assert!(brackets_are_balanced(""));
}

#[test]
fn unpaired_brackets() {
    assert!(!brackets_are_balanced("[["));
}

#[test]
fn wrong_ordered_brackets() {
    assert!(!brackets_are_balanced("}{"));
}

#[test]
fn paired_with_unbalanced_nested_brackets() {
    assert!(!brackets_are_balanced("[(])"));
}

#[test]
fn complex_balanced_string_from_instructions() {
    assert!(brackets_are_balanced("{what is (42)}?"));
}

#[test]
fn complex_unbalanced_string_from_instructions() {
    assert!(!brackets_are_balanced("[text}"));
}
