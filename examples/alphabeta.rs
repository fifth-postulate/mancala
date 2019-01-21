extern crate mancala;

use mancala::bout::Bout;
use mancala::game::GameBuilder;
use mancala::strategy::AlphaBeta;

fn main() {
    let mut red_strategy = AlphaBeta {};
    let mut blue_strategy = AlphaBeta {};
    let mut bout = Bout::new(&mut red_strategy, &mut blue_strategy);

    for stones in 1..15 {
        let game = GameBuilder::new().bowls(2).stones(stones).build();
        let result = bout.start(game).expect("a finished game with score");
        println!("{} {:?}", stones, result.score());
    }
}