extern crate clap;
extern crate mancala;

use clap::{App, Arg};
use mancala::bout::Bout;
use mancala::game::{Bowl, GameBuilder, Player};
use mancala::strategy::tree::Depth;
use mancala::strategy::{user, AlphaBeta, First, MinMax, Random, Strategy};
use rand::thread_rng;
use std::ops::Neg;

fn main() {
    let matches = App::new("Mancala Battle")
        .version("1.0")
        .author("Daan van Berkel <daan.v.berkel.1980@gmail.com>")
        .about("Pit various strategies to a game of Mancala")
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
        .arg(
            Arg::with_name("red")
                .long("red")
                .value_name("RED_STRATEGY")
                .help("the strategy the red player will employ")
                .default_value("user")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("blue")
                .long("blue")
                .value_name("BLUE_STRATEGY")
                .help("the strategy the blur player will employ")
                .default_value("alphabeta")
                .takes_value(true),
        )
        .get_matches();

    let depth = Depth::Limit(matches.value_of("depth").unwrap().parse().unwrap_or(5));
    let mut red_strategy = strategy_from_name(matches.value_of("red").unwrap_or("alphabeta"), depth);
    let mut blue_strategy = strategy_from_name(matches.value_of("blue").unwrap_or("alphabeta"), depth);   
    let mut bout = Bout::new(
        &mut red_strategy,
        &mut blue_strategy,
        &(|bowl: Bowl| println!("played {}", bowl)),
    );

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

fn strategy_from_name(name: &str, depth: Depth) -> Box<dyn Strategy> {
    match name {
        "user" => Box::new(user()),
        "minmax" => Box::new(MinMax::new()),
        "alphabeta" => Box::new(AlphaBeta::strategy().limited_to(depth).build()),
        "random" => Box::new(Random::new(thread_rng())),
        "first" => Box::new(First::new()),
        _ => Box::new(user()),
    }
}
