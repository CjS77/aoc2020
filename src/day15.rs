use std::fs;

pub fn day15a() -> String {
    let lines = read_data();
    let res = 1;
    format!("{}", res)
}

pub fn day15b() -> String {
    let lines = read_data();
    let res = 1;
    format!("{}", res)
}

fn read_data() -> Vec<String> {
    let values = fs::read_to_string("assets/day14.txt").expect("Could not load file");
    values
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect::<Vec<String>>()
}