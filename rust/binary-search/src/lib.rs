pub fn find<T: AsRef<[E]>, E: Ord>(array: T, key: E) -> Option<usize> {
    let array = array.as_ref();
    if array.is_empty() { return None }

    let mut low = 0;
    let mut high = array.len() - 1;

    while low < high {
        let mid = low + (high - low) / 2;

        if array[mid] == key { return Some(mid) }
        else if array[mid] > key { high = mid }
        else { low = mid + 1 }
    }

    if array[low] == key { return Some(low) }

    None
}
