//! Coordination of a bout between strategies.

use super::game::{FoulPlay, Game, Player, Bowl};
use super::strategy::Strategy;

/// Representation of the bout
pub struct Bout<'a> {
    displayer: &'a dyn DisplayPlay,
    red_strategy: &'a mut dyn Strategy,
    blue_strategy: &'a mut dyn Strategy,
}

/// Trait to display the play made by a strategy.
pub trait DisplayPlay {
    /// Receive the bowl played so that it can be displayed.
    fn display(&self, bowl_played: Bowl);
}

impl<F> DisplayPlay for F
where F: Fn(Bowl) + Sized {
    fn display(&self, bowl_played: Bowl) {
        self(bowl_played)
    }
}

/// Problems that can occur during a bout
#[derive(Debug)]
pub enum Problem {
    /// A problem that occurs before plays have been made
    RightOutOfTheGate,
    /// A player played an illegal move
    IllegalPlay(Player, FoulPlay),
    /// A player did not make a play
    NoPlay(Player),
}

impl<'a> Bout<'a> {
    /// Create a bout between strategies.
    pub fn new(red_strategy: &'a mut dyn Strategy, blue_strategy: &'a mut dyn Strategy, displayer: &'a dyn DisplayPlay) -> Self {
        Bout {
            displayer,
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
                Some(bowl) => {
                    self.displayer.display(bowl);
                    game
                    .play(bowl)
                    .map_err(|foul_play| Problem::IllegalPlay(game.turn(), foul_play))
                },
                None => Err(Problem::NoPlay(game.turn())),
            };
            if result.is_err() {
                break;
            }
        }
        result.map(|_| game)
    }
}
