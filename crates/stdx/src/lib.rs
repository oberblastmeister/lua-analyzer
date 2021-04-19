pub mod paths;

/// Returns `idx` such that:
///
/// ```text
///     ∀ x in slice[..idx]:  pred(x)
///  && ∀ x in slice[idx..]: !pred(x)
/// ```
///
/// https://github.com/rust-lang/rust/issues/73831
pub fn partition_point<T, P>(slice: &[T], mut pred: P) -> usize
where
    P: FnMut(&T) -> bool,
{
    let mut left = 0;
    let mut right = slice.len();

    while left != right {
        let mid = left + (right - left) / 2;
        // SAFETY:
        // When left < right, left <= mid < right.
        // Therefore left always increases and right always decreases,
        // and either of them is selected.
        // In both cases left <= right is satisfied.
        // Therefore if left < right in a step,
        // left <= right is satisfied in the next step.
        // Therefore as long as left != right, 0 <= left < right <= len is satisfied
        // and if this case 0 <= mid < len is satisfied too.
        let value = unsafe { slice.get_unchecked(mid) };
        if pred(value) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}
