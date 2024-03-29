extern crate mancala;

use std::time::{Duration, SystemTime};

use mancala::bout::Bout;
use mancala::game::GameBuilder;
use mancala::strategy::AlphaBeta;

const MAXIMUM_ALLOWED_DURATION: Duration = Duration::from_millis(2000);

#[test]
fn calculating_end_games_should_be_quick() {
    let mut red_strategy = AlphaBeta::strategy().build();
    let mut blue_strategy = AlphaBeta::strategy().build();
    let mut bout = Bout::new(&mut red_strategy, &mut blue_strategy, &|_| {});

    let start_time = SystemTime::now();
    for stones in 1..15 {
        let game = GameBuilder::new().bowls(2).stones(stones).build();
        let _result = bout.start(game).expect("a finished game with score");
    }
    let finish_time = SystemTime::now();
    let duration = finish_time.duration_since(start_time).expect("a duration");

    assert!(duration < MAXIMUM_ALLOWED_DURATION);
}
