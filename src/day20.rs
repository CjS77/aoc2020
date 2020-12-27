use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Add, Not};
use std::fmt::{Display, Formatter};

pub fn day20a() -> String {
    let (_tiles, edges) = read_tiles();
    let edge_tiles: Vec<_> = edges
        .iter()
        .filter(|(_, tile_ids)| tile_ids.len() == 1)
        .map(|(_, tile_ids)| tile_ids.iter().next().unwrap())
        .collect();

    let corner_tiles: HashSet<_> = edge_tiles
        .iter()
        .cloned()
        .filter(|&edge_tile| {
            edge_tiles
                .iter()
                .cloned()
                .filter(|&tile| tile == edge_tile)
                .count()
                == 4 // two times (regular and flipped) per each edge
        })
        .cloned()
        .collect();

    corner_tiles.iter().product::<u64>().to_string()
}

pub fn day20b() -> String {
    use EdgeIndex::*;
    let (tiles, edges) = read_tiles();
    let pieces = place_image_pieces(&tiles, &edges);

    let mut image = assemble_image(pieces);

    image = image.transform(&Left, &Upper, true);

    println!("{}", image);

    let monster_pixels = monster_pixels_positions();
    let monsters_count = count_monsters(&mut image, &monster_pixels);

    let answer = image.0.values()
        .filter(|&value| *value).count() -
        monsters_count * monster_pixels.len();

    format!("{}", answer)
}

fn read_tiles() -> (HashMap<TileId, Tile>, HashMap<EdgeChecksum, Vec<TileId>>) {
    let input = std::fs::read_to_string("assets/day20.txt").unwrap();
    let tiles = parse_tiles(&input);
    let edges = parse_edges(tiles.values());
    (tiles, edges)
}

const TILE_SIZE: i32 = 10;
const CROPPED_TILE_SIZE: i32 = TILE_SIZE - 2;

type Pixel = bool;

#[derive(Copy, Clone)]
enum EdgeIndex {
    Upper = 0,
    Right = 1,
    Lower = 2,
    Left = 3,
}

impl Not for &EdgeIndex {
    type Output = EdgeIndex;

