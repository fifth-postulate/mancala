//! A strategy that allows user interaction.
//!
//! It displays a position on standard out and asks the user what to play.
//! Plays are given by index, which is zero based.
use crate::game::{Bowl, Position};
use crate::strategy::Strategy;
use std::io;

/// Ask the user for a play.
pub struct User {}

/// Create the user strategy
pub fn user() -> User {
    User {}
}

impl Strategy for User {
    fn play(&mut self, position: &Position) -> Option<Bowl> {
        let plays = position.options();

        let stdin = io::stdin();
        let input = &mut String::new();

        println!("{}", position);
        let mut play = None;
        while play.is_none() {
            println!("enter a play: ");
            input.clear();
            stdin.read_line(input).expect("no problems reading a line");
            match input.trim().parse::<Bowl>() {
                Ok(bowl) => {
                    if plays.contains(&bowl) {
                        play = Some(bowl)
                    } else {
                        println!("not an option");
                    }
                }

                Err(_) => println!("enter a bowl."),
            }
        }

        play
    }
}
