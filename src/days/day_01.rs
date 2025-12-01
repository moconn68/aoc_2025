use std::io::BufRead;

use crate::puzzle::Puzzle;

pub fn get_puzzle() -> Puzzle {
    Puzzle {
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    }
}

fn part_one(input: Box<dyn BufRead>) -> anyhow::Result<crate::puzzle::Answer> {
    let ans = _part_one(input);
    Ok(Box::new(ans))
}

fn part_two(input: Box<dyn BufRead>) -> anyhow::Result<crate::puzzle::Answer> {
    let ans = _part_two(input);
    Ok(Box::new(ans))
}

fn _part_one(_input: impl BufRead) -> i32 {
    todo!("Actual logic implementation goes here")
}

fn _part_two(_input: impl BufRead) -> i32 {
    todo!("Actual logic implementation goes here")
}
