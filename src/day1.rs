use itertools::Itertools;
use std::fs;

pub fn day1a() -> String {
    let values = read_data();
    find_expenses(2, &values)
}

pub fn day1b() -> String {
    let values = read_data();
    find_expenses(3, &values)
}

fn find_expenses(n: usize, values: &[usize]) -> String {
    match values
        .iter()
        .combinations(n)
        .find(|v| v.iter().copied().sum::<usize>() == 2020)
        .map(|v| v.into_iter().product::<usize>())
        .map(|v| v.to_string())
    {
        Some(v) => v,
        None => "No solution".to_string(),
    }
}

fn read_data() -> Vec<usize> {
    let values = fs::read_to_string("assets/day1a.txt").expect("Could not load file");
    values
        .split('\n')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<usize>>()
}
