//! The minmax strategy for playing Mancala
//!
//! The [minmax strategy](https://en.wikipedia.org/wiki/Minimax) is a
//!
//! > decision rule used in artificial intelligence, decision theory, game theory, statistics and philosophy for minimizing the possible loss for a worst case (maximum loss) scenario. When dealing with gains, it is referred to as "maximin"—to maximize the minimum gain. Originally formulated for two-player zero-sum game theory, covering both the cases where players take alternate moves and those where they make simultaneous moves, it has also been extended to more complex games and to general decision-making in the presence of uncertainty.

use super::Value;
use crate::game::{Bowl, Position};
use crate::strategy::Strategy;

/// Pick the option that maximizes the minimum win.
pub struct MinMax {}

impl Strategy for MinMax {
    fn play(&mut self, position: &Position) -> Option<Bowl> {
        let (bowl, _) = minmax(&position);
        bowl
    }
}

fn minmax(position: &Position) -> (Option<Bowl>, Value) {
    if position.finished() {
        (
            None,
            Value::Actual(position.score().expect("finished game to have a score")),
        )
    } else {
        let mut best_bowl = None;
        let mut best_value = Value::NegativeInfinity;
        for bowl in position.options() {
            let candidate_position = position.play(bowl).expect("option to be playable");
            let (_, mut value) = minmax(&candidate_position);
            if candidate_position.turn() != position.turn() {
                value = value.opposite();
            }
            if value > best_value {
                best_bowl = Some(bowl);
                best_value = value;
            }
        }
        (best_bowl, best_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Position;

    #[test]
    fn finished_games_are_scored() {
        let position = Position::from((5, 0, [0, 0, 2, 2]));

        let (bowl, value) = minmax(&position);

        assert_eq!(value, Value::Actual(1));
        assert_eq!(bowl, None);
    }

    #[test]
    fn only_bowl_is_selected() {
        let position = Position::from([1, 0, 1, 0]);

        let result = minmax(&position);

        assert_eq!(result, (Some(0), Value::Actual(2)));
    }

    #[test]
    fn best_bowl_is_selected() {
        let position = Position::from([1, 2, 1, 0, 2, 1]);

        let (_, value) = minmax(&position);

        assert_eq!(value, Value::Actual(5));
    }
}
