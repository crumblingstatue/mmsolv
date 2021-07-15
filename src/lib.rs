//! Monster mind puzzle solver for 3/4/5 peg puzzles
//!
//! A puzzle of N size consists of:
//!
//! - The player's *guess* at the top. This is what we are solving for.
//! - A number of *clues*
//!
//! The *clues* consist of N *pegs* of different colors.
//! Each clue also has an *indicator* of how much it matches with the solution.
//! An *indicator* consists of 0 or more *hearts* and *dots*.
//! A *heart* indicates an exact match. It means there is a peg bug that has both
//! the right color and location.
//! A *dot* indicates a partial match. It means there is a peg bug that has the right color,
//! but it isn't at the right location.
//!
//! In order for a *guess* to be correct, it must not contradict any of the *clues*.
//! For example, if a clue has a green peg at the first slot, but it has no hearts,
//! a guess having a green peg at the first slot is not a valid solution, because it contradicts
//! a clue.
//!
//! The strategy this library employs is brute-force: It generates guesses of all possible
//! combinations of pegs, and checks each against all clues. If a generated guess doesn't contradict
//! any clue, it is a valid solution.
//!

pub use bruteforce::{solve_bruteforce, solve_bruteforce_raw};
pub use short_form::parse as parse_shortform;

mod bruteforce;
mod combinations;
mod short_form;

pub type Peg = u8;
pub type Pegs = [Peg];

#[derive(Debug, Clone)]
pub struct Clue {
    pub indicator: Indicator,
    pub pegs: Box<Pegs>,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Indicator {
    pub dots: u8,
    pub hearts: u8,
}

#[derive(Copy, Clone, PartialEq)]
enum Marker {
    None,
    Dot,
    Heart,
}

#[test]
fn test_solve() {
    assert_eq!(
        solve_bruteforce(
            &[],
            &parse_shortform("ccprg12 cyppc11 crycg13 rccgg13 yrccc03")
        )
        .as_deref(),
        Some("cgrgy")
    );
}
