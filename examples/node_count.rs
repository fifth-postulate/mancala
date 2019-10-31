extern crate mancala;

use mancala::strategy::{MinMax, Strategy};
use mancala::game::Position;

fn main() {
    let position = Position::new(2, 2);
    let mut strategy = MinMax::new();
    strategy.play(&position);

    println!("nodes: {} depth: {}", strategy.analyzer.node_count, strategy.analyzer.max_depth)

}