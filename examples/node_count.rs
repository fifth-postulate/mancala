extern crate mancala;

use mancala::game::Position;
use mancala::strategy::{MinMax, Strategy};

fn main() {
    for bowls in 1..4 {
        for stones in 1..4 {
            let position = Position::new(bowls, stones);
            let mut strategy = MinMax::new();
            strategy.play(&position);

            println!("bowls: {} stones:{} {}", bowls, stones, strategy.analyzer)
        }
    }
}