    fn not(self) -> Self::Output {
        use EdgeIndex::*;

        match self {
            Upper => Lower,
            Right => Left,
            Lower => Upper,
            Left => Right,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Vector2 {
    x: i32,
    y: i32,
}

type Position = Vector2;
type Direction = Vector2;

impl Add<Vector2> for Vector2 {
    type Output = Self;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<(i32, i32)> for Vector2 {
    fn from((x, y): (i32, i32)) -> Self {
        Vector2 { x, y }
    }
}

impl From<&EdgeIndex> for Direction {
    fn from(edge_index: &EdgeIndex) -> Self {
        use EdgeIndex::*;

        match edge_index {
            Upper => (0, -1).into(),
            Right => (1, 0).into(),
            Lower => (0, 1).into(),
            Left => (-1, 0).into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Image(HashMap<Position, Pixel>);

impl Image {
    fn normalize(&self) -> Image {
        let min_x = self.0.keys().map(|position| position.x).min().unwrap();
        let min_y = self.0.keys().map(|position| position.y).min().unwrap();

        Image(
            self.0
                .iter()
                .map(|(position, value)| (*position + (-min_x, -min_y).into(), *value))
                .collect(),
        )
    }

    fn transform(&self, source_index: &EdgeIndex, target_index: &EdgeIndex, flip: bool) -> Image {
        let source: Direction = source_index.into();
        let target: Direction = target_index.into();

        let cos = source.x * target.x + source.y * target.y;
        let sin = source.x * target.y - source.y * target.x;

        let mut pixels: HashMap<Position, Pixel> = self.0.iter()
            .map(|(position, value)| {
                (
                    (position.x * cos - position.y * sin, position.x * sin + position.y * cos).into(),
                    *value,
                )
            })
            .collect();

        use EdgeIndex::*;

        if flip {
            match &target_index {
                Upper | Lower => {
                    pixels = pixels
                        .iter()
                        .map(|(position, value)| ((-position.x, position.y).into(), *value))
                        .collect()
                }
                Right | Left => {
                    pixels = pixels
                        .iter()
                        .map(|(position, value)| ((position.x, -position.y).into(), *value))
                        .collect()
                }
            }
        }

        Image(pixels).normalize()
    }

    fn rotate_once(&self) -> Image {
        use EdgeIndex::*;
        self.transform(&Left, &Upper, false)
    }

    fn flip(&self) -> Image {
        use EdgeIndex::*;
        self.transform(&Upper, &Upper, true)
    }

    pub fn size(&self) -> usize {
        (self.0.len() as f64).sqrt() as usize
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let n = self.size() as i32;
        for y in 0..n {
            for x in 0..n {
                let pos = Vector2 { x, y};
                let pixel = match self.0.get(&pos) {
                    None => "X",
                    Some(true) => "#",
                    Some(false) => ".",
                };
                write!(f, "{}", pixel)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

type TileId = u64;
type EdgeChecksum = usize;

#[derive(Clone)]
struct Tile {
    id: TileId,
    image: Image,
    edges_checksums: [EdgeChecksum; 4],
    flipped_edges_checksums: [EdgeChecksum; 4],
}

impl Tile {
    fn transform(&self, source_index: &EdgeIndex, target_index: &EdgeIndex, flip: bool) -> Tile {
        let image = self.image.transform(source_index, target_index, flip);

        let mut edges_checksums = [0; 4];
        let mut flipped_edges_checksums = [0; 4];

        for i in 0..4 {
            edges_checksums[i] = self.edges_checksums
                [(4 + i + (*source_index) as usize - (*target_index) as usize) % 4];
            flipped_edges_checksums[i] = self.flipped_edges_checksums
                [(4 + i + (*source_index) as usize - (*target_index) as usize) % 4];
        }

        use EdgeIndex::*;

        if flip {
            let [upper, right, lower, left] = edges_checksums;
            let [upper_flipped, right_flipped, lower_flipped, left_flipped] =
                flipped_edges_checksums;

            match &target_index {
                Upper | Lower => {
                    edges_checksums = [upper_flipped, left_flipped, lower_flipped, right_flipped];
                    flipped_edges_checksums = [upper, left, lower, right];
                }
                Right | Left => {
                    edges_checksums = [lower_flipped, right_flipped, upper_flipped, left_flipped];
                    flipped_edges_checksums = [lower, right, upper, left];
                }
            }
        }

        Tile {
            id: self.id,
            image,
            edges_checksums,
            flipped_edges_checksums,
        }
    }
}

fn parse_tile(tile_str: &str) -> Tile {
    let id_str = tile_str.lines().next().unwrap();
    let id = id_str[5..id_str.len() - 1].parse::<TileId>().unwrap();

    let pixels: HashMap<Position, Pixel> = tile_str
        .lines()
        .skip(1)
        .enumerate()
        .map(move |(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32).into(), c == '#'))
        })
        .flatten()
        .collect();

    let edges_checksums = edges_checksums(&pixels);
    let flipped_edges_checksums = flipped_edges_checksums(&pixels);

    let pixels = pixels
        .into_iter()
        .filter(|(position, _)| {
            position.x != 0
                && position.y != 0
                && position.x != TILE_SIZE - 1
                && position.y != TILE_SIZE - 1
        })
        .collect();

    Tile {
        id,
        image: Image(pixels).normalize(),
        edges_checksums,
        flipped_edges_checksums,
    }
}

fn edges_checksums(pixels: &HashMap<Position, Pixel>) -> [EdgeChecksum; 4] {
    let upper_checksum = pixels
        .iter()
        .filter(|(&position, _)| position.y == 0)
        .map(|(position, &pixel)| (2_usize.pow(position.x as u32) * (pixel as EdgeChecksum)))
        .sum::<EdgeChecksum>();

    let right_checksum = pixels
        .iter()
        .filter(|(&position, _)| position.x == (TILE_SIZE - 1))
        .map(|(position, &pixel)| (2_usize.pow(position.y as u32) * (pixel as EdgeChecksum)))
        .sum::<EdgeChecksum>();

    let lower_checksum = pixels
        .iter()
        .filter(|(&position, _)| position.y == (TILE_SIZE - 1))
        .map(|(position, &pixel)| {
            2_usize.pow(((TILE_SIZE - 1) - position.x) as u32) * (pixel as EdgeChecksum)
        })
        .sum::<EdgeChecksum>();

    let left_checksum = pixels
        .iter()
        .filter(|(&position, _)| position.x == 0)
        .map(|(position, &pixel)| {
            2_usize.pow(((TILE_SIZE - 1) - position.y) as u32) * (pixel as EdgeChecksum)
        })
        .sum::<EdgeChecksum>();

    [
        upper_checksum,
        right_checksum,
        lower_checksum,
        left_checksum,
    ]
}

fn flipped_edges_checksums(pixels: &HashMap<Position, Pixel>) -> [EdgeChecksum; 4] {
    let upper_flipped_checksum = pixels
        .iter()
        .filter(|(&position, _)| position.y == 0)
        .map(|(position, &pixel)| {
            2_usize.pow(((TILE_SIZE - 1) - position.x) as u32) * (pixel as EdgeChecksum)
        })
        .sum::<EdgeChecksum>();

    let right_flipped_checksum = pixels
        .iter()
        .filter(|(&position, _)| position.x == (TILE_SIZE - 1))
        .map(|(position, &pixel)| {
            2_usize.pow(((TILE_SIZE - 1) - position.y) as u32) * (pixel as EdgeChecksum)
        })
        .sum::<EdgeChecksum>();

    let lower_flipped_checksum = pixels
        .iter()
        .filter(|(&position, _)| position.y == (TILE_SIZE - 1))
        .map(|(position, &pixel)| (2_usize.pow(position.x as u32) * (pixel as EdgeChecksum)))
        .sum::<EdgeChecksum>();

    let left_flipped_checksum = pixels
        .iter()
        .filter(|(&position, _)| position.x == 0)
        .map(|(position, &pixel)| (2_usize.pow(position.y as u32) * (pixel as EdgeChecksum)))
        .sum::<EdgeChecksum>();

    [
        upper_flipped_checksum,
        right_flipped_checksum,
        lower_flipped_checksum,
        left_flipped_checksum,
    ]
}

fn parse_tiles(input: &str) -> HashMap<TileId, Tile> {
    input.split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|tile_str| {
            let tile = parse_tile(tile_str);
            (tile.id, tile)
        })
        .collect()
}

fn parse_edges<'a>(tiles: impl Iterator<Item = &'a Tile>) -> HashMap<EdgeChecksum, Vec<TileId>> {
    let mut edges: HashMap<EdgeChecksum, Vec<TileId>> = HashMap::new();

    for tile in tiles {
        for edge in tile.edges_checksums.iter() {
            let edge_tiles = edges.entry(*edge).or_insert_with(Vec::new);
            edge_tiles.push(tile.id);
        }

        for edge in tile.flipped_edges_checksums.iter() {
            let edge_tiles = edges.entry(*edge).or_insert_with(Vec::new);
            edge_tiles.push(tile.id);
        }
    }

    edges
}

fn place_image_pieces(tiles: &HashMap<TileId, Tile>, edges: &HashMap<EdgeChecksum, Vec<TileId>>) -> HashMap<Position, Image> {
    use EdgeIndex::*;
    let first_tile = tiles.values().next().unwrap();
    let starting_position = (0, 0).into();

    let mut image_pieces: HashMap<Position, Image> =
        [(starting_position, first_tile.image.clone())]
            .iter()
            .cloned()
            .collect();

    let mut queue: VecDeque<(Position, Tile)> =
        VecDeque::from(vec![(starting_position, first_tile.clone())]);
    let mut visited: HashSet<Position> = [starting_position].iter().cloned().collect();

    while !queue.is_empty() {
        let (current_position, current_tile) = queue.pop_front().unwrap();

        for edge_index in &[Upper, Right, Lower, Left] {
            let position = current_position + edge_index.into();

            if !visited.contains(&position) {
                let edge_checksum = current_tile.edges_checksums[(*edge_index) as usize];

                for id in edges.get(&edge_checksum).unwrap() {
                    if *id != current_tile.id {
                        visited.insert(position);

                        let mut neighbor_tile = tiles.get(id).unwrap().clone();

                        for index in &[Upper, Right, Lower, Left] {
                            if neighbor_tile.flipped_edges_checksums[*index as usize]
                                == edge_checksum
                            {
                                neighbor_tile = neighbor_tile.transform(index, &!edge_index, false);
                                break;
                            } else if neighbor_tile.edges_checksums[*index as usize]
                                == edge_checksum
                            {
                                neighbor_tile = neighbor_tile.transform(index, &!edge_index, true);
                                break;
                            }
                        }

                        queue.push_back((position, neighbor_tile.clone()));
                        image_pieces.insert(position, neighbor_tile.image.clone());
                    }
                }
            }
        }
    }

    image_pieces
}

fn assemble_image(image_pieces: HashMap<Position, Image>) -> Image {
    let mut image = Image(HashMap::new());

    for (large_position, tile_image) in image_pieces.iter() {
        for x in 0..CROPPED_TILE_SIZE {
            for y in 0..CROPPED_TILE_SIZE {
                let pixel = tile_image.0.get(&(x as i32, y as i32).into()).unwrap();

                image.0.insert(
                    (
                        large_position.x * CROPPED_TILE_SIZE + (x - 1) as i32,
                        large_position.y * CROPPED_TILE_SIZE + (y - 1) as i32,
                    )
                        .into(),
                    *pixel,
                );
            }
        }
    }

    image.normalize()
}

fn monster_pixels_positions() -> Vec<Position> {
    const MONSTER_PATTERN: &str = r"                  #
#    ##    ##    ###
 #  #  #  #  #  #   ";

    MONSTER_PATTERN
        .lines()
        .enumerate()
        .map(move |(y, line)| {
            line.chars()
                .enumerate()
                .filter(move |(_, c)| *c == '#')
                .map(move |(x, _)| (x as i32, y as i32).into())
        })
        .flatten()
        .collect()
}

fn count_monsters(image: &mut Image, monster_pixels: &[Position]) -> usize {
    let max_x = image.0.keys().map(|position| position.x).max().unwrap();
    let max_y = image.0.keys().map(|position| position.y).max().unwrap();

    let monster_max_x = monster_pixels
        .iter()
        .map(|position| position.x)
        .max()
        .unwrap();
    let monster_max_y = monster_pixels
        .iter()
        .map(|position| position.y)
        .max()
        .unwrap();

    let mut monsters_count = 0;

    for attempt in 0..8 {
        for y in 0..=max_y - monster_max_y + 1 {
            'next_pixel: for x in 0..=max_x - monster_max_x + 1 {
                let current_position: Position = (x, y).into();

                for monster_position in monster_pixels.iter() {
                    match image.0.get(&(current_position + *monster_position)) {
                        None => {
                            println!("{}", image);
                            println!("Missing pixel - {:?}", current_position + *monster_position)
                        },
                        Some(false) => continue 'next_pixel,
                        Some(true) => {}
                    }
                }

                monsters_count += 1;
            }
        }

        if monsters_count > 0 {
            break;
        } else if attempt == 3 {
            *image = image.flip();
        } else {
            *image = image.rotate_once();
        }
    }

    monsters_count
}
