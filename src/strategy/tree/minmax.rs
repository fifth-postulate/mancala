//! The minmax strategy for playing Mancala
//!
//! The [minmax strategy](https://en.wikipedia.org/wiki/Minimax) is a
//!
//! > decision rule used in artificial intelligence, decision theory, game theory, statistics and philosophy for minimizing the possible loss for a worst case (maximum loss) scenario. When dealing with gains, it is referred to as "maximin"—to maximize the minimum gain. Originally formulated for two-player zero-sum game theory, covering both the cases where players take alternate moves and those where they make simultaneous moves, it has also been extended to more complex games and to general decision-making in the presence of uncertainty.

use crate::{
    game::{Bowl, Position},
    strategy::{heuristic::Value, Strategy},
};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

/// Pick the option that maximizes the minimum win.
pub struct MinMax {
    /// An Analyzer that keeps track of various statistics.
    pub analyzer: Analyzer,
}

impl MinMax {
    /// Create a default MinMax strategy
    pub fn new() -> Self {
        Self {
            analyzer: Analyzer::new(),
        }
    }
}

impl Default for MinMax {
    fn default() -> Self {
        Self::new()
    }
}

impl Strategy for MinMax {
    fn play(&mut self, position: &Position) -> Option<Bowl> {
        let (bowl, _) = minmax(&mut self.analyzer, position);
        bowl
    }
}

fn minmax(analyzer: &mut Analyzer, position: &Position) -> (Option<Bowl>, Value) {
    analyzer.count();
    if position.finished() {
        (
            None,
            Value::Actual(position.score().expect("finished game to have a score")),
        )
    } else {
        let (mut best_bowl, mut best_value) = (None, Value::NegativeInfinity);
        for bowl in position.options() {
            let candidate_position = position.play(bowl).expect("option to be playable");
            analyzer.increment_depth();
            let (_, mut value) = minmax(analyzer, &candidate_position);
            analyzer.decrement_depth();
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

/// Analyzes game trees
pub struct Analyzer {
    depth_counter: HashMap<u64, u64>,
    current_depth: u64,
}

impl Analyzer {
    /// Create an analyzer with the node count set to zero.i128
    pub fn new() -> Self {
        Self {
            depth_counter: HashMap::new(),
            current_depth: 0,
        }
    }

    fn count(&mut self) {
        let count = if self.depth_counter.contains_key(&self.current_depth) {
            self.depth_counter.get(&self.current_depth).unwrap()
        } else {
            &0u64
        };
        self.depth_counter.insert(self.current_depth, count + 1);
    }

    fn increment_depth(&mut self) {
        self.current_depth += 1;
    }

    fn decrement_depth(&mut self) {
        self.current_depth -= 1;
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Analyzer {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let max_depth = self.depth_counter.keys().max().unwrap();
        let node_count: u64 = self.depth_counter.values().sum();
        write!(formatter, "nodes: {} depth: {}", node_count, max_depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Position;

    #[test]
    fn finished_games_are_scored() {
        let position = Position::from((5, 0, [0, 0, 2, 2]));
        let mut analyzer = Analyzer::new();

        let (bowl, value) = minmax(&mut analyzer, &position);

        assert_eq!(value, Value::Actual(1));
        assert_eq!(bowl, None);
    }

    #[test]
    fn only_bowl_is_selected() {
        let position = Position::from([1, 0, 1, 0]);
        let mut analyzer = Analyzer::new();

        let result = minmax(&mut analyzer, &position);

        assert_eq!(result, (Some(0), Value::Actual(2)));
    }

    #[test]
    fn best_bowl_is_selected() {
        let position = Position::from([1, 2, 1, 0, 2, 1]);
        let mut analyzer = Analyzer::new();

        let (_, value) = minmax(&mut analyzer, &position);

        assert_eq!(value, Value::Actual(5));
    }
}
