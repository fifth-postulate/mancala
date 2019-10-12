extern crate clap;
extern crate mancala;

use clap::{App, Arg};
use mancala::bout::Bout;
use mancala::game::{GameBuilder, Player};
use mancala::strategy::tree::Depth;
use mancala::strategy::{user, AlphaBeta};
use std::ops::Neg;

fn main() {
    let matches = App::new("Play Mancala")
        .version("1.0")
        .author("Daan van Berkel <daan.v.berkel.1980@gmail.com>")
        .about("Challenge the computer to a game of Mancala")
        .arg(
            Arg::with_name("bowls")
                .short("b")
                .long("bowls")
                .value_name("NUMBER")
                .help("the numbers of bowls")
                .default_value("6")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("stones")
                .short("s")
                .long("stones")
                .value_name("NUMBER")
                .help("the numbers of stones per bowl")
                .default_value("4")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("depth")
                .short("d")
                .long("depth")
                .value_name("NUMBER")
                .help("the strength of the computer, higher is stronger")
                .default_value("5")
                .takes_value(true),
        )
        .get_matches();

    let mut red_strategy = user();
    let depth = matches.value_of("depth").unwrap().parse().unwrap_or(5);
    let mut blue_strategy = AlphaBeta::strategy()
        .limited_to(Depth::Limit(depth))
        .build();
    let mut bout = Bout::new(&mut red_strategy, &mut blue_strategy);

    let bowls = matches.value_of("bowls").unwrap().parse().unwrap_or(6);
    let stones = matches.value_of("stones").unwrap().parse().unwrap_or(4);
    let game = GameBuilder::new().bowls(bowls).stones(stones).build();
    let result = bout.start(game).expect("a finished game with score");
    let mut score = result.score().expect("a defined score");
    if result.turn() != Player::Red {
        score = score.neg();
    }
    println!("{:?}", score);
}
