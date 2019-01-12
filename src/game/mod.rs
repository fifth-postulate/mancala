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
        self.current.finished()
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
        let mut stones = bowls[bowl];
        let mut index = bowl; let mut store_offset = 0; let mut stored = 0;
        let mut change_player = true;
        bowls[index] = 0;
        while stones > 0 {
            index += 1;
            if index - store_offset == 2*self.size { index = 0; store_offset = 0; }
            if index == self.size {
                stored += 1;
                store_offset = 1;
            } else {
                bowls[index - store_offset] += 1;
            }
            if stones == 1 && index == self.size {
                change_player = false
            }
            if stones == 1 && store_offset == 0 && bowls[index] == 1 {
                let capture_index = 2*self.size - 1 - index;
                stored += bowls[capture_index];
                bowls[capture_index] = 0;
            }
            stones -= 1;
        }
        let mut capture = [self.capture[0] + stored as u8, self.capture[1]];
        if change_player {
            capture = [capture[1], capture[0]];
            bowls.rotate_left(self.size);
        }
        Position {
            size: self.size,
            capture: capture,
            bowls,
        }
    }

    /// Determine if a position is finished
    pub fn finished(&self) -> bool {
        self.bowls[0..self.size]
            .iter()
            .all(|&stones| stones == 0) ||
            self.bowls[self.size..(2*self.size)]
            .iter()
            .all(|&stones| stones == 0)
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

    #[test]
    fn play_that_goes_over_store_should_capture_stone() {
        let start= Position::from([2, 2, 2, 2]);

        let actual = start.play(1);

        let expected = Position::from((0, 1, [3, 2, 2, 0]));
        assert_eq!(actual, Some(expected))
    }

    #[test]
    fn play_that_cycles_should_start_over() {
        let start= Position::from([6, 6, 6, 6]);

        let actual = start.play(0);

        let expected = Position::from((0, 1, [7, 7, 1, 8]));
        assert_eq!(actual, Some(expected))
    }

    #[test]
    fn play_into_your_store_allows_an_other_turn() {
        let start= Position::from([2, 2, 2, 2, 2, 2]);

        let actual = start.play(1);

        let expected = Position::from((1, 0, [2, 0, 3, 2, 2, 2]));
        assert_eq!(actual, Some(expected))
    }

    #[test]
    fn play_into_empty_bowl_captures_opposite_bowl() {
        let start= Position::from([2, 2, 0, 2, 2, 2, 2, 2]);

        let actual = start.play(0);

        let expected = Position::from((0, 2, [2, 0, 2, 2, 0, 3, 1, 2]));
        assert_eq!(actual, Some(expected))
    }

    #[test]
    fn positions_with_no_stones_on_one_side_is_finished() {
        let start= Position::from([0, 0, 2, 2]);

        assert!(start.finished())
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

macro_rules! position_with_capture_from_array_for_sizes {
    ( $($n : expr),* ) => {
        $(
            impl From<(u8, u8, [u8; $n])> for Position {
                fn from(data: (u8, u8, [u8; $n])) -> Self {
                    Position {
                        size: $n/2,
                        capture: [data.0, data.1],
                        bowls: data.2.to_vec(),
                    }
                }
            }
        )*
    }
}

position_from_array_for_sizes!(2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32);
position_with_capture_from_array_for_sizes!(2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32);
