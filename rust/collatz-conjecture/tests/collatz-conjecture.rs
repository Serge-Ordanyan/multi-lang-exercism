use collatz_conjecture::collatz;

#[test]
fn test_element_one() {
    assert_eq!(collatz(1), Some(0));
}

#[test]
fn test_example_twelve_from_instructions() {
    assert_eq!(collatz(12), Some(9));
}

#[test]
fn test_zero_returns_none() {
    assert_eq!(collatz(0), None);
}

#[test]
fn test_large_even_number() {
    assert_eq!(collatz(1000000), Some(152));
}
