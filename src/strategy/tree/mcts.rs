//! The Monte Carlo tree search strategy.
//!
//! [Monte Carlo tree search](https://en.wikipedia.org/wiki/Monte_Carlo_tree_search) is
//!
//! > a heuristic search algorithm for some kinds of decision processes, most notably those employed in software that plays board games. In that context MCTS is used to solve the game tree.
//!
//! The way to create a `MonteCarlo` strategy is
//!
//! ```
//! use mancala::strategy::tree::{MonteCarlo, Depth};
//! use mancala::strategy::tree::alphabeta::delta;
//!
//! let strategy = MonteCarlo::strategy().limited_to(Depth::Limit(5)).with_heuristic(delta()).build();
//! ```

use super::{Depth, Heuristic, Value};
use crate::game::{Bowl, Position};
use crate::strategy::tree::DepthLimitedSearch;
use crate::strategy::Strategy;
use std::cmp::max;

/// Build MonteCarlo strategy instances
pub struct MonteCarloBuilder<H>
where
    H: Heuristic + Sized,
{
    search_depth: Depth,
    heuristic: H,
}

impl<H> MonteCarloBuilder<H>
where
    H: Heuristic + Sized,
{
    /// Build an Alpha Beta strategy
    pub fn build(self) -> MonteCarlo<H> {
        MonteCarlo {
            search_depth: self.search_depth,
            heuristic: self.heuristic,
        }
    }

    /// limited to a certain search depth
    pub fn limited_to(mut self, search_depth: Depth) -> Self {
        self.search_depth = search_depth;
        self
    }

    /// with a certain heuristic
    pub fn with_heuristic<H_>(self, heuristic: H_) -> MonteCarloBuilder<H_>
    where
        H_: Heuristic + Sized,
    {
        MonteCarloBuilder {
            search_depth: self.search_depth,
            heuristic,
        }
    }
}

/// Pick the option that maximizes wins after random rollouts.
pub struct MonteCarlo<H>
where
    H: Heuristic + Sized,
{
    search_depth: Depth,
    heuristic: H,
}

impl MonteCarlo<Delta> {
    /// Create a default AlphaBetaBuilder
    ///
    /// It has an unlimited search depth and the Delta heuristic.
    pub fn strategy() -> MonteCarloBuilder<Delta> {
        MonteCarloBuilder {
            search_depth: Depth::Infinite,
            heuristic: delta(),
        }
    }
}

impl<H> Strategy for MonteCarlo<H>
where
    H: Heuristic + Sized,
{
    fn play(&mut self, position: &Position) -> Option<Bowl> {
        let search_depth = self.search_depth;
        let (bowl, _) = self.search(position, &search_depth);
        bowl
    }
}

impl<H> DepthLimitedSearch<Position, (Option<Bowl>, Value)> for MonteCarlo<H>
where
    H: Heuristic + Sized,
{
    fn search(&mut self, position: &Position, search_depth: &Depth) -> (Option<Bowl>, Value) {
        alpha_beta(
            position,
            Value::NegativeInfinity,
            Value::PositiveInfinity,
            search_depth,
            &self.heuristic,
        )
    }
}

fn alpha_beta(
    position: &Position,
    alpha_prime: Value,
    beta: Value,
    search_depth: &Depth,
    heuristic: &dyn Heuristic,
) -> (Option<Bowl>, Value) {
    let mut alpha = alpha_prime;
    if position.finished() || search_depth.is_zero() {
        if position.finished() {
            (
                None,
                Value::Actual(position.score().expect("finished game to have a score")),
            )
        } else {
            (None, heuristic.evaluate(position))
        }
    } else {
        let mut best_bowl = None;
        let mut best_value = Value::NegativeInfinity;
        for bowl in position.options() {
            let candidate_position = position.play(bowl).expect("option to be playable");
            let value = if candidate_position.turn() == position.turn() {
                let tuple = alpha_beta(
                    &candidate_position,
                    alpha,
                    beta,
                    &search_depth.decrement(),
                    heuristic,
                );
                tuple.1
            } else {
                let tuple = alpha_beta(
                    &candidate_position,
                    beta.opposite(),
                    alpha.opposite(),
                    &search_depth.decrement(),
                    heuristic,
                );
                tuple.1.opposite()
            };
            if value > best_value {
                best_bowl = Some(bowl);
                best_value = value;
            }
            alpha = max(alpha, value);
            if alpha >= beta {
                break;
            }
        }
        (best_bowl, best_value)
    }
}

/// A simple heuristic that looks at the difference between the captured stones.
pub struct Delta {}

/// create a delta heuristic
pub fn delta() -> Delta {
    Delta {}
}

impl Heuristic for Delta {
    fn evaluate(&self, position: &Position) -> Value {
        Value::Actual(position.delta())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Position;

    #[test]
    fn finished_games_are_scored() {
        let position = Position::from((5, 0, [0, 0, 2, 2]));
        let heuristic = Delta {};

        let (bowl, value) = alpha_beta(
            &position,
            Value::NegativeInfinity,
            Value::PositiveInfinity,
            &Depth::Infinite,
            &heuristic,
        );

        assert_eq!(value, Value::Actual(1));
        assert_eq!(bowl, None);
    }

    #[test]
    fn only_bowl_is_selected() {
        let position = Position::from([1, 0, 1, 0]);
        let heuristic = Delta {};

        let result = alpha_beta(
            &position,
            Value::NegativeInfinity,
            Value::PositiveInfinity,
            &Depth::Infinite,
            &heuristic,
        );

        assert_eq!(result, (Some(0), Value::Actual(2)));
    }

    #[test]
    fn best_bowl_is_selected() {
        let position = Position::from([1, 2, 1, 0, 2, 1]);
        let heuristic = Delta {};

        let (_, value) = alpha_beta(
            &position,
            Value::NegativeInfinity,
            Value::PositiveInfinity,
            &Depth::Infinite,
            &heuristic,
        );

        assert_eq!(value, Value::Actual(5));
    }
}
