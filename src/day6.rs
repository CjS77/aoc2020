use std::collections::HashSet;
use std::fs;

pub fn day6a() -> String {
    let groups = read_data();
    groups.iter()
        .map(| group| {
            count_answers(group.as_str())
        }).sum::<usize>().to_string()
}

pub fn day6b() -> String {
    let groups = read_data();
    groups.iter()
        .map( | group| {
            count_all_answered(group.as_str())
        }).sum::<usize>().to_string()
}

fn count_all_answered(s: &str) -> usize {
    // Assuming no-one answers the same question more than once
    let groups_size = s.split('\n').filter(|s| !s.is_empty()).count();
    let mut result = [0usize; 26];
    s.chars().filter(|c| *c >= 'a' && *c <= 'z')
        .for_each(|c| {
            let index = c as usize - 97;
            result[index] += 1
        });
    result.iter().filter(|&&elem| elem == groups_size).count()
}

fn count_answers(s: &str) -> usize {
    let mut result = HashSet::new();
    s.chars().filter(|c| *c >= 'a' && *c <= 'z')
        .for_each(|c| { result.insert(c); });
    result.len()
}

fn read_data() -> Vec<String> {
    fs::read_to_string("assets/day6.txt")
        .expect("Could not read file")
        .split("\n\n")
        .map(String::from)
        .collect::<Vec<String>>()
}