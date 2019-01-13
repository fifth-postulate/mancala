extern crate mancala;

use mancala::game::{GameBuilder, Game};
use mancala::strategy::Strategy;
use mancala::strategy::naive::First;
use mancala::bout::Bout;

fn main() {
    let mut red_strategy = First {};
    let mut blue_strategy = First {};
    let mut bout = Bout::new(&mut red_strategy, &mut blue_strategy);

    let game = GameBuilder::new()
        .bowls(2)
        .stones(4)
        .build();

    let game = bout.start(game).expect("a finished game with score");
    println!("game: {}", game.score().expect("a score"));
}
