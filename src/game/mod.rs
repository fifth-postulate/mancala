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

use std::fmt::{self, Display, Formatter};

/// Representation of a Bowl
pub type Bowl = usize;

/// Representation of a number of stones in a bowl
pub type Stones = u8;

/// Score a finished game;
pub type Score = i8;

/// GameBuilder is used to create a Mancala game.
pub struct GameBuilder {
    bowls: u8,
    stones: Stones,
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
    pub fn stones(self, stones: Stones) -> Self {
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

impl Default for GameBuilder {
    fn default() -> Self {
        GameBuilder::new()
    }
}

/// Game is an sequence of Positions.
///
/// A Game is created with a GameBuilder.
#[derive(Debug, PartialEq)]
pub struct Game {
    /// The current position of this game
    pub current: Position,
    history: Vec<(Player, Bowl)>,
}

impl Game {
    /// Determine if this game is finished
    pub fn finished(&self) -> bool {
        self.current.finished()
    }

    /// Determine which bowls are playable.
    pub fn options(&self) -> Vec<Bowl> {
        self.current.options()
    }

    /// Play a certain bowl.
    ///
    /// Fails if the bowl does not contain any stones.
    pub fn play(&mut self, bowl: Bowl) -> Result<(), FoulPlay> {
        match self.current.play(bowl) {
            Some(position) => {
                self.history.push((self.current.player, bowl));
                self.current = position;
                Ok(())
            }

            None => Err(FoulPlay::NoStonesInBowl),
        }
    }

    /// Determine the score of a game.
    ///
    /// None if the game is not finished
    pub fn score(&self) -> Option<Score> {
        self.current.score()
    }

    /// Return which players turn it is
    pub fn turn(&self) -> Player {
        self.current.turn()
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
    player: Player,
    size: usize,
    capture: [Stones; 2],
    bowls: Vec<Stones>,
}

/// The names for the player.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Player {
    /// The starting player
    Red,
    /// The other player
    Blue,
}

impl Player {
    /// The opposite player
    pub fn other(self) -> Self {
        match self {
            Player::Red => Player::Blue,
            Player::Blue => Player::Red,
        }
    }
}

impl Position {
    /// Create a position with a number of bowls and a number of stones per bowl.
    pub fn new(bowls: u8, stones: Stones) -> Self {
        let size = bowls as usize;
        let bowls = vec![stones; 2 * size];
        Position {
            player: Player::Red,
            size,
            capture: [0, 0],
            bowls,
        }
    }

    /// Determine which bowls are playable.
    pub fn options(&self) -> Vec<Bowl> {
        self.bowls[0..self.size]
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(bowl, stones)| if stones > 0 { Some(bowl) } else { None })
            .collect()
    }

    /// Play a certain bowl.
    ///
    /// If the bowl is empty, returns nothing.
    pub fn play(&self, bowl: Bowl) -> Option<Self> {
        if self.bowls[bowl] > 0 {
            Some(self.sow(bowl))
        } else {
            None
        }
    }

    fn sow(&self, bowl: Bowl) -> Self {
        let mut player = self.player;
        let mut bowls = self.bowls.clone();
        let stones = bowls[bowl];
        bowls[bowl] = 0;
        let tour = 2 * self.size + 1;
        let all_gain = stones as usize / tour;
        bowls = bowls
            .iter()
            .map(|s| (*s as usize + all_gain) as Stones)
            .collect();
        let extra = stones as usize % tour;
        let final_index = bowl + extra;
        (1..=(if final_index >= self.size {
            extra - 1
        } else {
            extra
        }))
            .for_each(|i| bowls[(bowl + i) % (2 * self.size)] += 1);
        let mut captured = 0;
        if final_index < self.size && bowls[final_index] == 1 {
            let index = 2 * self.size - 1 - final_index;
            captured = bowls[index] as usize;
            bowls[index] = 0;
        }
        let capture_gain = all_gain + captured + if final_index >= self.size { 1 } else { 0 };
        let mut capture = [self.capture[0] + capture_gain as Stones, self.capture[1]];
        let change_player = final_index != self.size;
        if change_player {
            player = self.player.other();
            capture = [capture[1], capture[0]];
            bowls.rotate_left(self.size);
        }
        Position {
            player,
            size: self.size,
            capture,
            bowls,
        }
    }

    /// Determine if a position is finished.
    /// 
    /// A position is finished when the current play can't make any plays
    pub fn finished(&self) -> bool {
        self.bowls[0..self.size].iter().all(|&stones| stones == 0)
   }

    /// Which player is allowed to make a play
    pub fn active_player(&self) -> Player {
        self.player
    }

    /// Determine the score after the game is finished.
    ///
    /// Scores are awarded to the current player. Positive scores are a win, negative scores are a loss.
    pub fn score(&self) -> Option<Score> {
        if self.finished() {
            let mut first: Stones = self.bowls[0..self.size].iter().cloned().sum();
            first += self.capture[0];
            let mut second: Stones = self.bowls[self.size..2 * self.size].iter().cloned().sum();
            second += self.capture[1];

            let score = first as Score - second as Score;
            Some(score)
        } else {
            None
        }
    }

    /// Difference between the actual captured stones
    pub fn delta(&self) -> Score {
        self.capture[0] as Score - self.capture[1] as Score
    }

    /// Return which players turn it is
    pub fn turn(&self) -> Player {
        self.player
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let current_side = &self.bowls[0..self.size];
        let opposite_side: &Vec<Stones> = &self.bowls[self.size..].iter().cloned().rev().collect();

        write!(f, "{:<3}", self.capture[1])?;
        for stones in opposite_side {
            write!(f, "| {:<3} ", stones)?
        }
        writeln!(f, "|")?;
        write!(f, "{:<3}", "")?;
        for stones in current_side {
            write!(f, "| {:<3} ", stones)?
        }
        writeln!(f, "| {:<3}", self.capture[0])
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
        }
    }

