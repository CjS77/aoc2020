use crate::bits::read_data;
use std::collections::{HashMap, HashSet};
use std::panic::resume_unwind;
use strum::IntoEnumIterator;
use strum::EnumIter;

pub fn day24a() -> String {
    let data = get_moves("assets/day24.txt");
    let set = process_moves(data);

    format!("{}", set.len())
}


pub fn day24b() -> String {
    let data = get_moves("assets/day24.txt");
    let day0 = process_moves(data);
    let day100 = (0..100).fold(day0, |floor_plan, i| {
        println!("Day {} - {}", i, floor_plan.len());
        flip_tiles(&floor_plan)
    });
    format!("{}", day100.len())
}

fn get_moves(f: &str) -> Vec<Vec<Move>> {
    read_data(f).iter().filter(|&s| !s.is_empty())
        .map(|s| {
            let mut this_move = Vec::new();
            let mut i = 0;
            let s = s.as_bytes();
            while i < s.len() {
                let size = match s[i] as char {
                    'n' | 's' => 2,
                    'e' | 'w' => 1,
                    _ => unreachable!()
                };
                let next = slice_to_move(&s[i..i + size]);
                i += size;
                this_move.push(next);
            }
            this_move
        }).collect()
}

fn process_move(moves: &[Move]) -> Position {
    moves.iter()
        .fold(Position::default(), |loc, m| {
            loc.offset(m)
        })
}

fn process_moves(moves: Vec<Vec<Move>>) -> HashSet<Position> {
    let mut result = HashSet::new();
    for mov in moves.iter() {
        let pos = process_move(mov);
        if !result.remove(&pos) {
            result.insert(pos);
        }
    }
    result
}

fn flip_tiles(floor_plan: &HashSet<Position>) -> HashSet<Position> {
    let mut result = HashSet::new();
    let (min_x, max_x, min_y, max_y) = floor_plan.iter()
        .fold((0,0,0,0), |bounds, pos| {
            // println!("{:?}", pos);
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
            // println!("{:?} ({}) -> {} black neighbours", pos, is_black, n_neighbours);
            if !is_black && n_neighbours == 2 {
                result.insert(pos);
                // println!("Flipped to black");
            }
            // A black tile that's NOT flipped stays black and must be put in the new plan
            if is_black && (n_neighbours == 1 || n_neighbours == 2) {
                result.insert(pos);
                // println!("Stays black");
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

    pub fn count_neighbours(&self, bag: &HashSet<Position>) -> usize {
        Move::iter().filter(|m| {
            let neighbour = self.offset(m);
            // println!("{:?} => {:?} - {}", self, neighbour, bag.contains(&neighbour));
            bag.contains(&neighbour)
        }).count()
    }

}

fn slice_to_move(slice: &[u8]) -> Move {
    let s: String = slice.iter().map(|&v| v as char).collect();
    match s.as_str() {
        "e" => Move::East,
        "w" => Move::West,
        "ne" => Move::NorthEast,
        "nw" => Move::NorthWest,
        "se" => Move::SouthEast,
        "sw" => Move::SouthWest,
        _ => unreachable!()
    }
}