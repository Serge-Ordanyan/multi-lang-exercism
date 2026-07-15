/// Find an item in a sorted collection.
///
/// This works for slices, Vectors, and Arrays containing any type that implements `Ord`.
pub fn find<C, T>(container: C, key: T) -> Option<usize>
where
    C: AsRef<[T]>,
    T: Ord,
{
    let slice = container.as_ref();
    let mut low = 0;
    let mut high = slice.len();

    while low < high {
        let mid = low + (high - low) / 2;
        match slice[mid].cmp(&key) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
        }
    }

    None
}