    struct PlayedGameBuilder {
        current: Position,
    }

    impl PlayedGameBuilder {
        fn with_history(self, history: Vec<(Player, Bowl)>) -> Game {
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

        let position = (Player::Blue, [2, 2, 2, 0, 3, 3]);
        let expected = from_position(position).with_history(vec![(Player::Red, 0)]);
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn play_that_goes_over_store_should_capture_stone() {
        let start = Position::from([2, 2, 2, 2]);

        let actual = start.play(1);

        let expected = Position::from((Player::Blue, 0, 1, [3, 2, 2, 0]));
        assert_eq!(actual, Some(expected))
    }

    #[test]
    fn play_that_cycles_should_start_over() {
        let start = Position::from([6, 6, 6, 6]);

        let actual = start.play(0);

        let expected = Position::from((Player::Blue, 0, 1, [7, 7, 1, 8]));
        assert_eq!(actual, Some(expected))
    }

    #[test]
    fn play_into_your_store_allows_an_other_turn() {
        let start = Position::from([2, 2, 2, 2, 2, 2]);

        let actual = start.play(1);

        let expected = Position::from((1, 0, [2, 0, 3, 2, 2, 2]));
        assert_eq!(actual, Some(expected))
    }

    #[test]
    fn play_into_empty_bowl_captures_opposite_bowl() {
        let start = Position::from([2, 2, 0, 2, 2, 2, 2, 2]);

        let actual = start.play(0);

        let expected = Position::from((Player::Blue, 0, 2, [2, 0, 2, 2, 0, 3, 1, 2]));
        assert_eq!(actual, Some(expected))
    }

    #[test]
    fn positions_with_no_stones_on_one_side_is_finished() {
        let start = Position::from([0, 0, 2, 2]);

        assert!(start.finished());
        assert_eq!(start.score(), Some(-4));
    }

    #[test]
    fn play_changes_player() {
        let start = Position::from([1, 0, 1, 0]);

        let actual = start.play(0).unwrap();

        let expected = Position::from((Player::Blue, 0, 1, [0, 0, 0, 1]));
        assert_eq!(actual, expected);
        assert_eq!(expected.score(), Some(-2));
    }
}

macro_rules! position_from_array_for_sizes {
    ( $($n : expr),* ) => {
        $(
            impl From<[Stones; $n]> for Position {
                fn from(bowls: [Stones; $n]) -> Self {
                    Position {
                        player: Player::Red,
                        size: $n/2,
                        capture: [0, 0],
                        bowls: bowls.to_vec(),
                    }
                }
            }
        )*
    }
}

macro_rules! position_with_player_from_array_for_sizes {
    ( $($n : expr),* ) => {
        $(
        impl From<(Player, [Stones; $n])> for Position {
            fn from(data: (Player, [Stones; $n])) -> Self {
                Position {
                    player: data.0,
                    size: $n/2,
                    capture: [0, 0],
                    bowls: data.1.to_vec(),
                }
            }
        }
        )*
    }
}

macro_rules! position_with_capture_from_array_for_sizes {
    ( $($n : expr),* ) => {
        $(
            impl From<(Stones, Stones, [Stones; $n])> for Position {
                fn from(data: (Stones, Stones, [Stones; $n])) -> Self {
                    Position {
                        player: Player::Red,
                        size: $n/2,
                        capture: [data.0, data.1],
                        bowls: data.2.to_vec(),
                    }
                }
            }
        )*
    }
}

macro_rules! position_with_player_with_capture_from_array_for_sizes {
    ( $($n : expr),* ) => {
        $(
            impl From<(Player, Stones, Stones, [Stones; $n])> for Position {
                fn from(data: (Player, Stones, Stones, [Stones; $n])) -> Self {
                    Position {
                        player: data.0,
                        size: $n/2,
                        capture: [data.1, data.2],
                        bowls: data.3.to_vec(),
                    }
                }
            }
        )*
    }
}

position_from_array_for_sizes!(2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32);
position_with_player_from_array_for_sizes!(
    2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32
);
position_with_capture_from_array_for_sizes!(
    2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32
);
position_with_player_with_capture_from_array_for_sizes!(
    2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32
);
