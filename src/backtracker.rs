type MoveIter<M> = Box<dyn Iterator<Item=M>>;

pub fn depth_first_iterator<S, M>(initial_state: S) -> Option<S>
    where
        S: State<Move=M>,
{
    let mut states: Vec<(S, _)> = Vec::new();
    let mut moves = initial_state.get_moves::<MoveIter<M>>();
    let mut current_state = initial_state;

    loop {
        match moves.next() {
            None => {
                // println!("Backtracking. {} states in stack", states.len());
                if states.is_empty() {
                    return None;  // There's no solution
                }
                let (cs, mvs) = states.pop().unwrap();
                current_state = cs;
                moves = Box::new(mvs);
            }
            Some(next_move) => {
                // println!("Forward. {} states in stack", states.len());
                let next_state = match current_state.apply(&next_move) {
                    None => continue, // next move
                    Some(s) => s,
                };
                if next_state.is_solved() {
                    return Some(next_state);
                }
                let last_state = std::mem::replace(&mut current_state, next_state);
                states.push((last_state, moves));
                moves = current_state.get_moves::<MoveIter<M>>();
            }
        }
    }
}

pub trait State: Eq + Sized {
    type Move;
    fn is_solved(&self) -> bool;
    fn get_moves<I>(&self) -> Box<dyn Iterator<Item=Self::Move>> where I: Iterator<Item=Self::Move>;
    fn apply(&self, next_move: &Self::Move) -> Option<Self>;
}

#[cfg(test)]
mod test {
    use crate::backtracker::{State, depth_first_iterator};
    use std::fmt;

    #[derive(Eq, PartialEq, Clone)]
    struct Board {
        squares: Vec<isize>,
        pos: usize,
        size: usize,
        turn: usize,
    }

    impl Board {
        // Panics if x0, y0 is not a valid position
        pub fn new(size: usize, x0: usize, y0: usize) -> Self {
            let mut squares = vec![-1; size * size];
            let pos = x0 + y0 * size;
            squares[pos] = 0;
            Board { squares, pos, size, turn: 0 }
        }
    }

    impl Default for Board {
        fn default() -> Self {
            Self::new(8, 0, 0)
        }
    }

    impl fmt::Display for Board {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for (i, moved) in self.squares.iter().enumerate() {
                if *moved == -1 { write!(f, "| XX ")? } else { write!(f, "| {:2} ", *moved)? };
                if (i + 1) % self.size == 0 { write!(f, "|\n")? }
            }
            Ok(())
        }
    }

    impl State for Board {
        type Move = (isize, isize);

        fn is_solved(&self) -> bool {
            self.squares.iter().all(|s| *s >= 0)
        }

        fn get_moves<I>(&self) -> Box<dyn Iterator<Item=Self::Move>> where I: Iterator<Item=Self::Move> {
            Box::new(vec![
                (2, 1),
                (2, -1),
                (-2, 1),
                (-2, -1),
                (1, 2),
                (1, -2),
                (-1, 2),
                (-1, -2),
            ].into_iter())
        }

        fn apply(&self, next_move: &Self::Move) -> Option<Self> {
            let mut x = (self.pos % self.size) as isize;
            let mut y = (self.pos / self.size) as isize;
            x += next_move.0;
            y += next_move.1;
            if x < 0 || y < 0 || x >= self.size as isize || y >= self.size as isize {
                return None;
            }
            let i = y as usize * self.size + x as usize;
            if self.squares[i] >= 0 {
                return None;
            }
            let mut next = self.clone();
            next.turn += 1;
            next.pos = i;
            next.squares[i] = next.turn as isize;
            Some(next)
        }
    }

    #[test]
    fn knight() {
        let k0 = Board::new(5, 0, 0);
        let result = depth_first_iterator(k0).unwrap();
        println!("{}", result);
        assert_eq!(&result.squares, &[0,13,18,7,24,5,8,1,12,17,14,19,6,23,2,9,4,21,16,11,20,15,10,3,22]);
    }
}