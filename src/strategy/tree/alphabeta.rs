//! The alpha-beta pruning strategy.
//!
//! The [alpha-beta pruning](https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning) is a
//!
//! > a search algorithm that seeks to decrease the number of nodes that are evaluated by the minimax algorithm in its search tree. It is an adversarial search algorithm used commonly for machine playing of two-player games (Tic-tac-toe, Chess, Go, etc.). It stops completely evaluating a move when at least one possibility has been found that proves the move to be worse than a previously examined move.

use super::{Value, Depth, Heuristic};
use crate::game::{Bowl, Position};
use crate::strategy::Strategy;
use std::cmp::max;

/// Pick the option that maximizes the minimum win, pruning sub-trees along the way.
pub struct AlphaBeta<H> where H : Heuristic + Sized {
    search_depth: Depth,
    heuristic: H,
}

impl AlphaBeta<Delta> {
    /// create an alpha beta strategy with an infinite search depth
    pub fn new() -> Self {
        AlphaBeta { search_depth: Depth::Infinite, heuristic: Delta {} }
    }

    /// create an alpha beta strategy with a limited search depth
    pub fn limited_to(search_depth: Depth) -> Self {
        AlphaBeta { search_depth, heuristic: Delta {} }
    }
}

impl <H> Strategy for AlphaBeta<H> where H : Heuristic + Sized {
    fn play(&mut self, position: &Position) -> Option<Bowl> {
        let (bowl, _) = alpha_beta(position, Value::NegativeInfinity, Value::PositiveInfinity, &self.search_depth, &self.heuristic);
        bowl
    }
}

fn alpha_beta(position: &Position, alpha_prime: Value, beta: Value, search_depth: &Depth, heuristic: &Heuristic) -> (Option<Bowl>, Value) {
    let mut alpha = alpha_prime;
    if position.finished() || search_depth.is_zero() {
        if position.finished() {
            (
                None,
                Value::Actual(position.score().expect("finished game to have a score")),
            )
        } else {
            (
                None,
                heuristic.evaluate(position),
            )
        }
    } else {
        let mut best_bowl = None;
        let mut best_value = Value::NegativeInfinity;
        for bowl in position.options() {
            let candidate_position = position.play(bowl).expect("option to be playable");
            let value;
            if candidate_position.turn() == position.turn() {
                let tuple = alpha_beta(&candidate_position, alpha, beta, &search_depth.decrement(), heuristic);
                value = tuple.1;
            } else {
                let tuple = alpha_beta(&candidate_position, beta.opposite(), alpha.opposite(), &search_depth.decrement(), heuristic);
                value = tuple.1.opposite()
            }
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

// A simple heuristic that looks at the difference between the captured stones.
pub struct Delta {}

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

        let (bowl, value) = alpha_beta(&position, Value::NegativeInfinity, Value::PositiveInfinity, &Depth::Infinite, &heuristic);

        assert_eq!(value, Value::Actual(1));
        assert_eq!(bowl, None);
    }

    #[test]
    fn only_bowl_is_selected() {
        let position = Position::from([1, 0, 1, 0]);
        let heuristic = Delta {};

        let result = alpha_beta(&position, Value::NegativeInfinity, Value::PositiveInfinity, &Depth::Infinite, &heuristic);

        assert_eq!(result, (Some(0), Value::Actual(2)));
    }

    #[test]
    fn best_bowl_is_selected() {
        let position = Position::from([1, 2, 1, 0, 2, 1]);
        let heuristic = Delta {};


        let (_, value) = alpha_beta(&position, Value::NegativeInfinity, Value::PositiveInfinity, &Depth::Infinite, &heuristic);

        assert_eq!(value, Value::Actual(5));
    }
}
