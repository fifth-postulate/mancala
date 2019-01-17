//! The alpha-beta pruning strategy.
//!
//! The [alpha-beta pruning](https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning) is a
//!
//! > a search algorithm that seeks to decrease the number of nodes that are evaluated by the minimax algorithm in its search tree. It is an adversarial search algorithm used commonly for machine playing of two-player games (Tic-tac-toe, Chess, Go, etc.). It stops completely evaluating a move when at least one possibility has been found that proves the move to be worse than a previously examined move.

use crate::game::{Bowl, Position};
use crate::strategy::Strategy;

/// Pick the option that maximizes the minimum win, pruning sub-trees along the way.
pub struct AlphaBeta {
}

impl Strategy for AlphaBeta {
    fn play(&mut self, _position: &Position) -> Option<Bowl> {
        None
    }
}
