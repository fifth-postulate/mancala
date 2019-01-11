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
#[derive(Debug, PartialEq)]
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

    /// Play a certain bowl.
    ///
    /// Fails if the bowl does not contain any stones.
    pub fn play(&mut self, bowl: usize) -> Result<(), FoulPlay> {
        match self.current.play(bowl) {
            Some(position) => {
                self.history.push(bowl);
                self.current = position;
                Ok(())
            }

            None => Err(FoulPlay::NoStonesInBowl),
        }
    }
}

/// Discriminates between all the ways a play can go wrong.
#[derive(Debug)]
pub enum FoulPlay {
    /// Playing a bowl when there are no stones in the bowl, is foul play.
    NoStonesInBowl,
}

/// Position is a instance of the board.
#[derive(Debug, PartialEq)]
pub struct Position {
    size: usize,
    capture: [u8; 2],
    bowls: Vec<u8>,
}

impl Position {
    /// Create a position with a number of bowls and a number of stones per bowl.
    pub fn new(bowls: u8, stones: u8) -> Self {
        let size = bowls as usize;
        let bowls = vec![stones; 2 * size];
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

    /// Play a certain bowl.
    ///
    /// If the bowl returns nothing.
    pub fn play(&self, bowl: usize) -> Option<Self> {
        if self.bowls[bowl] > 0 {
            Some(self.sow(bowl))
        } else {
            None
        }
    }

    fn sow(&self, bowl: usize) -> Self {
        let mut bowls = self.bowls.clone();
        let stones = bowls[bowl];
        bowls[bowl] = 0;
        for offset in 0..(stones as usize) {
            bowls[bowl + offset + 1] += 1;
        }
        bowls.rotate_left(self.size);
        // TODO correctly implement sow; capture, store
        Position {
            size: self.size,
            capture: self.capture,
            bowls,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from_position<P>(position: P) -> PlayedGameBuilder
    where
        P: Into<Position>,
    {
        PlayedGameBuilder {
            current: position.into(),
            history: vec![],
        }
    }

    struct PlayedGameBuilder {
        current: Position,
        history: Vec<usize>,
    }

    impl PlayedGameBuilder {
        fn with_history(self, history: Vec<usize>) -> Game {
            Game {
                current: self.current,
                history,
            }
        }
    }

    #[test]
    fn fresh_game_is_not_finished() {
        let game = GameBuilder::new().bowls(6).stones(4).build();

        assert!(!game.finished());
    }

    #[test]
    fn game_knows_options_to_play() {
        let game = GameBuilder::new().bowls(3).stones(2).build();

        let options = game.options();

        assert_eq!(options, vec!(0, 1, 2));
    }

    #[test]
    fn game_records_history_of_what_is_played() -> Result<(), FoulPlay> {
        let mut actual = GameBuilder::new().bowls(3).stones(2).build();

        actual.play(0)?;

        let position = [2, 2, 2, 0, 3, 3];
        let expected = from_position(position).with_history(vec![0]);
        assert_eq!(actual, expected);
        Ok(())
    }
}

macro_rules! position_from_array_for_sizes {
    ( $($n : expr),* ) => {
        $(
        impl From<[u8; $n]> for Position {
            fn from(bowls: [u8; $n]) -> Self {
                Position {
                    size: $n/2,
                    capture: [0, 0],
                    bowls: bowls.to_vec(),
                }
            }
        }
        )*
    }
}

position_from_array_for_sizes!(2, 4, 6);
