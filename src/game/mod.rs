//! Mancala Game related definitions
//!
//! [Mancala](https://en.wikipedia.org/wiki/Mancala) is a game with many variants.
//! Here we focus on one variant, but we allow different number of bowls.
//!
//! The code below shows how one would build a standard mancala game.
//!
//! ```rust
//! # use mancala::game::GameBuilder;
//! let game =
//!   GameBuilder::new()
//!     .bowls(6)
//!     .stones(4)
//!     .build();
//! ```

/// GameBuilder is used to create a Mancala game.
pub struct GameBuilder {
    bowls: u8,
    stones: u8,
}

impl GameBuilder {
    /// Creates a new GameBuilder
    ///
    /// The default number of bowls is 6 and the default number of stones per bowl is 4.
    pub fn new() -> Self {
        GameBuilder {
            bowls: 6,
            stones: 4,
        }
    }

    /// Sets the number of bowls for this GameBuilder
    pub fn bowls(self, bowls: u8) -> Self {
        GameBuilder { bowls, ..self }
    }

    /// Sets the number of stones for this GameBuilder
    pub fn stones(self, stones: u8) -> Self {
        GameBuilder { stones, ..self }
    }

    /// Creates a Game with the required number of bowls and stones per bowl
    pub fn build(self) -> Game {
        let current = Position::new(self.bowls, self.stones);
        Game {
            current,
            history: vec![],
        }
    }
}

/// Game is an sequence of Positions.
///
/// A Game is created with a GameBuilder.
pub struct Game {
    current: Position,
    history: Vec<usize>,
}

impl Game {
    /// Determine if this game is finished
    pub fn finished(&self) -> bool {
        false
    }

    /// Determine which bowls are playable.
    pub fn options(&self) -> Vec<usize> {
        self.current.options()
    }
}

/// Position is a instance of the board.
pub struct Position {
    size: usize,
    capture: [u8; 2],
    bowls: Vec<u8>,
}

impl Position {
    /// Create a position with a number of bowls and a number of stones per bowl.
    pub fn new(bowls: u8, stones: u8) -> Self {
        let size = bowls as usize;
        let bowls = vec![stones; size];
        Position {
            size,
            capture: [0, 0],
            bowls,
        }
    }

    /// Determine which bowls are playable.
    pub fn options(&self) -> Vec<usize> {
        let options = self.bowls[0..self.size]
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(index, stones)| if stones > 0 { Some(index) } else { None })
            .collect();
        return options;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_game_is_not_finished() {
        let game = GameBuilder::new().bowls(6).stones(4).build();

        assert!(!game.finished())
    }

    #[test]
    fn game_knows_options_to_play() {
        let game = GameBuilder::new().bowls(3).stones(2).build();

        let options = game.options();

        assert_eq!(options, vec!(0, 1, 2))
    }
}
