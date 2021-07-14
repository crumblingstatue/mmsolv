use mmsolv::{Clue, Indicator};

enum ParseState {
    Init,
    HeartsParsed,
}

fn parse_arg(arg: &str) -> Vec<Clue> {
    let mut state = ParseState::Init;
    let mut clues = Vec::new();
    let mut pegs = Vec::new();
    let mut hearts = 0;
    for &b in arg.as_bytes() {
        match state {
            ParseState::Init => {
                if b.is_ascii_alphabetic() {
                    pegs.push(b);
                } else if b.is_ascii_digit() {
                    // Parsing hearts
                    hearts = b - b'0';
                    state = ParseState::HeartsParsed;
                }
            }
            ParseState::HeartsParsed => {
                if b.is_ascii_digit() {
                    // Parsing dots
                    let dots = b - b'0';
                    clues.push(Clue {
                        pegs: pegs.clone().into_boxed_slice(),
                        indicator: Indicator { dots, hearts },
                    });
                    pegs.clear();
                    state = ParseState::Init;
                }
            }
        }
    }
    clues
}

fn main() {
    let clues = parse_arg(&std::env::args().nth(1).expect("Need string as first arg"));
    let result = mmsolv::solve_bruteforce(&[], &clues);
    match result {
        Some(solution) => println!("The soution is {}", solution),
        None => println!("There is no solution. Apparently."),
    }
}
