fn main() {
    let clues =
        mmsolv::parse_shortform(&std::env::args().nth(1).expect("Need string as first arg"));
    let result = mmsolv::solve_bruteforce(&[], &clues);
    match result {
        Some(solution) => println!("The soution is {solution}"),
        None => println!("There is no solution. Apparently."),
    }
}
