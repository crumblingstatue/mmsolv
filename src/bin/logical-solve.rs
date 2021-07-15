fn main() {
    let clues =
        mmsolv::parse_shortform(&std::env::args().nth(1).expect("Need string as first arg"));
    let solver = mmsolv::LogicalSolver::new(&clues);
    for step in solver {
        println!("{}", step.describe());
    }
}
