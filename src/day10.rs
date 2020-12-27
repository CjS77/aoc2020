use std::fs;

pub fn day10a() -> String {
    let adapters = read_data();
    let (ones, threes) = &adapters.windows(2)
        .fold((0, 0), |(ones, threes), v| {
            match v[1] - v[0] {
                1 => (ones + 1, threes),
                3 => (ones, threes + 1),
                _ => (ones, threes)
            }
    });
    format!("{}", ones * threes)
}

pub fn day10b() -> String {
    let adapters = read_data();
    let mut result = Vec::new();
    // Scan the array, looking for 3-step jumps
    let mut start = 0;
    let mut end = 0;
    while end < adapters.len() - 1 {
        if adapters[end + 1] - adapters[end] == 3 {
            let count = count_configurations(&adapters[start..end + 1]);
            // println!("start: {}, end: {}, count: {}, seq: {:?}", start, end, count, &adapters[start..end + 1]);
            result.push(count);
            start = end + 1;
        }
        end += 1;
    }
    result.iter().product::<usize>().to_string()
}

fn validate_slice(values: &[usize]) -> bool {
    values.windows(2).all(|v| v[1] - v[0] <= 3)
}

// Convert val to a binary bitmap and select the elements of values that have the bits set
fn select_filter(val: usize, values: &[usize]) -> Vec<usize> {
    values.iter()
        .enumerate()
        .filter(|(i, _)| {
            // Is the ith bit of val set?
            val & (1 << *i) > 0
        })
        .map(|(_, v)| *v)
        .collect()
}

fn count_configurations(values: &[usize]) -> usize {
    if values.len() < 3 { return 1; }
    let mut result = 0usize;
    let n = 1 << (values.len() - 2);
    let sub_slice = &values[1..values.len()-1];
    for v in 0..n {
        let mut config = vec![values[0]];
        config.append(&mut select_filter(v, sub_slice));
        config.push(*values.last().unwrap());
        if validate_slice(&config) {
            result += 1;
        }
    }
    result
}


fn read_data() -> Vec<usize> {
    let mut result = vec![0usize];
    let values = fs::read_to_string("assets/day10.txt").expect("Could not load file");
    values
        .lines()
        .filter_map(|s| s.parse::<usize>().ok())
        .for_each(|v| result.push(v));
    result.sort_unstable();
    // Push the device joltage
    result.push(result[result.len() - 1] + 3);
    result
}

