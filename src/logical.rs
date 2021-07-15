use crate::{Clue, Peg};
use std::{collections::HashMap, fmt::Write};

fn pegs_debug(pegs: &[Peg]) {
    print!("[");
    for &peg in pegs {
        print!("{} ", peg as char);
    }
    println!("\u{0008}]");
}

pub fn solve_logical(free: &[Peg], clues: &[Clue]) -> (Option<Vec<u8>>, Vec<String>) {
    let mut solution = None;
    let mut steps = vec!["First, we try the replacement rule.".into()];
    // Try the replacement rule
    for i in 0..clues.len() {
        for j in i + 1..clues.len() {
            let c1 = &clues[i];
            let c2 = &clues[j];
            let diff_ind = c1.indicator.difference(&c2.indicator);
            let diff_pegs = get_pegs_difference(&c1.pegs, &c2.pegs);
            if diff_ind == diff_pegs.count() {
                println!(
                    "({}, {}) Peg difference and indicator difference is same ({}, {})",
                    i,
                    j,
                    diff_pegs.count(),
                    diff_ind
                );
                pegs_debug(&c1.pegs);
                pegs_debug(&c2.pegs);
                let mut step = format!(
                    "Applying the replacement rule for rows {} and {}, we can deduce that:\n",
                    i + 1,
                    j + 1
                );
                // Pegs that were added must be in the solution
                for (added_peg, times) in diff_pegs.added {
                    writeln!(
                        &mut step,
                        "\tAt least {} {} must be in the solution",
                        times, added_peg as char
                    )
                    .unwrap();
                }
                // Pegs that were removed can not be in the solution
                for (removed_peg, times) in diff_pegs.removed {
                    writeln!(
                        &mut step,
                        "\tAt least {} {} can not be in the solution",
                        times, removed_peg as char
                    )
                    .unwrap();
                }
                steps.push(step);
            }
        }
    }
    (solution, steps)
}

struct PegsDifference {
    added: PegCountMap,
    removed: PegCountMap,
}

impl PegsDifference {
    /// Count the number of differences
    fn count(&self) -> u8 {
        let added = pegmap_count_total(&self.added);
        let removed = pegmap_count_total(&self.removed);
        assert_eq!(added, removed);
        added
    }
}

fn pegmap_count_total(map: &PegCountMap) -> u8 {
    map.values().sum()
}

type PegCountMap = HashMap<Peg, u8>;

fn count(pegs: &[Peg]) -> PegCountMap {
    let mut hm = HashMap::default();
    for &peg in pegs {
        *hm.entry(peg).or_insert(0) += 1
    }
    hm
}

/// What map2 adds to map1
fn adds(map1: &PegCountMap, map2: &PegCountMap) -> PegCountMap {
    let mut adds = PegCountMap::default();
    for (&k2, &v2) in map2 {
        let v1 = *map1.get(&k2).unwrap_or(&0);
        if v2 > v1 {
            adds.insert(k2, v2 - v1);
        }
    }
    adds
}

/// What map2 removes from map1
fn removes(map1: &PegCountMap, map2: &PegCountMap) -> PegCountMap {
    let mut removes = PegCountMap::default();
    for (&k1, &v1) in map1 {
        let v2 = *map2.get(&k1).unwrap_or(&0);
        if v2 < v1 {
            removes.insert(k1, v1 - v2);
        }
    }
    removes
}

#[cfg(test)]
macro_rules! make_countmap {
    ($($k:literal*$v:literal)+) => {{
        let mut map = PegCountMap::default();
        $(
            map.insert($k, $v);
        )+
        map
    }};
}

#[test]
fn test_adds_removes() {
    let pegs1 = make_countmap!(99*2 114*1 103*1 112*1);
    let pegs2 = make_countmap!(121*1 112*2 99*2);
    assert_eq!(adds(&pegs1, &pegs2), make_countmap!(121*1 112*1));
    assert_eq!(removes(&pegs1, &pegs2), make_countmap!(114*1 103*1));
}

fn get_pegs_difference(group1: &[Peg], group2: &[Peg]) -> PegsDifference {
    let pegs1 = count(group1);
    let pegs2 = count(group2);
    PegsDifference {
        added: adds(&pegs1, &pegs2),
        removed: removes(&pegs1, &pegs2),
    }
}

/*Relacement Rule:
If the difference in number of markers ( 💖 + ⚪ ) between two rows is the same as
 the number of different pegs, then all those which were removed can't be in the solution,
 and all those which were added have to be in the solution. */
