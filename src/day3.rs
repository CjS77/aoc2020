use std::fs;

fn read_data() -> Vec<String> {
    let values = fs::read_to_string("assets/day3.txt").expect("Could not load file");
    values
        .split('\n')
        .filter(|&s| s.len() > 0)
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

pub fn day3a() -> String {
    let rows = read_data();
    let trees = track_path(&rows, 3, 1);
    trees.to_string()
}

pub fn day3b() -> String {
    let rows = read_data();
    let trees = [
        track_path(&rows, 1, 1),
        track_path(&rows, 3, 1),
        track_path(&rows, 5, 1),
        track_path(&rows, 7, 1),
        track_path(&rows, 1, 2),
    ]
    .iter()
    .product::<usize>();
    trees.to_string()
}

fn track_path(rows: &[String], n_right: usize, n_down: usize) -> usize {
    let (trees, _) =
        rows.iter()
            .skip(n_down)
            .step_by(n_down)
            .fold((0usize, 0usize), |(tot, pos), s| {
                let (new_pos, is_tree) = move_right(s.as_str(), pos, n_right);
                let new_tot = if is_tree { tot + 1 } else { tot };
                (new_tot, new_pos)
            });
    trees
}

fn move_right(s: &str, pos: usize, steps: usize) -> (usize, bool) {
    let n = s.len();
    let new_pos = (pos + steps) % n;
    let is_tree = char_at(s, new_pos) == '#';
    (new_pos, is_tree)
}

fn char_at(s: &str, pos: usize) -> char {
    s.as_bytes()[pos] as char
}
