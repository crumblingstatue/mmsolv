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
//! In order for a *guess* to be correct, it must not contradict and of the *clues*.
//! For example, if a clue has a green peg at the first slot, but it has no hearts,
//! a guess having a green peg at the first slot is not a valid solution, because it contradicts
//! a clue.
//!
//! The strategy this library employs is brute-force: It generates guesses of all possible
//! combinations of pegs, and checks each against all clues. If a generated guess doesn't contradict
//! any clue, it is a valid solution.
//!

mod combinations;

pub type Peg = u8;
pub type Pegs = [Peg];

#[derive(Debug)]
pub struct Clue {
    pub indicator: Indicator,
    pub pegs: Box<Pegs>,
}

#[derive(PartialEq, Eq, Debug)]
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

/// Compares `guess` against `clue`, and returns the resulting indicator
fn compare(guess: &Pegs, clue: &Pegs) -> Indicator {
    assert!(guess.len() == clue.len());
    let len = guess.len();
    let mut markers = vec![Marker::None; len];
    // First determine hearts
    for (i, &g_peg) in guess.iter().enumerate() {
        if clue[i] == g_peg {
            markers[i] = Marker::Heart;
        }
    }
    // Then determine dots
    for (i, &g_peg) in guess.iter().enumerate() {
        if markers[i] == Marker::Heart {
            // A peg used for a heart can't be used for being a dot.
            continue;
        }
        for (j, &c_peg) in clue.iter().enumerate() {
            if g_peg == c_peg && markers[j] == Marker::None {
                markers[j] = Marker::Dot;
                // A single guess peg can only ever count as one dot, and
                // we just counted a dot, so break.
                break;
            }
        }
    }
    let mut dots: u8 = 0;
    let mut hearts: u8 = 0;
    for marker in markers {
        if marker == Marker::Dot {
            dots += 1;
        } else if marker == Marker::Heart {
            hearts += 1;
        }
    }
    Indicator { dots, hearts }
}

fn validate_guess(guess: &Pegs, clues: &[Clue]) -> bool {
    for clue in clues {
        if clue.indicator != compare(guess, &clue.pegs) {
            return false;
        }
    }
    true
}

pub fn solve(set: &Pegs, clues: &[Clue]) -> Option<String> {
    let first_clue = match clues.get(0) {
        Some(clue) => clue,
        None => return None,
    };
    let combos = combinations::SliceCombo::new(set, first_clue.pegs.len());
    for guess in combos {
        if validate_guess(&guess, clues) {
            return Some(String::from_utf8(guess).unwrap());
        }
    }
    None
}

#[test]
fn test_compare() {
    assert_eq!(
        compare(b"wpgrc", b"rwcrg"),
        Indicator { dots: 3, hearts: 1 }
    );
    assert_eq!(
        compare(b"gwgcg", b"rwcrg"),
        Indicator { dots: 1, hearts: 2 }
    );
    assert_eq!(
        compare(b"wwwgr", b"rwcrg"),
        Indicator { dots: 2, hearts: 1 }
    );
    assert_eq!(
        compare(b"brbrr", b"ggbgb"),
        Indicator { dots: 1, hearts: 1 }
    );
}
