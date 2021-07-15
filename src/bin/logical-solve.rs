use std::collections::{hash_map::RandomState, HashMap};

use mmsolv::Desc;

fn main() {
    let mut args = std::env::args().skip(1);
    let clues = mmsolv::parse_shortform(&args.next().expect("Need string as first arg"));
    let mut peg_map = HashMap::<_, _, RandomState>::default();
    if let Some(mappings) = args.next() {
        dbg!(&mappings);
        let kvpairs = mappings.split_whitespace();
        for pair in kvpairs {
            dbg!(pair);
            let (k, v) = pair.split_once('=').unwrap();
            println!("{} is {}", k, v);
            peg_map.insert(k.as_bytes()[0], v.to_owned());
        }
    }
    let (result, descs) = mmsolv::solve_logical(&[], &clues);
    let mut buf = String::new();
    for desc in descs {
        match desc {
            Desc::Text(text) => println!("{}", text),
            Desc::AtLeastMustBe { peg, times } => {
                println!(
                    "\tAt least {} {} peg{} *must* be in the solution",
                    times,
                    peg_map.get(&peg).unwrap_or_else(|| {
                        buf = (peg as char).to_string();
                        &buf
                    }),
                    if times == 1 { "" } else { "s" }
                );
            }
            Desc::AtLeastCantBe { peg, times } => {
                println!(
                    "\tAt least {} {} peg{} *can not* be in the solution",
                    times,
                    peg_map.get(&peg).unwrap_or_else(|| {
                        buf = (peg as char).to_string();
                        &buf
                    }),
                    if times == 1 { "" } else { "s" }
                );
            }
        }
    }
    match result {
        Some(solution) => println!(
            "The solution is: {}",
            std::str::from_utf8(&solution).unwrap()
        ),
        None => println!("Couldn't determine a solution."),
    }
}
