use crate::bits::read_data;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;

pub fn day22a() -> String {
    let mut game = read_decks(25);
    let winner = game.play();
    let score = Game::score(winner);
    format!("{}", score)
}

pub fn day22b() -> String {
    let mut game = read_decks(25);
    let winner = game.play_recursive();
    let score = Game::score(&game.hands[winner]);
    format!("{}", score)
}

pub fn read_decks(n: usize) -> Game {
    let data = read_data("assets/day22.txt");
    let pa = data[1..n + 1].iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let pb = data[n + 3..2 * n + 3].iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    Game::new(pa, pb)
}

pub struct Game {
    turn: usize,
    pub hands: [Vec<usize>; 2],
    prev_states: HashSet<u64>,
}

impl Game {
    pub fn new(pa: Vec<usize>, pb: Vec<usize>) -> Self {
        Game {
            turn: 0,
            hands: [pa, pb],
            prev_states: HashSet::new(),
        }
    }

    pub fn is_finished(&self) -> bool {
        self.hands[0].is_empty() || self.hands[1].is_empty()
    }

    pub fn draw(&mut self) -> (usize, usize) {
        (
            self.hands[0].remove(0),
            self.hands[1].remove(0),
        )
    }

    pub fn play(&mut self) -> &[usize] {
        while !self.is_finished() {
            let (a, b) = self.draw();
            self.turn += 1;
            if a > b {
                self.hands[0].push(a);
                self.hands[0].push(b);
                continue;
            }
            if a < b {
                self.hands[1].push(b);
                self.hands[1].push(a);
                continue;
            }
            unreachable!()
        }
        println!("Done after {} turns", self.turn);
        if self.hands[0].is_empty() {
            &self.hands[1]
        } else {
            &self.hands[0]
        }
    }

    fn sub_game(&self, a: usize, b: usize) -> Game {
        let pa = self.hands[0][0..a].to_vec();
        let pb = self.hands[1][0..b].to_vec();
        Game::new(pa, pb)
    }

    fn seen_before(&mut self) -> bool {
        let mut hasher = DefaultHasher::new();
        self.hands.hash(&mut hasher);
        let hash = hasher.finish();
        !self.prev_states.insert(hash)
    }

    pub fn play_recursive(&mut self) -> usize {
        while !self.is_finished() {
            if self.seen_before() {
                println!("Seen this hand before");
                return 0;
            }
            let (a, b) = self.draw();
            self.turn += 1;
            if self.hands[0].len() >= a && self.hands[1].len() >= b {
                let mut sub_game = self.sub_game(a, b);
                let winner = sub_game.play_recursive();
                if winner == 0 {
                    self.hands[0].push(a);
                    self.hands[0].push(b);
                } else {
                    self.hands[1].push(b);
                    self.hands[1].push(a);
                }
            } else if a > b {
                self.hands[0].push(a);
                self.hands[0].push(b);
            } else {
                self.hands[1].push(b);
                self.hands[1].push(a);
            }
        }
        // println!("Done after {} turns", self.turn);
        let winner = if self.hands[0].is_empty() {
            1
        } else {
            0
        };
        // println!("{:?}", self.hands[winner]);
        winner
    }

    pub fn score(hand: &[usize]) -> usize {
        hand.iter().rev()
            .enumerate()
            .fold(0usize, |score, (i, c)| {
                score + (i + 1) * *c
            })
    }
}