use crate::Clue;

pub struct LogicalSolver<'c> {
    clues: &'c [Clue],
}

impl<'c> LogicalSolver<'c> {
    pub fn new(clues: &'c [Clue]) -> Self {
        Self { clues }
    }
}

impl<'c> Iterator for LogicalSolver<'c> {
    type Item = StepSummary;

    fn next(&mut self) -> Option<Self::Item> {
        Some(StepSummary {})
    }
}

pub struct StepSummary {}

impl StepSummary {
    pub fn describe(&self) -> String {
        "Test message".into()
    }
}
