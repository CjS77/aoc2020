use crate::bits::read_data;

pub fn day17a() -> String {
    let data = read_data("assets/day17.txt");
    let cube = parse_data(data);
    let cube = (0..6).fold(cube, |mut cube, i| {
        println!("Round: {}. {} active", i, cube.count_active());
        match cube.simulate() {
            Some(c) => c,
            None => panic!("Size {} is insufficient", cube.size),
        }
    });
    format!("{}", cube.count_active())
}

pub fn day17b() -> String {
    let data = read_data("assets/day17.txt");

    format!("{}", 1)
}

#[derive(Clone)]
struct Cube {
    size: usize,
    pos_locs: Vec<bool>,
    neg_locs: Vec<bool>,
}

fn parse_data(data: Vec<String>) -> Cube {
    let mut cube = Cube::new(100);
    data.into_iter().enumerate()
        .for_each(|(y, s)| {
            s.chars().enumerate()
                .for_each(|(x, c)| {
                    if c == '#' {
                        cube.set_cube((x as isize,y as isize ,0), true);
                    }
                })
        });
    cube
}

impl Cube {
    pub fn new(size: usize) -> Self {
        let max = Self::coord_to_index(size, ((size-1) as isize, (size-1) as isize, (size-1) as isize)) as usize + 1;
        Cube {
            size,
            pos_locs: vec![false; max],
            neg_locs: vec![false; max],
        }
    }

    pub fn coord_to_index(size: usize, pos: (isize, isize, isize)) -> isize {
        let (x, y, z) = pos;
        let b = size as isize + 1;
        let c = ((size + 1) * (size + 1)) as isize;
        x + b * y + c * z
    }

    pub fn index_to_coord(size: usize, idx: isize) -> (isize, isize, isize) {
        let size = size as isize;
        let mut idx = idx;
        let x = idx % (size + 1);
        idx /= size + 1;
        let y = idx % (size + 1);
        idx /= size + 1;
        let z = idx;
        (x, y, z)
    }

    pub fn is_active(&self, index: isize) -> Option<bool> {
        if index < 0 {
            self.neg_locs.get(-index as usize).copied()
        } else {
            self.pos_locs.get(index as usize).copied()
        }
    }

    pub fn set_cube(&mut self, pos: (isize, isize, isize), val: bool) -> Option<()> {
        let index = Self::coord_to_index(self.size, pos);
        if index < 0 {
            let index = -index as usize;
            if index >= self.neg_locs.len() { return None; }
            self.neg_locs[index] = val;
        } else {
            let index = index as usize;
            if index >= self.pos_locs.len() { return None; }
            self.pos_locs[index] = val;
        }
        Some(())
    }

    pub fn count_active(&self) -> usize {
        let pc = self.pos_locs.iter().filter(|&p| *p).count();
        let nc = self.neg_locs.iter().filter(|&p| *p).count();
        pc + nc
    }

    pub fn count_neighbours(&self, pos: (isize, isize, isize)) -> Option<usize> {
        let this = Self::coord_to_index(self.size, pos);
        let (x, y, z) = pos;
        let mut count = 0usize;
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let index = Self::coord_to_index(self.size,(x + dx, y + dy, z + dz));
                    if index == this { continue; }
                    // ignore oob
                    if matches!(self.is_active(index), Some(true)) { count += 1; }
                }
            }
        }
        Some(count)
    }

    pub fn simulate(&mut self) -> Option<Self> {
        let mut next = Self::new(self.size);
        for i in 0..self.pos_locs.len() {
            let index = i as isize;
            self.apply_rules(&mut next, index)?;
            if i > 0 { self.apply_rules(&mut next, -index)? }
        }
        Some(next)
    }

    fn apply_rules(&mut self, next: &mut Cube, index: isize) -> Option<()> {
        let pos = Self::index_to_coord(self.size, index);
        let nc = self.count_neighbours(pos)?;
        // println!("{:?} - active: {}, neighbours: {}", pos, self.is_active(index)?, nc);
        let new_state = if self.is_active(index)? {
            nc == 2 || nc == 3
        } else {
            nc == 3
        };
        next.set_cube(pos, new_state)
    }
}