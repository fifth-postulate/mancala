//! Coordination of a bout between strategies.

use super::game::{FoulPlay, Game, Player, Score};
use super::strategy::Strategy;

/// Representation of the bout
pub struct Bout<'a> {
    red_strategy: &'a mut dyn Strategy,
    blue_strategy: &'a mut dyn Strategy,
}

/// Problems that can occur during a bout
#[derive(Debug)]
pub enum Problem {
    RightOutOfTheGate,
    IllegalPlay(Player, FoulPlay),
    NoPlay(Player),
}

impl<'a> Bout<'a> {
    /// Create a bout between strategies.
    pub fn new(red_strategy: &'a mut Strategy, blue_strategy: &'a mut Strategy) -> Self {
        Bout {
            red_strategy,
            blue_strategy,
        }
    }

    /// Start the bout. Returns the result.
    pub fn start(&mut self, game: Game) -> Result<Game, Problem> {
        let mut game = game;
        let mut result = Err(Problem::RightOutOfTheGate);
        while !game.finished() {
            let bowl_option = match game.turn() {
                Player::Red => self.red_strategy.play(&game.current),
                Player::Blue => self.blue_strategy.play(&game.current),
            };
            result = match bowl_option {
                Some(bowl) => game
                    .play(bowl)
                    .map_err(|foul_play| Problem::IllegalPlay(game.turn(), foul_play)),

                None => Err(Problem::NoPlay(game.turn())),
            };
            if result.is_err() {
                break;
            }
        }
        result.map(|_| game)
    }
}
