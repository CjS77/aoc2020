use regex::Regex;
use std::fs;

pub fn day4a() -> String {
    let passports = read_data();
    passports
        .iter()
        .filter_map(|s| Passport::all_fields_present(s.as_str()))
        .count()
        .to_string()
}

pub fn day4b() -> String {
    let passports = read_data();
    passports
        .iter()
        .filter_map(|s| Passport::new(s.as_str()))
        .count()
        .to_string()
}

#[derive(Default, Debug)]
pub struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>, // (Country ID)
}

impl Passport {
    fn new(s: &str) -> Option<Passport> {
        Self::all_fields_present(s).and_then(Self::check_validity)
    }

    pub fn all_fields_present(s: &str) -> Option<Passport> {
        let mut result = Passport::default();
        let fields = s.split_whitespace().collect::<Vec<&str>>();
        for field in fields {
            let parts = field.split(':').collect::<Vec<&str>>();
            if parts.len() == 2 {
                match parts[0] {
                    "byr" => result.byr = parts[1].to_string(),
                    "iyr" => result.iyr = parts[1].to_string(),
                    "eyr" => result.eyr = parts[1].to_string(),
                    "hgt" => result.hgt = parts[1].to_string(),
                    "hcl" => result.hcl = parts[1].to_string(),
                    "ecl" => result.ecl = parts[1].to_string(),
                    "pid" => result.pid = parts[1].to_string(),
                    "cid" => result.cid = Some(parts[1].to_string()),
                    _ => {}
                }
            }
        }
        if !result.byr.is_empty()
            && !result.iyr.is_empty()
            && !result.eyr.is_empty()
            && !result.hgt.is_empty()
            && !result.hcl.is_empty()
            && !result.ecl.is_empty()
            && !result.pid.is_empty()
        {
            Some(result)
        } else {
            None
        }
    }

    fn check_validity(self) -> Option<Passport> {
        if check_pid(&self.pid)
            && check_byr(&self.byr)
            && check_iyr(&self.iyr)
            && check_eyr(&self.eyr)
            && check_hgt(&self.hgt)
            && check_hcl(&self.hcl)
            && check_ecl(&self.ecl)
        {
            Some(self)
        } else {
            None
        }
    }
}

fn check_byr(s: &str) -> bool {
    matches!(s.parse::<usize>(), Ok(v) if v >= 1920 && v <= 2002)
}

fn check_iyr(s: &str) -> bool {
    match s.parse::<usize>() {
        Ok(v) => v >= 2010 && v <= 2020,
        _ => false,
    }
}

fn check_eyr(s: &str) -> bool {
    match s.parse::<usize>() {
        Ok(v) => v >= 2020 && v <= 2030,
        _ => false,
    }
}

fn check_hgt(s: &str) -> bool {
    let re = Regex::new(r"^(\d*)(in|cm)$").unwrap();
    match re.captures(s) {
        Some(cap) => {
            let val = cap[1].parse::<usize>().unwrap_or(0);
            let unit = &cap[2];
            match unit {
                "cm" => val >= 150 && val <= 193,
                "in" => val >= 59 && val <= 76,
                _ => false,
            }
        }
        None => false,
    }
}

fn check_hcl(s: &str) -> bool {
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    re.is_match(s)
}

fn check_ecl(s: &str) -> bool {
    matches!(s, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}

fn check_pid(s: &str) -> bool {
    let re = Regex::new(r"^\d{9}$").unwrap();
    re.is_match(s)
}

fn read_data() -> Vec<String> {
    fs::read_to_string("assets/day4.txt")
        .expect("Could not read file")
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}
