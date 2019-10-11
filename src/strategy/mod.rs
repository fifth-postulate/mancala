//! Various strategies for playing Mancala

pub mod naive;
pub mod tree;
pub mod user;

pub use self::naive::First;
pub use self::tree::AlphaBeta;
pub use self::tree::MinMax;
pub use self::user::user;

use super::game::{Bowl, Position, Score};

/// A strategy for playing Mancala
pub trait Strategy {
    /// Return the play for this position
    fn play(&mut self, position: &Position) -> Option<Bowl>;
}
