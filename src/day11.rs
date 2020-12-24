use std::fmt::{Display, Formatter};
use std::fs;

pub fn day11a() -> String {
    let layout = read_data();
    let result = layout.simulate(Layout::count_adjacent, 4);
    result.count_occupied().to_string()
}

pub fn day11b() -> String {
    let layout = read_data();
    let result = layout.simulate(Layout::count_visible, 5);
    result.count_occupied().to_string()
}

type CountFn = fn(&Layout, usize, usize) -> usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Place {
    Floor,
    Chair(bool),
}

impl Place {
    pub fn from_str(s: &str) -> Vec<Place> {
        s.chars()
            .map(|c| match c {
                '.' => Place::Floor,
                'L' => Place::Chair(false),
                '#' => Place::Chair(true),
                _ => unreachable!(),
            })
            .collect()
    }

    pub fn is_occupied(&self) -> bool {
        matches!(self, Place::Chair(true))
    }
}

impl Display for Place {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Floor => f.write_str("."),
            Self::Chair(false) => f.write_str("L"),
            Self::Chair(true) => f.write_str("#"),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Layout {
    rows: Vec<Vec<Place>>,
}

impl Layout {
    pub fn get(&self, row: usize, col: usize) -> Option<&Place> {
        self.rows.get(row)?.get(col)
    }

    pub fn is_occupied(&self, row: usize, col: usize) -> bool {
        self.get(row, col).map(|p| p.is_occupied()).unwrap_or(false)
    }

    pub fn count_visible(&self, row: usize, col: usize) -> usize {
        self.lookup(row, col, -1, -1)
            + self.lookup(row, col, -1, 0)
            + self.lookup(row, col, -1, 1)
            + self.lookup(row, col, 0, -1)
            + self.lookup(row, col, 0, 1)
            + self.lookup(row, col, 1, -1)
            + self.lookup(row, col, 1, 0)
            + self.lookup(row, col, 1, 1)
    }

    pub fn count_adjacent(&self, row: usize, col: usize) -> usize {
        let mut result = 0;
        result += if row > 0 && self.is_occupied(row - 1, col) { 1 } else { 0 };
        result += if row > 0 && col > 0 && self.is_occupied(row - 1, col - 1) { 1 } else { 0 };
        result += if row > 0 && self.is_occupied(row - 1, col + 1) { 1 } else { 0 };
        result += if self.is_occupied(row + 1, col) { 1 } else { 0 };
        result += if self.is_occupied(row + 1, col + 1) { 1 } else { 0 };
        result += if col > 0 && self.is_occupied(row, col - 1) { 1 } else { 0 };
        result += if col > 0 && self.is_occupied(row + 1, col - 1) { 1 } else { 0 };
        result += if self.is_occupied(row, col + 1) { 1 } else { 0 };
        result
    }

    pub fn simulate_one(&self, count_fn: CountFn, max_occupied: usize) -> Self {
        let mut result = self.clone();
        self.rows.iter().enumerate().for_each(|(row, seats)| {
            seats.iter().enumerate().for_each(|(col, p)| {
                let filled = count_fn(self, row, col);
                let occupied = self.is_occupied(row, col);
                if occupied && filled >= max_occupied {
                    result.rows[row][col] = Place::Chair(false);
                }
                if !occupied && filled == 0 && *p != Place::Floor {
                    result.rows[row][col] = Place::Chair(true);
                }
            })
        });
        result
    }

    pub fn simulate(&self, count_fn: CountFn, max_occupied: usize) -> Layout {
        let mut last = self.clone();
        loop {
            let next = last.simulate_one(count_fn, max_occupied);
            println!("{}\n\n\n", next);
            if next == last {
                return next;
            }
            last = next;
        }
    }

    pub fn count_occupied(&self) -> usize {
        self.rows.iter().fold(0usize, |r, row| {
            r + row.iter().filter(|p| p.is_occupied()).count()
        })
    }

    fn lookup(&self, row: usize, col: usize, dir_x: i32, dir_y: i32) -> usize {
        let mut cur_row = row;
        let mut cur_col = col;
        loop {
            cur_row = if (cur_row as i32 + dir_y) < 0 {
                return 0;
            } else {
                (cur_row as i32 + dir_y) as usize
            };
            cur_col = if (cur_col as i32 + dir_x) < 0 {
                return 0;
            } else {
                (cur_col as i32 + dir_x) as usize
            };
            match self.get(cur_row, cur_col) {
                Some(Place::Chair(true)) => return 1,
                Some(Place::Chair(false)) => return 0,
                Some(Place::Floor) => {}
                None => return 0,
            }
        }
    }
}

impl Display for Layout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for p in row {
                write!(f, "{}", p)?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn read_data() -> Layout {
    let values = fs::read_to_string("assets/day11.txt").expect("Could not load file");
    let rows = values
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| Place::from_str(s))
        .collect();
    Layout { rows }
}
