#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_subslice<T: PartialEq>(small: &[T], large: &[T]) -> bool {
    if small.len() > large.len() {
        return false;
    }
    if small.is_empty() {
        return true;
    }
    // sliding window over large of size small.len()
    for window in large.windows(small.len()) {
        if window == small {
            return true;
        }
    }
    false
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    if is_subslice(first_list, second_list) {
        return Comparison::Sublist;
    }
    if is_subslice(second_list, first_list) {
        return Comparison::Superlist;
    }
    Comparison::Unequal
}
