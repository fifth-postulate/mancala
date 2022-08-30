///! The naive strategy to pick the first option.
use super::super::Strategy;
use crate::game::{Bowl, Position};

/// Pick the first option.
pub struct First {}

impl First {
    /// Create a first strategy
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for First {
    fn default() -> Self {
        Self::new()
    }
}

impl Strategy for First {
    fn play(&mut self, position: &Position) -> Option<Bowl> {
        let options = position.options();
        options.first().cloned()
    }
}
