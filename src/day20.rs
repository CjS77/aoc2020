use crate::bits::read_data;
use std::collections::HashMap;
use itertools::Itertools;

pub fn day20a() -> String {
    let tiles = read_tiles();
    let n = (tiles.len() as f64).sqrt();
    println!("{} tiles ({} x {})", tiles.len(), n, n);
    let corners = find_corners(&tiles);
    println!("Corners: {:?}", corners);
    format!("{}", corners.iter().product::<usize>())
}

pub fn day20b() -> String {
    let tiles = read_tiles();
    let image = Image::build_image(tiles);
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

    pub fn is_corner(&self, others: &[Tile]) -> bool {
        let no_matches: Vec<usize> = (0..4).filter(|i| {
            others.iter()
                .filter(|&t| t != self)
                .all(|t| {
                    // println!("{}.{} vs {}", self.id, *i, t.id);
                    self.find_matching_edge(*i, t).is_empty()
                })
        }).collect();
        // println!("{} had these edges with no matches: {:?}", self.id, no_matches);
        no_matches.len() == 2
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct EdgeMatch {
    flip: bool,
    edge: usize,
    that_id: usize,
    that_edge: usize,
}


pub struct Image {
    data: Vec<char>
}

impl Image {
    pub fn new(n: usize) -> Self {
        let size = n * 100;
        Self { data: vec!['@'; size] }
    }

    fn build_image(tiles: Vec<Tile>) -> Image {
        let mut image = Image::new(tiles.len());
        let corners = find_corners(&tiles);
        println!("Corners: {:?}", corners);

        assert_eq!(corners.len(), 4);
        let mut match_db = HashMap::<usize, Vec<EdgeMatch>>::new();
        tiles.iter()
            .for_each(|t| {
                let matches = t.get_all_matches(&tiles);
                match_db.insert(t.id, matches);
            });

        // Find the top left corner -- this can be any corner tile, the image will just be
        // rotated / flipped

        let (corner, m) = match_db.iter().find(|(&id, &m)| m.len() == 2).unwrap();
        // How much to rotate?
        let index = m.iter().map(|m| m.edge).min().unwrap();
        let corner = remove_tile(&corner)
        image
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

