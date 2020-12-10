use itertools::Itertools;
use std::fs;

pub fn day9a() -> String {
    let data = read_data();
    match check_xmas(&data, 25) {
        None => "No solution".to_string(),
        Some(v) => v.to_string(),
    }
}

pub fn day9b() -> String {
    let data = read_data();
    if let Some(val) = check_xmas(&data, 25) {
        let n = data.len();
        for i in 0..n {
            let mut tot = 0usize;
            let mut offset = 1;
            while tot < val && i+offset < n {
                tot += data[i + offset];
                offset += 1;
            }
            if tot == val {
                let min = data[i..i+offset].iter().copied().min().unwrap();
                let max = data[i..i+offset].iter().copied().max().unwrap();
                return (min + max).to_string()
            }
        }
    }
    "No solution".to_string()
}

fn check_xmas(data: &[usize], preamble: usize) -> Option<usize> {
    for i in 0..data.len()-preamble {
        let window = &data[i..i+preamble];
        let target = data[preamble + i];
        if !window.iter().combinations(2).any(|v| v.iter().copied().sum::<usize>() == target) {
            return Some(target)
        }
    }
    None
}

fn read_data() -> Vec<usize> {
    let values = fs::read_to_string("assets/day9.txt").expect("Could not load file");
    values
        .split("\n")
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<usize>>()
}