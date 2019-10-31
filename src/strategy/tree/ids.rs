//! The iterative deepening search strategy.
//!
//! The [ids search strategy](https://en.wikipedia.org/wiki/Iterative_deepening_depth-first_search) is a
//!
//! >  is a state space/graph search strategy in which a depth-limited version of depth-first search is run repeatedly with increasing depth limits until the goal is found.

use super::{Depth, Value, DepthLimitedSearch};
use crate::game::{Bowl, Position};
use crate::strategy::Strategy;

struct IterativeDeepeningSearch<S>
where
    S: DepthLimitedSearch<Position, (Option<Bowl>, Value)> + Sized,
{
    max_depth: Depth,
    searcher: S,
}

impl<S> Strategy for IterativeDeepeningSearch<S>
where
    S: DepthLimitedSearch<Position, (Option<Bowl>, Value)> + Sized,
{
    fn play(&mut self, position: &Position) -> Option<Bowl> {
        let (mut best_bowl, mut best_value) = (None, Value::NegativeInfinity);
        for current_depth in Depth::Limit(1).to(self.max_depth) {
            let (candidate_bowl, candidate_value) = self.searcher.search(&position, &current_depth); 
            if candidate_value > best_value {
                best_bowl = candidate_bowl;
                best_value = candidate_value;
            }
       }
       best_bowl
    }
}
