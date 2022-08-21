//! The naive strategy to randomly pick an available option.
use super::super::Strategy;
use crate::game::{Bowl, Position};
use rand::{rngs::ThreadRng, seq::SliceRandom};

/// Pick a random option.
pub struct Random {
    rng: ThreadRng,
}

impl Random {
    /// Create a Random strategy
    pub fn new(rng: ThreadRng) -> Self {
        Self { rng }
    }
}

impl Strategy for Random {
    fn play(&mut self, position: &Position) -> Option<Bowl> {
        let options = position.options();
        options.choose(&mut self.rng).cloned()
    }
}
