use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut nums = HashSet::new();

    for &factor in factors {
        if factor == 0 {
            continue; // avoid infinite loop / division by zero
        }

        let mut multiple = factor;
        while multiple < limit {
            nums.insert(multiple);
            multiple += factor;
        }
    }

    nums.iter().sum()
}
