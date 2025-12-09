use crate::puzzle::Puzzle;

use std::{collections::VecDeque, io::BufRead, str::FromStr};

use anyhow::Context;

pub(crate) fn get_puzzle() -> crate::puzzle::Puzzle {
    Puzzle {
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    }
}

struct Input<I> {
    intervals: VecDeque<Interval>,
    ids: I,
}

struct Interval {
    start: usize,
    end: usize,
}

impl FromStr for Interval {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once('-')
            .context("Interval format should always be {start}-{end}")?;
        let start: usize = start
            .parse()
            .context("Interval start should be valid usize")?;
        let end: usize = end.parse().context("Interval end should be valid usize")?;

        Ok(Self { start, end })
    }
}

fn parse_input(raw: impl BufRead) -> Input<impl Iterator<Item = usize>> {
    let mut lines = raw.lines();

    let mut intervals = VecDeque::new();
    while let interval_raw = lines
        .next()
        .expect("Input should have content for interval")
        .expect("Should be able to read valid line for interval")
        // Empty line denotes input's separation between intervals and IDs
        && !interval_raw.is_empty()
    {
        let interval = interval_raw.parse().unwrap();
        intervals.push_back(interval);
    }

    // The rest of the contents of the input is just the IDs
    let ids = lines.map(|line| {
        line.expect("Should read valid line for ID")
            .parse()
            .expect("Input ID line should contain only a single valid usize")
    });

    Input { intervals, ids }
}

fn part_one(input: Box<dyn BufRead>) -> anyhow::Result<crate::puzzle::Answer> {
    let input = parse_input(input);
    let ans = _part_one(input);
    Ok(Box::new(ans))
}

fn _part_one<I: Iterator<Item = usize>>(input: Input<I>) -> usize {
    input
        .ids
        .filter(|id| {
            for interval in &input.intervals {
                if (interval.start..=interval.end).contains(id) {
                    return true;
                }
            }
            false
        })
        .count()
}

fn part_two(input: impl BufRead) -> anyhow::Result<crate::puzzle::Answer> {
    let Input { intervals, .. } = parse_input(input);
    Ok(Box::new(_part_two(intervals)))
}

fn _part_two(mut input: VecDeque<Interval>) -> usize {
    // Merge overlapping intervals
    input.make_contiguous().sort_by_key(|r| r.start);
    let mut ret = vec![input.pop_front().unwrap()];
    for cur in input {
        let prev = ret.last_mut().unwrap();
        if cur.start <= prev.end {
            prev.end = std::cmp::max(prev.end, cur.end)
        } else {
            ret.push(cur);
        }
    }

    ret.into_iter()
        .map(|range| range.end - range.start + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part_one() {
        let expected = 3;
        let input = parse_input(INPUT.as_bytes());
        let actual = _part_one(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = 14;
        let input = parse_input(INPUT.as_bytes());
        let actual = _part_two(input.intervals);
        assert_eq!(expected, actual);
    }
}
