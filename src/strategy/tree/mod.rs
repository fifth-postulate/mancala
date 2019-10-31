//! Various tree strategies for playing Mancala

pub mod alphabeta;
pub mod ids;
pub mod minmax;

pub use self::alphabeta::AlphaBeta;
pub use self::minmax::MinMax;
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

/// Determine the search depth of tree algorithms
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Depth {
    /// No limit on the search depth
    Infinite,
    /// Limit the search depth
    Limit(usize),
}

impl Depth {
    /// Determine if we can go any deeper.
    pub fn is_zero(&self) -> bool {
        match self {
            Depth::Infinite => false,

            Depth::Limit(depth) => *depth == 0usize,
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

    /// Return succeding depth.
    pub fn increment(&self) -> Self {
        match self {
            Depth::Infinite => Depth::Infinite,

            Depth::Limit(depth) => Depth::Limit(depth + 1),
        }
    }

    /// Return an interator that will visit every depth between self and limit (inclusive).
    pub fn to(&self, limit: Depth) -> DepthIterator {
        DepthIterator {
            current: *self,
            limit,
        }
    }
}

/// Iterator that iterates over all Depth values
pub struct DepthIterator {
    current: Depth,
    limit: Depth,
}

impl Iterator for DepthIterator {
    type Item = Depth;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.limit {
            let current = self.current;
            self.current = current.increment();
            Some(current)
        } else {
            None
        }
    }
}

/// A search strategy that can be limited by depth
pub trait DepthLimitedSearch<I, O> {
    /// Search up to `search_depth` levels
    fn search(&mut self, start: &I, search_depth: &Depth) -> Option<O>;
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
