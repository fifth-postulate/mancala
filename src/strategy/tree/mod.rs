//! Various tree strategies for playing Mancala

pub mod alphabeta;
pub mod ids;
pub mod mcts;
pub mod minmax;

pub use self::alphabeta::AlphaBeta;
pub use self::mcts::MonteCarlo;
pub use self::minmax::MinMax;
use std::cmp::PartialOrd;

/// Determine the search depth of tree algorithms
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
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
    fn search(&mut self, start: &I, search_depth: &Depth) -> O;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn depth_iterator_should_contain_all_intermediate_values() {
        let expected = vec![Depth::Limit(1), Depth::Limit(2), Depth::Limit(3)];

        let actual: Vec<Depth> = Depth::Limit(1).to(Depth::Limit(3)).collect();

        assert_eq!(actual, expected);
    }
}
