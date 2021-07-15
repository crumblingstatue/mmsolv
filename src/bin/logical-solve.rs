fn main() {
    let clues =
        mmsolv::parse_shortform(&std::env::args().nth(1).expect("Need string as first arg"));
    let (result, steps) = mmsolv::solve_logical(&[], &clues);
    for step in steps {
        println!("{}", step);
    }
    match result {
        Some(solution) => println!(
            "The solution is: {}",
            std::str::from_utf8(&solution).unwrap()
        ),
        None => println!("Couldn't determine a solution."),
    }
}
