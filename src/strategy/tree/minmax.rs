//! The minmax strategy for playing Mancala
//!
//! The [minmax strategy](https://en.wikipedia.org/wiki/Minimax) is a
//!
//! > decision rule used in artificial intelligence, decision theory, game theory, statistics and philosophy for minimizing the possible loss for a worst case (maximum loss) scenario. When dealing with gains, it is referred to as "maximin"—to maximize the minimum gain. Originally formulated for two-player zero-sum game theory, covering both the cases where players take alternate moves and those where they make simultaneous moves, it has also been extended to more complex games and to general decision-making in the presence of uncertainty.

use crate::game::{Bowl, Position};
use crate::strategy::Strategy;

/// Pick the option that maximizes the minimum win.
pub struct MinMax {
}

impl Strategy for MinMax {
    fn play(&mut self, _position: &Position) -> Option<Bowl> {
        None
    }
}
