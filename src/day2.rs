use std::fs;
use regex::Regex;

pub fn day2a() -> String {
    let policies = read_data();
    let total_valid = policies
        .iter()
        .filter(|&p| p.is_valid_at_sled())
        .count();
    total_valid.to_string()
}

pub fn day2b() -> String {
    let policies = read_data();
    let total_valid = policies
        .iter()
        .filter(|&p| p.is_valid_at_tobbogan())
        .count();
    total_valid.to_string()
}

const REGEX: &str = r"^(\d+)-(\d+) (.): (.*)$";

#[derive(Debug)]
pub struct PasswordPolicy {
    min: usize,
    max: usize,
    letter: String,
    password: String,
}

impl PasswordPolicy {
    pub fn new(s: &str, re: &Regex) -> Option<Self> {
        let matches = re.captures(s)?;
        let min = matches.get(1).and_then(|s| s.as_str().parse::<usize>().ok())?;
        let max = matches.get(2).and_then(|s| s.as_str().parse::<usize>().ok())?;
        let letter = matches.get(3)?.as_str().to_string();
        let password = matches.get(4)?.as_str().to_string();
        Some(Self { min, max, letter, password })
    }

    pub fn is_valid_at_sled(&self) -> bool {
        // Count how many times letter is in password
        let count = self.password.matches(&self.letter).count();
        count >= self.min && count <= self.max
    }

    pub fn is_valid_at_tobbogan(&self) -> bool {
        if self.password.len() < self.min.max(self.max) { return false; }
        let letter = self.letter.as_bytes()[0] as char;
        let chars = self.password.chars().collect::<Vec<char>>();
        let first = chars[self.min - 1] == letter;
        let second = chars[self.max - 1] == letter;
        first ^ second
    }
}

fn read_data() -> Vec<PasswordPolicy> {
    let values = fs::read_to_string("assets/passwords.txt").expect("Could not load file");
    let regex = Regex::new(REGEX).unwrap();
    values
        .split('\n')
        .filter_map(|s| PasswordPolicy::new(s, &regex))
        .collect()
}