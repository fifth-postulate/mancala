extern crate mancala;

use mancala::bout::Bout;
use mancala::game::{GameBuilder, Player};
use mancala::strategy::Random;
use rand::rngs::ThreadRng;

fn main() {
    let mut red_strategy = Random::new(ThreadRng::default());
    let mut blue_strategy = Random::new(ThreadRng::default());
    let mut bout = Bout::new(&mut red_strategy, &mut blue_strategy);

    for stones in 1..15 {
        let game = GameBuilder::new().bowls(2).stones(stones).build();
        let result = bout.start(game).expect("a finished game with score");
        let score = result
            .score()
            .map(|score| score * score_multiplier(result.current.active_player()));
        println!("{} {:?}", stones, score);
    }
}

/// Scores are returned for the active player.
/// This multiplier corrects to the start player.
fn score_multiplier(active_player: Player) -> i8 {
    if active_player != Player::Red {
        -1
    } else {
        1
    }
}
