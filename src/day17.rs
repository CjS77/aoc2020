use crate::bits::read_data;

pub fn day17a() -> String {
    for i in -1000..=1000 {
        let pos = Cube::index_to_coord(20, i);
        let index = Cube::coord_to_index(20, pos);
        assert_eq!(i, index, "index {}, pos {:?}", index, pos);
    }
    "Ok".to_string()
}

pub fn day17b() -> String {
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

#[derive(Clone)]
struct Cube {
    size: usize,
    pos_locs: Vec<bool>,
    neg_locs: Vec<bool>,
}

fn parse_data(data: Vec<String>) -> Cube {
    let mut cube = Cube::new(30);
    data.into_iter().enumerate()
        .for_each(|(y, s)| {
            s.chars().enumerate()
                .for_each(|(x, c)| {
                    if c == '#' {
                        cube.set_cube((x as isize, y as isize, 0, 0), true);
                    }
                })
        });
    cube
}

impl Cube {
    pub fn new(size: usize) -> Self {
        let max_i = size as isize + 1;
        let max = Self::coord_to_index(size, (max_i, max_i, max_i, max_i)) as usize + 1;
        Cube {
            size,
            pos_locs: vec![false; max],
            neg_locs: vec![false; max],
        }
    }

    #[allow(clippy::many_single_char_names)]
    pub fn coord_to_index(size: usize, pos: (isize, isize, isize, isize)) -> isize {
        let (x, y, z, w) = pos;
        let b = size as isize + 1;
        let c = b * b;
        let d = c * b;
        x + b * y + c * z + d * w
    }

    pub fn index_to_coord(size: usize, idx: isize) -> (isize, isize, isize, isize) {
        let n = (size + 1) as isize;
        let mut idx = idx;
        let x = idx % n;
        idx /= n;
        let y = idx % n;
        idx /= n;
        let z = idx % n;
        idx /= n;
        (x, y, z, idx)
    }

    pub fn is_active(&self, index: isize) -> Option<bool> {
        if index < 0 {
            self.neg_locs.get(-index as usize).copied()
        } else {
            self.pos_locs.get(index as usize).copied()
        }
    }

    pub fn set_cube(&mut self, pos: (isize, isize, isize, isize), val: bool) -> Option<()> {
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

    pub fn count_neighbours(&self, pos: (isize, isize, isize, isize)) -> Option<usize> {
        let this = Self::coord_to_index(self.size, pos);
        let (x, y, z, w) = pos;
        let mut count = 0usize;
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    for dw in -1..=1 {
                        let index = Self::coord_to_index(self.size, (x + dx, y + dy, z + dz, w + dw));
                        if index == this { continue; }
                        // ignore oob
                        if matches!(self.is_active(index), Some(true)) { count += 1; }
                    }
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
        // println!("idx: {}, {:?} - active: {}, neighbours: {}", index, pos, self.is_active(index)?, nc);
        let new_state = if self.is_active(index)? {
            nc == 2 || nc == 3
        } else {
            nc == 3
        };
        next.set_cube(pos, new_state)
    }
}