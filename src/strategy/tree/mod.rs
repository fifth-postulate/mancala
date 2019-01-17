//! Various tree strategies for playing Mancala

pub mod minmax;
pub mod alphabeta;

pub use self::minmax::MinMax;
pub use self::alphabeta::AlphaBeta;
