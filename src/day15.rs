use std::collections::HashMap;

#[derive(Default)]
struct Game {
    // Number, last 2 turns spoken. or zero if never spoken
    values: HashMap<usize, (usize, usize)>,
    turn: usize,
    value: usize,
}

impl Game {
    pub fn play(&mut self, input: &[usize], turns: usize) -> usize {
        self.turn = 0;
        self.values = HashMap::new();
        (0..turns).for_each(|_| self.next_turn(input));
        self.last_value()
    }

    pub fn next_turn(&mut self, input: &[usize]) {
        // starting numbers
        self.turn += 1;
        if self.turn <= input.len() {
            self.value = input[self.turn - 1];
            assert!(self.values.insert(self.value, (self.turn, 0)).is_none());
            println!("*Turn: {}, Value: {}", self.turn, self.value);
            return;
        }
        self.next_value();
    }

    fn next_value(&mut self) {
        let turn = self.turn;
        let last_spoken = self.value;
        let v = *self.values.get(&last_spoken).unwrap();
        // If it wasn't spoken, say 0 next
        let mut next_val = 0usize;
        // If it has been said more than once, the next value is the difference between the turns it was last spoken
        // It will have been spoken more than one if both terms are non-zero
        if v.1 > 0 { next_val = v.0 - v.1; }
        self.values.entry(next_val).and_modify(|v| {
            *v = (turn, v.0)
        }).or_insert((turn, 0));
        self.value = next_val;
    }

    pub fn last_value(&self) -> usize {
        self.value
    }
}

pub fn day15a() -> String {
    let mut game = Game::default();
    let res = game.play(&[0, 3, 1, 6, 7, 5], 2020);
    format!("{}", res)
}

pub fn day15b() -> String {
    let mut game = Game::default();
    let res = game.play(&[0, 3, 1, 6, 7, 5], 30_000_000);
    format!("{}", res)
}

