use std::{fmt::Display, io::BufRead};

/// At the highest level, a puzzle is a function that takes an input reader, and returns an [`Answer`].
pub type Solver = Box<dyn FnOnce(Box<dyn BufRead>) -> anyhow::Result<Answer>>;
/// At the highest level, the answer from each day is just displayed to the user.
pub type Answer = Box<dyn Display>;

/// Simple wrapper representing the puzzle for a given day.
pub struct Puzzle {
    pub part_one: Solver,
    pub part_two: Solver,
}
