//! The iterative deepening search strategy.
//!
//! The [ids search strategy](https://en.wikipedia.org/wiki/Iterative_deepening_depth-first_search) is a
//!
//! >  is a state space/graph search strategy in which a depth-limited version of depth-first search is run repeatedly with increasing depth limits until the goal is found.

use crate::strategy::Strategy;
use super::{Depth, DepthLimitedSearch};
use crate::game::{Bowl, Position};

struct IterativeDeepeningSearch<S>
where
    S: DepthLimitedSearch<Position, Bowl> + Sized,
{
    max_depth: Depth,
    searcher: S,
}

impl<S> Strategy for IterativeDeepeningSearch<S>
where
    S: DepthLimitedSearch<Position, Bowl> + Sized
{
    fn play(&mut self, position: &Position) -> Option<Bowl> {
        let mut best_bowl = None;
        for current_depth in Depth::Limit(1).to(self.max_depth) {
            let candidate = self.searcher.search(&position, &current_depth);
            best_bowl = match best_bowl {
                Some(bowl) => {
                    Some(bowl)
                },
                None => candidate,
            }
        }
        best_bowl
   }
}