//! Various strategies for playing Mancala

use super::game::Position;

/// A strategy for playing Mancala
pub trait Strategy {
    /// Return the play for this position
    fn play(position: &Position) -> usize;
}
