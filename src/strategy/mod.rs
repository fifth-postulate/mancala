//! Various strategies for playing Mancala

pub mod naive;
pub mod tree;

pub use self::naive::First;
pub use self::tree::minmax;

use super::game::{Bowl, Position};

/// A strategy for playing Mancala
pub trait Strategy {
    /// Return the play for this position
    fn play(&mut self, position: &Position) -> Option<Bowl>;
}
