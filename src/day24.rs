use crate::bits::read_data;
use strum::IntoEnumIterator;
use strum::EnumIter;
use fnv::FnvHashSet;

type MyHashSet = FnvHashSet<Position>;

pub fn day24a() -> String {
    let data = get_moves("assets/day24.txt");
    let set = process_moves(data);
    format!("{}", set.len())
}


pub fn day24b() -> String {
    let data = get_moves("assets/day24.txt");
    let day0 = process_moves(data);
    let day100 = (0..100).fold(day0, |floor_plan, _i| {
        // println!("Day {} - {}", i, floor_plan.len());
        flip_tiles(&floor_plan)
    });
    format!("{}", day100.len())
}

fn new_hashset() -> MyHashSet {
    MyHashSet::default()
}

fn get_moves(f: &str) -> Vec<Vec<Move>> {
    read_data(f)
        .iter()
        .filter(|&s| !s.is_empty())
        .map(str_to_moves)
        .collect()
}

fn locate_tile(moves: &[Move]) -> Position {
    moves.iter()
        .fold(Position::default(), |loc, m| {
            loc.offset(m)
        })
}

fn process_moves(moves: Vec<Vec<Move>>) -> MyHashSet {
    let mut result = new_hashset();
    for these_moves in moves.iter() {
        let pos = locate_tile(these_moves);
        if !result.remove(&pos) {
            result.insert(pos);
        }
    }
    result
}

fn flip_tiles(floor_plan: &MyHashSet) -> MyHashSet {
    let mut result = new_hashset();
    let (min_x, max_x, min_y, max_y) = floor_plan.iter()
        .fold((0,0,0,0), |bounds, pos| {
            (
                bounds.0.min(pos.x),
                bounds.1.max(pos.x),
                bounds.2.min(pos.y),
                bounds.3.max(pos.y),
            )
        });
    // println!("Bounds - {},{},{},{}", min_x, max_x, min_y, max_y);
    // Process the square set by the bounds
    for x in min_x-2..=max_x+2 {
        for y in min_y-1..=max_y+1 {
            // even x offsets must have even y offsets
            if ((x % 2 == 0) && (y % 2 == 1)) || ((x % 2 == 1) && (y % 2 ==0)) { continue; }
            let pos = Position { x, y };
            let is_black = floor_plan.contains(&pos);
            let n_neighbours = pos.count_neighbours(&floor_plan);
            if !is_black && n_neighbours == 2 {
                result.insert(pos);
            }
            // A black tile that's NOT flipped stays black and must be put in the new plan
            if is_black && (n_neighbours == 1 || n_neighbours == 2) {
                result.insert(pos);
            }
        }
    }
    result
}

#[derive(Debug, EnumIter, PartialEq)]
enum Move {
    East,
    SouthEast,
    SouthWest,
    NorthWest,
    NorthEast,
    West,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Hash, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    pub fn offset(&self, m: &Move) -> Position {
        use Move::*;
        let (dx, dy) = match m {
            East => (2, 0),
            West => (-2, 0),
            NorthWest => (-1, -1),
            NorthEast => (1, -1),
            SouthWest => (-1, 1),
            SouthEast => (1, 1),
        };
        Position {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    pub fn count_neighbours(&self, bag: &MyHashSet) -> usize {
        Move::iter().filter(|m| {
            let neighbour = self.offset(m);
            bag.contains(&neighbour)
        }).count()
    }

}

fn str_to_moves<S: AsRef<str>>(s: S) -> Vec<Move> {
    let mut result = Vec::new();
    let mut chars = s.as_ref().chars();
    while let Some(c) = chars.next() {
        let next = match c {
            'n' | 's' => format!("{}{}", c, chars.next().unwrap()),
            'e' | 'w' => String::from(c),
            _ => panic!("Invalid input")
        };
        let mov = match next.as_str() {
            "e" => Move::East,
            "w" => Move::West,
            "ne" => Move::NorthEast,
            "nw" => Move::NorthWest,
            "se" => Move::SouthEast,
            "sw" => Move::SouthWest,
            _ => unreachable!()
        };
        result.push(mov);
    }
    result
}