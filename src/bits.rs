// Set the ith bit of v
#[inline(always)]
pub fn set_bit(v: usize, index: usize) -> usize {
    v | (1 << index)
}

// Clear the ith bit of v
#[inline(always)]
pub fn clear_bit(v: usize, index: usize) -> usize {
    v & ( u64::MAX ^ (1 << index) )
}

/// Given the iterator that produces a list of indices to adjust, and a boolean to indicate
/// whether the bit must be cleared or not, modify the value accordingly
pub fn assign_bits<I: Iterator<Item=(usize, bool)>>(v: usize, iter: I) -> usize {
    iter.fold(v, |(i, set)| {
        if set { set_bit(v, i) } else { clear_bit(v, i) }
    })
}