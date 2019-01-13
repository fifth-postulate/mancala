//! Naive strategies, mainly for testing purposes.

use super::super::game::{Bowl, Position};
use super::Strategy;

/// Pick the first option.
pub struct First {}

impl Strategy for First {
    fn play(&mut self, position: &Position) -> Option<Bowl> {
        let options = position.options();
        options.first().cloned()
    }
}
