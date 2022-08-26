//! Estimate a value for a position.
//!
//! A [heuristic](https://en.wikipedia.org/wiki/Heuristic) is
//!
//! > any approach to problem solving or self-discovery that employs a practical method that is not guaranteed to be optimal, perfect, or rational, but is nevertheless sufficient for reaching an immediate, short-term goal or approximation.

use crate::game::{Position, Score};
use std::cmp::{Ord, Ordering, PartialOrd};

/// A way to evaluate a position without full knowledge of the game tree
pub trait Heuristic {
    /// Return the heuristic value for this position
    fn evaluate(&self, position: &Position) -> Value;
}

impl<F> Heuristic for F
where
    F: Fn(&Position) -> Value,
{
    fn evaluate(&self, position: &Position) -> Value {
        self(position)
    }
}

/// Positions can have a value `v`
///
/// It ranges from -∞ and ∞
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Value {
    /// Represent the lowest possible value for a position
    NegativeInfinity,
    /// Actual value or an estimate of the actual value of a position
    Actual(Score),
    /// Represent the highest possible value for a position
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
