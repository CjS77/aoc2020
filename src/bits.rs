use std::fs;

// Set the ith bit of v
#[inline(always)]
pub fn set_bit(v: usize, index: usize) -> usize {
    v | (1 << index)
}

// Clear the ith bit of v
#[inline(always)]
pub fn clear_bit(v: usize, index: usize) -> usize {
    v & ( usize::MAX ^ (1 << index) )
}

/// Given the iterator that produces a list of indices to adjust, and a boolean to indicate
/// whether the bit must be cleared or not, modify the value accordingly
pub fn assign_bits<I: Iterator<Item=(usize, bool)>>(v: usize, iter: I) -> usize {
    iter.fold(v, |v, (i, set)| {
        if set { set_bit(v, i) } else { clear_bit(v, i) }
    })
}

// Checks whether the ith bit is set
pub fn is_set(v: usize, index: usize) -> bool {
    (1 << index) & v > 0
}

pub fn bit_array(v: usize) -> [bool; 64] {
    let mut result = [false; 64];
    for i in 0..64 {
        result[i] = is_set(v, i)
    }
    result
}

pub fn read_data(file: &str) -> Vec<String> {
    let values = fs::read_to_string(file).expect("Could not load file");
    values
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect::<Vec<String>>()
}