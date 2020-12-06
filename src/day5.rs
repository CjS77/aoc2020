use std::fs;

pub fn day5a() -> String {
    let cards = read_data();
    println!("{}", cards.len());
    cards.iter()
        .map(|s| find_id(s.as_str()))
        .max()
        .unwrap()
        .to_string()
}

pub fn day5b() -> String {
    let cards = read_data();
    let mut seats = [false; 128 * 8];
    cards.iter()
        .for_each(|s| {
            let id = find_id(s);
            seats[id] = true;
        });
    for i in 1..128*8-1 {
        if !seats[i] && seats[i-1] && seats[i+1] {
            return i.to_string();
        }
    }
    "No solution".to_string()
}

pub fn find_id(s: &str) -> usize {
    let row = find_row(s);
    let seat = find_seat(s);
    row * 8 + seat
}

pub fn find_row(s: &str) -> usize {
    find_index(128, 'F', 'B', &s[0..7])
}

pub fn find_seat(s: &str) -> usize {
    find_index(8, 'L', 'R', &s[7..10])
}

pub fn find_index(len: usize, bottom: char, top: char, code: &str) -> usize {
    let (low, _high, _rem) = code.chars()
        .fold((0usize, len-1usize, len), |state, c| {
            let (mut low, mut high, mut rem) = state;
            rem = rem / 2;
            match c {
                c if c == bottom => high = high - rem,
                c if c == top => low = low + rem,
                _ => unreachable!()
            }
            (low, high, rem)
        });
    low
}

fn read_data() -> Vec<String> {
    fs::read_to_string("assets/day5.txt")
        .expect("Could not read file")
        .split("\n")
        .filter(|&s| s.len() == 10)
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}