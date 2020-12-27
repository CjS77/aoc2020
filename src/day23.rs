use std::collections::LinkedList;

pub fn day23a() -> String {
    let mut cups = Cups::new("685974213");
    for _ in 0..100 {
        cups.turn();
        // println!("{}", cups.to_string())
    }
    format!("{}", cups.ans())
}

pub fn day23b() -> String {
    format!("{}", 1)
}

struct Cups {
    ring: Vec<u8>,
}

impl Cups {
    pub fn new(s: &str) -> Self {
        let ring = s.chars().map(|c| c as u8 - 48).collect::<Vec<u8>>();
        Self { ring }
    }

    pub fn turn(&mut self) {
        let current = self.ring[0];
        let excluded = &self.ring[1..=3];
        let rem = &self.ring[4..];
        let mut destination = if current == 1 { 9 } else { current - 1 };
        while excluded.contains(&destination) {
            destination -= 1;
            if destination == 0 { destination = 9; }
        }
        let index = rem.iter().enumerate().find(|(_i, &v)| v == destination).unwrap().0 + 1;
        self.ring[1..=index + 3].rotate_left(3);
        self.ring.rotate_left(1);
    }

    pub fn ans(&self) -> String {
        let mut v = self.ring.clone();
        let index = v.iter().enumerate().find(|(_i, &val)| val == 1).unwrap().0;
        if index > 0 { v.rotate_left(index - 1); }
        v[1..].iter().map(|c| (c + 48) as char).collect()
    }
}

const SIZE: usize = 1_000_000;

struct BigCups {
    ring: Vec<usize>,
    current_index: usize,
}

impl BigCups {
    pub fn new(s: &str) -> Self {
        let mut ring = s.chars().map(|c| c as usize - 48).collect::<Vec<usize>>();
        for v in 10..=SIZE {
            ring.push(v)
        }
        Self { ring, current_index: 0 }
    }

    pub fn turn(&mut self) {
        let ci = self.current_index;
        let current = self.ring[ci];
        let excluded = &[self.ring[(ci + 1) % SIZE], self.ring[(ci + 2) % SIZE], self.ring[(ci + 3) % SIZE]];

        let mut destination = if current == 1 { SIZE } else { current - 1 };
        while excluded.contains(&destination) {
            destination -= 1;
            if destination == 0 { destination = SIZE; }
        }
        let di = self.ring.iter().enumerate().find(|(_i, &v)| v == destination).unwrap().0;
        let mut next_current = ci + 1;
        if di > ci {
            self.ring[ci + 1..=di].rotate_left(3);
        } else if ci == SIZE - 1 {
            self.ring[0..=di].rotate_left(3);
            next_current = 0;
        } else if ci > SIZE - 4 {
            let n = SIZE - ci - 1;
            next_current = 0;
            self.ring[di + 1..SIZE].rotate_right(n);
            self.ring[0..=di + n].rotate_left(3 - n);
        } else {
            self.ring[di + 1..=ci + 3].rotate_right(3);
            next_current = (ci + 4) % SIZE;
        }
        self.current_index = next_current;
    }

    pub fn ans(self) -> usize {
        let v = self.ring;
        let index = v.iter().enumerate().find(|(_i, &val)| val == 1).unwrap().0;
        let (a, b, c) = (v[index % SIZE], v[(index + 1) % SIZE], v[(index + 2) % SIZE]);
        println!("{} {} {}", a, b, c);
        b * c
    }
}

pub fn new_list(s: &str) -> LinkedList<usize> {
    let mut list = LinkedList::new();
    for c in s.chars() {
        let v = c as usize - 48;
        list.push_back(v);
    }
    for v in 10..=SIZE {
        list.push_back(v)
    }
    list
}

