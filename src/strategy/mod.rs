//! Various strategies for playing Mancala

pub mod naive;

use super::game::{Position, Bowl};

/// A strategy for playing Mancala
pub trait Strategy {
    /// Return the play for this position
    fn play(position: &Position) -> Option<Bowl>;
}

