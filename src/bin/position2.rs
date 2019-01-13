extern crate mancala;

use mancala::game::Position;

fn main() {
    for stones in 1..100 {
        let mut position = Position::from([stones; 2]);

        let mut round = 0;
        while !position.finished() {
            position = position.play(0).unwrap();
            round += 1;
        }

        println!("stones {}, round {}, score {}", stones, round, position.score().unwrap());
    }
}
