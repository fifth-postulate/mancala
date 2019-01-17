//! Various tree strategies for playing Mancala

pub mod minmax;
pub mod alphabeta;

use std::cmp::{Ord, PartialOrd, Ordering};
use crate::game::Score;
pub use self::minmax::MinMax;
pub use self::alphabeta::AlphaBeta;


#[derive(Debug, PartialEq, Eq)]
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
    pub fn opposite(&self) -> Self {
        match *self {
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
            Value::NegativeInfinity => {
                match *other {
                    Value::NegativeInfinity => Ordering::Equal,

                    _ => Ordering::Less,
                }
            },

            Value::Actual(self_score) => {
                match *other {
                    Value::NegativeInfinity => Ordering::Greater,

                    Value::Actual(other_score) => self_score.cmp(&other_score),

                    Value::PositiveInfinity => Ordering::Less,
                }
            },

            Value::PositiveInfinity => {
                match *other {
                    Value::PositiveInfinity => Ordering::Equal,

                    _ => Ordering::Greater,
                }
            }
        }
    }
}
