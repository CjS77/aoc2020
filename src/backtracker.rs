use std::collections::HashMap;
use std::hash::Hash;

pub struct BackTracker<S, I>
where
    S: Eq + Hash,
{
    states: HashMap<S, I>,
}

impl<S,I> BackTracker<S,I>
where
    S: State,
    I: Iterator<Item=S>
{
    pub fn solve(&mut self, state: &S) -> Option<S> {
        if let Some(steps) = self.states.get_mut(state) {
            // If there are no more possibilities, then there's no solution
            let next_state = steps.next()?;
            if !next_state.is_valid() {

            }
            if next_state.is_solved() {
                return Some(next_state)
            }
            return self.solve(next_state);
        }
        let possible_next_states = state.next_steps().into_iter();
        self.states.insert(state, possible_next_states);
        self.solve(state)
    }
}

pub trait State {
    fn is_valid(&self) -> bool;
    fn is_solved(&self) -> bool;
    fn next_steps<I: IntoIterator<Item=Self>>(&self) -> I;
}