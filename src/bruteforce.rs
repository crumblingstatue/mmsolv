//! This strategy is brute-force: It generates guesses of all possible
//! combinations of pegs, and checks each against all clues. If a generated guess doesn't contradict
//! any clue, it is a valid solution.

use crate::{combinations::SliceCombo, Clue, Indicator, Marker, Peg, Pegs};
use std::{collections::HashSet, convert::TryInto};

pub fn solve_bruteforce(free_pegs: &Pegs, clues: &[Clue]) -> Option<String> {
    let mut raw = solve_bruteforce_raw(free_pegs, clues);
    raw.next().map(|guess| String::from_utf8(guess).unwrap())
}

pub fn solve_bruteforce_raw<'a>(
    free_pegs: &'a Pegs,
    clues: &'a [Clue],
) -> impl Iterator<Item = Vec<u8>> + 'a {
    let first_clue = match clues.get(0) {
        Some(clue) => clue,
        None => panic!("Can't solve without clues"),
    };
    let set: HashSet<Peg> = clues
        .iter()
        .flat_map(|clue| clue.pegs.iter().cloned())
        .chain(free_pegs.iter().cloned())
        .collect();
    let set: Vec<Peg> = set.into_iter().collect();
    let combos = SliceCombo::new(set, first_clue.pegs.len());
    combos.filter(move |guess| validate_guess(guess, clues))
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

fn seven_peg_any_neighbouring_same(&[p0, p1, p2, p3, p4, p5, p6]: &[Peg; 7]) -> bool {
    //  [0][1]
    // [2][3][4]
    //  [5][6]
    // Sorry...
    p0 == p1
        || p0 == p2
        || p0 == p3
        || p1 == p3
        || p1 == p4
        || p2 == p3
        || p2 == p5
        || p3 == p4
        || p3 == p5
        || p3 == p6
        || p4 == p6
        || p5 == p6
}

fn validate_guess(guess: &Pegs, clues: &[Clue]) -> bool {
    for clue in clues {
        if clue.indicator != compare(guess, &clue.pegs) {
            return false;
        }
        if guess.len() == 7 && seven_peg_any_neighbouring_same(guess.try_into().unwrap()) {
            return false;
        }
    }
    true
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
