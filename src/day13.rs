use std::fs;

pub fn day13a() -> String {
    let (ts, buses) = read_data();
    let mut time = ts;
    let found = false;
    while !found {
        if let Some(id) = buses.iter().find(|&&id| time % id == 0) {
            return format!("{}", id * (time - ts));
        }
        time += 1;
    }
    format!("Ok")
}

fn is_prime(v: usize) -> bool {
    (2..(v as f64).sqrt() as usize).all(|d| v % d != 0)
}

// from: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
    let mut sum = 0;
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
    Some(sum % prod)
}


pub fn day13b() -> String {
    let mut vals = read_data2();
    vals.sort_by(|a, b| b.1.cmp(&a.1));
    let mods = vals.iter().map(|(_, p)| *p as i64).collect::<Vec<i64>>();
    let resids = vals.iter().map(|(i, p)| *p as i64 - *i as i64).collect::<Vec<i64>>();

    chinese_remainder(&resids, &mods).unwrap().to_string()
}

fn read_data2() -> Vec<(usize, usize)> {
    let values = fs::read_to_string("assets/day13.txt").expect("Could not load file");
    let lines = values
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect::<Vec<String>>();
    lines[1]
        .split(",")
        .enumerate()
        .filter_map(|(i, s)| s.parse::<usize>().ok().map(|v| (i, v)))
        .collect::<Vec<(usize, usize)>>()
}

fn read_data() -> (usize, Vec<usize>) {
    let values = fs::read_to_string("assets/day13.txt").expect("Could not load file");
    let lines = values
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect::<Vec<String>>();
    let ts = lines[0].parse::<usize>().unwrap();
    let mut busses = lines[1].split(",")
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    busses.sort_unstable();
    (ts, busses)
}