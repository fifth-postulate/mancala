extern crate mancala;

use std::ops::Neg;
use mancala::bout::Bout;
use mancala::game::{GameBuilder, Player};
use mancala::strategy::tree::Depth;
use mancala::strategy::{AlphaBeta, User};

fn main() {
    let mut red_strategy = User {};
    let mut blue_strategy = AlphaBeta::limited_to(Depth::Limit(10));
    let mut bout = Bout::new(&mut red_strategy, &mut blue_strategy);

    let game = GameBuilder::new().bowls(6).stones(4).build();
    let result = bout.start(game).expect("a finished game with score");
    let mut score = result.score().expect("a defined score");
    if result.turn() != Player::Red {
        score = score.neg();
    }
    println!("{:?}", score);
}
