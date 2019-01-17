//! The alpha-beta pruning strategy.
//!
//! The [alpha-beta pruning](https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning) is a
//!
//! > a search algorithm that seeks to decrease the number of nodes that are evaluated by the minimax algorithm in its search tree. It is an adversarial search algorithm used commonly for machine playing of two-player games (Tic-tac-toe, Chess, Go, etc.). It stops completely evaluating a move when at least one possibility has been found that proves the move to be worse than a previously examined move.

use std::cmp::max;
use crate::game::{Bowl, Position};
use crate::strategy::Strategy;
use super::Value;

/// Pick the option that maximizes the minimum win, pruning sub-trees along the way.
pub struct AlphaBeta {
}

impl Strategy for AlphaBeta {
    fn play(&mut self, position: &Position) -> Option<Bowl> {
        let (bowl, _) = alpha_beta(position, Value::NegativeInfinity, Value::PositiveInfinity);
        bowl
    }
}

fn alpha_beta(position: &Position, alpha_prime: Value, beta:Value) -> (Option<Bowl>, Value) {
    let mut alpha = alpha_prime;
    if position.finished() {
        (None, Value::Actual(position.score().expect("finished game to have a score")))
    } else {
        let mut best_bowl = None;
        let mut best_value = Value::NegativeInfinity;
        for bowl in position.options() {
            let candidate_position = position.play(bowl).expect("option to be playable");
            let value;
            if candidate_position.turn() == position.turn() {
                let tuple = alpha_beta(&candidate_position, alpha, beta);
                value = tuple.1;
            } else {
                let tuple = alpha_beta(&candidate_position, beta.opposite(), alpha.opposite());
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

#[cfg(test)]
mod tests {
    use crate::game::Position;
    use super::*;

    #[test]
    fn finished_games_are_scored() {
        let position = Position::from((5, 0, [0,0, 2, 2]));

        let (bowl, value) = alpha_beta(&position, Value::NegativeInfinity, Value::PositiveInfinity);

        assert_eq!(value, Value::Actual(1));
        assert_eq!(bowl, None);
    }

    #[test]
    fn only_bowl_is_selected() {
        let position = Position::from([1,0,1,0]);

        let result = alpha_beta(&position, Value::NegativeInfinity, Value::PositiveInfinity);

        assert_eq!(result, (Some(0), Value::Actual(2)));
    }

    #[test]
    fn best_bowl_is_selected() {
        let position = Position::from([1,2,1,0,2,1]);

        let (_, value) = alpha_beta(&position, Value::NegativeInfinity, Value::PositiveInfinity);

        assert_eq!(value, Value::Actual(5));
    }
}
