//! Naive strategies, mainly for testing purposes.

use super::Strategy;
use crate::game::{Bowl, Position};

/// Pick the first option.
pub struct First {}

impl Strategy for First {
    fn play(&mut self, position: &Position) -> Option<Bowl> {
        let options = position.options();
        options.first().cloned()
    }
}
