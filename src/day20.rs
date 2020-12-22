use crate::bits::read_data;
use crate::backtracker::{State, depth_first_iterator};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::slice::{Iter};
use std::fmt::{Display, Formatter};
use std::fmt;
use itertools::Itertools;

pub fn day20a() -> String {
    let tiles = read_tiles();
    let n = (tiles.len() as f64).sqrt();
    println!("{} tiles ({} x {})", tiles.len(), n, n);
    let corners = find_corners(&tiles);
    println!("Corners: {:?}", corners);
    tiles.iter().filter(|&t| corners.contains(&t.id))
        .for_each(|t| {
            println!("{:?}", t.unique_edges(&tiles))
        });
    format!("{}", corners.iter().product::<usize>())
}

pub fn day20b() -> String {
    let tiles = read_tiles();
    let corner = tiles.iter().find(|t| t.id == 2711).unwrap().clone();
    println!("Corner:\n{}", corner);
    let mut image = Image::new(tiles);
    image.place_tile(&corner, true, 1, 0);
    let image = depth_first_iterator(image).unwrap();
    println!("{}", image);
    format!("")
}

fn read_tiles() -> Vec<Tile> {
    let data = read_data("assets/day20.txt");
    data[0..].chunks(12)
        .filter(|t| t.len() >= 10)
        .map(|t| {
            // println!("{}", &t[0][5..9]);
            let id = t[0][5..9].parse::<usize>().unwrap();
            Tile::new(id, &t[1..11])
        }).collect()
}

const MONSTER: [&str; 3] = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   "
];

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    id: usize,
    data: [char; 100],
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for chunk in self.data[..].chunks(10) {
            let s: String = chunk.iter().cloned().collect();
            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}

