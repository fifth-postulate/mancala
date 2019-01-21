# mancala [![Build Status](https://travis-ci.org/fifth-postulate/mancala.svg?branch=master)](https://travis-ci.org/fifth-postulate/mancala)
Reinforcement learning Mancala, in Rust.

## What is
This section will define the terms in the description of this project.

### Mancala
[Mancala][mancala] is a

> generic name for a family of two-player turn-based strategy board games played with small stones, beans, or seeds and rows of holes or pits in the earth, a board or other playing surface. The objective is usually to capture all or some set of the opponent's pieces.

### Rust
[Rust][rust] is a programming language that

> is fundamentally about empowerment: no matter what kind of code you are writing now, Rust empowers you to reach farther, to program with confidence in a wider variety of domains than you did before.

### Reinforcement learning
[Reinforcement learning][rl] is an

> area of machine learning concerned with how software agents ought to take actions in an environment so as to maximize some notion of cumulative reward. The problem, due to its generality, is studied in many other disciplines, such as game theory, control theory, operations research, information theory, simulation-based optimization, multi-agent systems, swarm intelligence, statistics and genetic algorithms.

## Usage
### Who will win?
Let's say we are interested in a game of mancala where each player has only one bowl. Players do not have a real choice. They can only play their single bowl. So determining who wins should not be that hard. Let's use `mancala` for that.

Start with announcing the `mancala` crate.

```rust
extern crate mancala;
```

Since we will be using `Position` directly import it now.

```rust
use mancala::game::Position;
```

Let's agree to start out with 8 stones each. We can create a starting position from a array in the following manner

```rust
let mut position = Position::from([8; 2]);
```

This array represents the bowls with their stones, one bowl for each player.

No we want to play from the only bowl until the game is over.

```rust
while !position.finished() {
    position = position.play(0).unwrap();
}
```

With the game finished, we can determine the score.

```rust
println!("score {}", position.score().unwrap());
```

Now we know who is going to win. You can see a similar [program][position2] in `src/bin`

## Strategies
A strategy is an algorithm that determines what to play in a given situation. A strategy can have a profound impact. For example below there is a comparison between the strategies [`MinMax`][minmax] en [`AlphaBeta`][alphabeta].

Both strategies are used to determine which player will win for a mancala game with two bowls and a number of stones between 1 and 14.

| number of stones | Score |
|                1 |    -2 |
|                2 |    -2 |
|                3 |     4 |
|                4 |    1- |
|                5 |     6 |
|                6 |    -8 |
|                7 |    12 |
|                8 |    -4 |
|                9 |   -10 |
|               10 |     6 |
|               11 |    -6 |
|               12 |    14 |
|               13 |    -6 |
|               14 |    -4 |

The time it took to produce these result are found below

| Algorithm   |  Time |
| `MinMax`    | 25.21 |
| `AlphaBeta` | 0.59  |

Which is quit impressive.

[mancala]: https://en.wikipedia.org/wiki/Mancala
[rust]: https://www.rust-lang.org/
[rl]: https://en.wikipedia.org/wiki/Reinforcement_learning
[position2]: https://github.com/fifth-postulate/mancala/blob/master/src/bin/position2.rs
[minmax]: https://en.wikipedia.org/wiki/Minimax
[alphabeta]: https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning
