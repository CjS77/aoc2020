use crate::bits::read_data;

pub fn day22a() -> String {
    let foods = read_ingredients();
    format!("{}", 1)
}

pub fn day22b() -> String {
    let foods = read_ingredients();

    format!("{}", 1)
}

pub fn read_ingredients() -> Vec<()> {
    let data = read_data("assets/day21.txt");
    data.iter()
        .filter(|s| !s.is_empty())
        .map(|s| ())
        .collect()
}