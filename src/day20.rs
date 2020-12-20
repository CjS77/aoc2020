use crate::bits::read_data;
use itertools::Itertools;

pub fn day20a() -> String {
    let tiles = read_tiles();
    let n = (tiles.len() as f64).sqrt();
    println!("{} tiles ({} x {})", tiles.len(), n, n);
    // check(&tiles);
    let corners = find_corners(&tiles);
    println!("{:?}", corners);
    format!("{}", corners.iter().product::<usize>())
}

pub fn day20b() -> String {
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    id: usize,
    data: [char; 100],
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

    pub fn find_matching_edge(&self, this_edge: usize, other: &Tile) -> Vec<Orientation> {
        // println!("\nLooking for edge matches on {}.{} against {}", self.id, this_edge, other.id);
        let mut res = Vec::new();
        let mut this = *self;
        for that_edge in 0..4 {
            for flip in &[false, true] {
                if this.match_edge(this_edge, *flip, other, that_edge) {
                    res.push(Orientation { flip_x: *flip, edge: this_edge });
                    println!("{}.{} ({}) matches {}.{}", this.id, this_edge, flip, other.id, that_edge);
                }
            }
        }
        res
    }

    pub fn is_corner(&self, others: &[Tile]) -> bool {
        let no_matches: Vec<usize> = (0..4).filter(|i| {
            others.iter()
                .filter(|&t| t != self)
                .all(|t| {
                    // println!("{}.{} vs {}", self.id, *i, t.id);
                    self.find_matching_edge(*i, t).is_empty()
                })
        }).collect();
        println!("{} had these edges with no matches: {:?}", self.id, no_matches);
        no_matches.len() == 2
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Orientation {
    flip_x: bool,
    edge: usize,
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

fn check(tiles: &[Tile]) {
    let c = tiles.iter().find(|t| t.id == 1171).unwrap();
    let a = tiles.iter().find(|t| t.id == 1489).unwrap();
    let cf = c.flip_x();
    let m = c.find_matching_edge(1, a);
    println!("Matches: {:?}", m);
}
