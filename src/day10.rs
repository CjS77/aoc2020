use std::fs;

pub fn day10a() -> String {
    let mut adapters = vec![0usize];
    adapters.append(&mut read_data());
    let mut one = 0usize;
    let mut three = 1usize;
    for (i, adapter) in adapters.iter().enumerate().skip(1) {
        println!("{} {}", i, adapter);
        match adapter - adapters[i-1] {
            1 => one += 1,
            3 => three += 1,
            _ => {},
        }
    }
    format!("{}, {}, {}", one, three, one*three)

}

pub fn day10b() -> String {
    let mut adapters = vec![0usize];
    adapters.append(&mut read_data());
    // Push the device joltage
    adapters.push(adapters[adapters.len()-1] + 3);
    let mut swaps = vec![];
    let n = adapters.len();
    let mut cursor = n-1;
    while cursor > 0 {
        if adapters[cursor] <= 3 { break; }
        let target = adapters[cursor] - 3;
        let mut next_index = cursor - 1;
        while adapters[next_index] >= target {
            next_index -= 1;
        }
        match cursor - next_index {
            1 | 2 => {},
            3 => swaps.push(2),
            4 => swaps.push(4),
            _ => unreachable!()
        }
        println!("Cursor = {}, value = {}, next_index = {}", cursor, adapters[cursor], next_index);
        println!("{:?}", swaps);
        cursor = next_index + 1;
    }
    (swaps.iter().product::<usize>()).to_string()
}



fn read_data() -> Vec<usize> {
    let values = fs::read_to_string("assets/test.txt").expect("Could not load file");
    let mut values = values
        .split("\n")
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    values.sort();
    values
}