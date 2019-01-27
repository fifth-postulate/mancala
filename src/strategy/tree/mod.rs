//! Various tree strategies for playing Mancala

pub mod alphabeta;
pub mod minmax;

pub use self::alphabeta::AlphaBeta;
pub use self::minmax::MinMax;
use crate::game::Score;
use std::cmp::{Ord, Ordering, PartialOrd};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Value {
    NegativeInfinity,
    Actual(Score),
    PositiveInfinity,
}

impl Value {
    /// Determine the opposite of a Value
    /// -∞ → ∞
    ///  s → -s
    ///  ∞ → -∞
    pub fn opposite(self) -> Self {
        match self {
            Value::NegativeInfinity => Value::PositiveInfinity,
            Value::Actual(score) => Value::Actual(-score),
            Value::PositiveInfinity => Value::NegativeInfinity,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match *self {
            Value::NegativeInfinity => match *other {
                Value::NegativeInfinity => Ordering::Equal,

                _ => Ordering::Less,
            },

            Value::Actual(self_score) => match *other {
                Value::NegativeInfinity => Ordering::Greater,

                Value::Actual(other_score) => self_score.cmp(&other_score),

                Value::PositiveInfinity => Ordering::Less,
            },

            Value::PositiveInfinity => match *other {
                Value::PositiveInfinity => Ordering::Equal,

                _ => Ordering::Greater,
            },
        }
    }
}

/// Determine the search depth of tree algorithms
pub enum Depth {
    /// No limit on the search depth
    Infinite,
    /// Limit the search depth
    Limit(u8),
}

impl Depth {
    /// Determine if we can go any deeper.
    pub fn is_zero(&self) -> bool {
        match self {
            Depth::Infinite => false,

            Depth::Limit(depth) => *depth == 0u8,
        }
    }

    /// Return preceding depth.
    pub fn decrement(&self) -> Self {
        match self {
            Depth::Infinite => Depth::Infinite,

            Depth::Limit(depth) => {
                if *depth == 0 {
                    Depth::Limit(0)
                } else {
                    Depth::Limit(depth - 1)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_compare_correctly() {
        assert!(Value::NegativeInfinity < Value::Actual(0));
        assert!(Value::NegativeInfinity < Value::PositiveInfinity);
        assert!(Value::Actual(0) > Value::NegativeInfinity);
        assert!(Value::Actual(0) < Value::PositiveInfinity);
        assert!(Value::PositiveInfinity > Value::NegativeInfinity);
        assert!(Value::PositiveInfinity > Value::Actual(0));
    }
}
