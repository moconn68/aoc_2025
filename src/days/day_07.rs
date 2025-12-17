use anyhow::Context;

use crate::puzzle::Puzzle;

use std::{collections::HashSet, io::BufRead};

pub(crate) fn get_puzzle() -> crate::puzzle::Puzzle {
    Puzzle {
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    }
}

fn part_one(input: Box<dyn BufRead>) -> anyhow::Result<crate::puzzle::Answer> {
    let ans = _part_one(input)?;
    Ok(Box::new(ans))
}

fn part_two(mut _input: Box<dyn BufRead>) -> anyhow::Result<crate::puzzle::Answer> {
    todo!()
}

const START: char = 'S';
const SPLITTER: char = '^';

fn _part_one(input: impl BufRead) -> anyhow::Result<usize> {
    let mut split_count = 0;
    let mut beam_cols = HashSet::new();
    let mut lines = input.lines();

    beam_cols.insert(
        lines
            .next()
            .context("Input is empty")??
            .chars()
            .position(|c| c == START)
            .context("'S' start point is missing'")?,
    );

    for line in lines {
        let line = line?;
        let line_bytes = line.as_bytes();
        let to_check: Vec<_> = beam_cols
            .iter()
            .map(|pos| (*pos, line_bytes[*pos] as char))
            .collect();

        for (idx, c) in to_check {
            if c == SPLITTER {
                split_count += 1;
                beam_cols.remove(&idx);
                beam_cols.insert(idx - 1);
                beam_cols.insert(idx + 1);
            }
        }
    }

    Ok(split_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part_one() {
        let expected = 21;
        let actual = _part_one(INPUT.as_bytes()).unwrap();
        assert_eq!(expected, actual);
    }
}
