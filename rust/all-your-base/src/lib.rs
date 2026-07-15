#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

/// Convert a number between two bases.
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // 1. Validate bases
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    // 2. Convert from input base to base 10
    let mut decimal_value = 0;
    for &digit in number {
        if digit >= from_base {
            return Err(Error::InvalidDigit(digit));
        }
        decimal_value = decimal_value * from_base + digit;
    }

    // 3. Handle base 10 value of 0 (including empty input slice)
    if decimal_value == 0 {
        return Ok(vec![0]);
    }

    // 4. Convert from base 10 to output base
    let mut result = Vec::new();
    while decimal_value > 0 {
        result.push(decimal_value % to_base);
        decimal_value /= to_base;
    }

    // Reverse the vector since digits are processed from least to most significant
    result.reverse();
    Ok(result)
}