impl Tile {
    pub fn new(id: usize, s: &[String]) -> Tile {
        let mut data = ['.'; 100];
        for (i, line) in s.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                data[i * 10 + j] = c;
            }
        }
        Self { id, data }
    }

    pub fn transpose(&self) -> Tile {
        let mut new_data = self.data.clone();
        for (i, val) in self.data.iter().enumerate() {
            let (x, y) = i_to_xy(i);
            let new_i = xy_to_i((y, x));
            new_data[new_i] = *val;
        }
        Tile { id: self.id, data: new_data }
    }

    pub fn flip_x(&self) -> Tile {
        let mut new_data = ['X'; 100];
        for (i, val) in self.data.iter().enumerate() {
            let (x, y) = i_to_xy(i);
            let new_i = xy_to_i((9 - x, y));
            new_data[new_i] = *val;
        }
        Tile { id: self.id, data: new_data }
    }

    pub fn flip_y(&self) -> Tile {
        let mut new_data = self.data.clone();
        for (i, val) in self.data.iter().enumerate() {
            let (x, y) = i_to_xy(i);
            let new_i = xy_to_i((x, 9 - y));
            new_data[new_i] = *val;
        }
        Tile { id: self.id, data: new_data }
    }

    pub fn rotate(&self, dir: usize) -> Tile {
        match dir {
            0 => self.clone(),
            1 => self.transpose().flip_x(),
            2 => self.flip_x().flip_y(),
            3 => self.transpose().flip_y(),
            _ => unreachable!()
        }
    }

    pub fn get_edge(&self, i: usize) -> String {
        match i {
            0 => self.data[0..10].iter().collect(),
            1 => (0..10).map(|i| 9 + 10 * i).map(|i| self.data[i]).collect(),
            2 => self.data[90..100].iter().collect(),
            3 => (0..10).map(|i| 10 * i).map(|i| self.data[i]).collect(),
            _ => unreachable!(),
        }
    }

    pub fn match_edge(&self, this_edge: usize, flip: bool, other: &Tile, other_edge: usize) -> bool {
        let mut this = self.get_edge(this_edge);
        if flip { this = this.chars().rev().collect(); }
        let that = other.get_edge(other_edge);
        this == that
    }

    pub fn find_matching_edge(&self, this_edge: usize, other: &Tile) -> Vec<EdgeMatch> {
        // println!("\nLooking for edge matches on {}.{} against {}", self.id, this_edge, other.id);
        let mut res = Vec::new();
        for that_edge in 0..4 {
            for flip in &[false, true] {
                if self.match_edge(this_edge, *flip, other, that_edge) {
                    res.push(EdgeMatch { flip: *flip, edge: this_edge, that_id: other.id, that_edge });
                    // println!("{}.{} ({}) matches {}.{}", this.id, this_edge, flip, other.id, that_edge);
                }
            }
        }
        res
    }

    pub fn find_matches_on_edge(&self, edge: usize, other: &HashSet<Tile>) -> Vec<Placement> {
        let that_edge = (edge + 2) % 4;
        let orientations = [
            (false, 0),
            (false, 1),
            (false, 2),
            (false, 3),
            (true, 0),
            (true, 1),
            (true, 2),
            (true, 3),
        ];
        let mut result = Vec::new();
        for t in other {
            for (flip, rot) in orientations.into_iter() {
                let mut tile = t.rotate(*rot);
                if *flip { tile = tile.flip_x(); }
                if self.match_edge(edge, false, &tile, that_edge) {
                    let p = Placement {
                        id: t.id,
                        pos: 0,
                        flip: *flip,
                        rot: that_edge,
                    };
                    result.push(p);
                }
            }
        }
        result
    }

    pub fn get_all_matches(&self, others: &[Tile]) -> Vec<EdgeMatch> {
        (0..4).map(|edge| {
            others.iter().filter(|&t| t != self)
                .map(|t| self.find_matching_edge(edge, t))
                .flatten()
                .collect::<Vec<EdgeMatch>>()
        })
            .flatten()
            .collect()
    }

    pub fn unique_edges(&self, others: &[Tile]) -> Vec<usize> {
        (0..4).filter(|i| {
            others.iter()
                .filter(|&t| t != self)
                .all(|t| {
                    self.find_matching_edge(*i, t).is_empty()
                })
        }).collect::<Vec<usize>>()
    }

    pub fn is_corner(&self, others: &[Tile]) -> bool {
        self.unique_edges(others).len() == 2
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct EdgeMatch {
    flip: bool,
    edge: usize,
    that_id: usize,
    that_edge: usize,
}


#[derive(Clone)]
pub struct Placement {
    id: usize,
    pos: usize,
    flip: bool,
    rot: usize,
}

#[derive(PartialEq, Eq, Clone)]
pub struct Image {
    size: usize,
    edges: HashSet<Tile>,
    insides: HashSet<Tile>,
    arrangement: Vec<Option<Tile>>,
}

impl Image {
    pub fn new(tiles: Vec<Tile>) -> Self {
        let n = (tiles.len() as f64).sqrt() as usize;
        let mut edges = HashSet::new();
        let mut insides = HashSet::new();
        for tile in &tiles {
            match tile.unique_edges(&tiles).len() {
                1 | 2 => { edges.insert(tile.clone()); }
                0 => { insides.insert(tile.clone()); }
                _ => unreachable!()
            }
        }
        Self {
            size: n,
            edges,
            insides,
            arrangement: vec![None; n * n],
        }
    }

    // 0 = corner, 1 = edge, 2 = inside
    fn location(&self, pos: usize) -> usize {
        let n = self.size - 1;
        match self.i_to_xy(pos) {
            (x, y) if x == 0 && y > 0 && y < n => 1,
            (x, y) if x == n && y > 0 && y < n => 1,
            (x, y) if y == 0 && x > 0 && x < n => 1,
            (x, y) if y == n && x > 0 && x < n => 1,
            (x, y) if x > 0 && x < n && y > 0 && y < n => 2,
            _ => 0
        }
    }

    pub fn place_tile(&mut self, tile: &Tile, flip: bool, rot: usize, pos: usize) -> Option<()> {
        let n = self.size;
        if pos >= n * n {
            println!("Placement out of bounds");
            return None;
        }
        if self.arrangement[pos].is_some() {
            println!("Place over existing tile");
            return None;
        }
        let mut bag = match self.location(pos) {
            0 | 1 => &mut self.edges,
            2 => &mut self.insides,
            _ => unreachable!()
        };
        let mut tile = bag.take(tile)?;
        tile = tile.rotate(rot);
        if flip {
            tile = tile.flip_x();
        }
        self.arrangement[pos] = Some(tile);
        Some(())
    }

    #[inline(always)]
    fn xy_to_i(&self, x: usize, y: usize) -> usize {
        self.size * y + x
    }

    #[inline(always)]
    fn i_to_xy(&self, i: usize) -> (usize, usize) {
        let x = i % self.size;
        let y = i / self.size;
        (x, y)
    }

    fn neighbour_pos(&self, i: usize, dir: usize) -> Option<usize> {
        let i = i as isize;
        let ni = match dir {
            0 => i - self.size as isize,
            1 => i + 1,
            2 => i + self.size as isize,
            3 => i - 1,
            _ => unreachable!()
        };
        if ni < 0 || ni >= self.arrangement.len() as isize {
            return None;
        }
        Some(ni as usize)
    }

    fn is_empty_in_dir(&self, i: usize, dir: usize) -> Option<usize> {
        let ni = self.neighbour_pos(i, dir)?;
        match self.arrangement[ni] {
            None => Some(ni),
            Some(_) => None,
        }
    }

    fn get_moves(&self, edge: bool) -> Box<dyn Iterator<Item=Placement>> {
        let bag = if edge { &self.edges } else { &self.insides };
        let mut moves: Vec<Placement> = Vec::new();
        // For each occupied tile in edge / insides...
        self.arrangement.iter()
            .enumerate()
            .filter(|(i, _t)| {
                let is_edge = self.location(*i) != 2;
                !(is_edge ^ edge)
            })
            .filter(|(_i, t)| t.is_some())
            .for_each(|(i, t)| {
                let t = t.as_ref().unwrap();
                // ... look for matches along each edge
                for edge in 0usize..4 {
                    if let Some(new_i) = self.is_empty_in_dir(i, edge) {
                        let mut matches = t.find_matches_on_edge(edge, bag);
                        matches.iter_mut().for_each(|p| p.pos = new_i);
                        moves.append(&mut matches);
                    }
                }
            });
        Box::new(moves.into_iter())
    }

    pub fn check_placement(&self) -> Option<()> {
        // Loop through every placed tile and check the edges
        let placed_tiles = self.arrangement.iter()
            .enumerate()
            .filter(| (_, t)| t.is_some())
            .map(|(i, t)| (i, t.as_ref().unwrap()));
        for (pos, tile) in  placed_tiles {
            for edge in 0..4 {
                if self.is_empty_in_dir(pos, edge).is_none() {
                    if let Some(neighbour_pos) = self.neighbour_pos(pos, edge) {
                        let other = self.arrangement[neighbour_pos].as_ref().unwrap();
                        let that_edge = (edge + 2) % 4;
                        if !tile.match_edge(edge, false, other, that_edge) {
                            return None;
                        }
                    }
                }
            }
        }
        Some(())
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row_num in 0..self.size * 10 {
            let row = (0..self.size).map(|block_x| {
                let block_y = row_num / 10;
                let block_i = block_y * self.size + block_x;
                let j = row_num % 10;
                match self.arrangement[block_i] {
                    None => ".".repeat(10),
                    Some(t) => (&t.data[j * 10..j * 10 + 10]).iter().cloned().collect(),
                }
            }).join(" ");
            writeln!(f, "{}", row)?;
            if (row_num + 1) % 10 == 0 {
                writeln!(f, "")?;
            }
        }
        Ok(())
    }
}

impl State for Image {
    type Move = Placement;

    fn is_solved(&self) -> bool {
        self.arrangement.iter().all(|t| t.is_some())
    }

    fn get_moves<I>(&self) -> Box<dyn Iterator<Item=Placement>> where I: Iterator<Item=Placement> {
        let moves = self.get_moves(!self.edges.is_empty());
        Box::new(moves.into_iter())
    }

    fn apply(&self, p: &Placement) -> Option<Self> {
        let tile = self.edges.iter().find(|t| t.id == p.id)
            .or_else(|| self.insides.iter().find(|t| t.id == p.id))?;
        let mut next_state = self.clone();
        println!("{} tiles placed", self.arrangement.iter().filter(|t| t.is_some()).count());
        println!("{}", self);
        next_state.place_tile(tile, p.flip, p.rot, p.pos)?;
        next_state.check_placement().map(|_| next_state)
    }
}

fn remove_tile(tile: &Tile, tiles: &mut Vec<Tile>) {
    let index = tiles.iter().enumerate().find(|(_, &t)| t.id == tile.id).map(|(i, _)| i).unwrap();
    tiles.remove(index);
}

#[inline(always)]
fn i_to_xy(i: usize) -> (usize, usize) {
    (i % 10, i / 10)
}

#[inline(always)]
fn xy_to_i(pos: (usize, usize)) -> usize {
    pos.0 + pos.1 * 10
}

fn find_corners(tiles: &[Tile]) -> Vec<usize> {
    let mut res = Vec::new();
    for tile in tiles.iter() {
        if tile.is_corner(tiles) {
            res.push(tile.id)
        }
    }
    res
}

