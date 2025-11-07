pub fn is_valid(code: &str) -> bool {
    // Remove spaces
    let stripped: String = code.chars().filter(|c| *c != ' ').collect();

    // Must be at least 2 digits
    if stripped.len() <= 1 {
        return false;
    }

    // Convert to digits and check validity
    let mut digits: Vec<u32> = Vec::with_capacity(stripped.len());
    for c in stripped.chars() {
        if let Some(d) = c.to_digit(10) {
            digits.push(d);
        } else {
            // found a non-digit character
            return false;
        }
    }

    let mut sum = 0;
    // We will iterate from right to left
    let len = digits.len();
    for (i, &digit) in digits.iter().rev().enumerate() {
        // i = 0 is rightmost, i=1 is second from right, etc.
        if i % 2 == 1 {
            // this is "every second digit" from the right: double it
            let mut dbl = digit * 2;
            if dbl > 9 {
                dbl -= 9;
            }
            sum += dbl;
        } else {
            sum += digit;
        }
    }

    sum % 10 == 0
}
