use std::{collections::HashSet, io::BufRead, ops::RangeInclusive};

use crate::puzzle::Puzzle;

pub(crate) fn get_puzzle() -> crate::puzzle::Puzzle {
    Puzzle {
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    }
}

struct Input<I> {
    ranges: HashSet<RangeInclusive<usize>>,
    ids: I,
}

fn parse_input(raw: impl BufRead) -> Input<impl Iterator<Item = usize>> {
    let mut lines = raw.lines();

    let mut ranges = HashSet::new();
    while let range = lines
        .next()
        .expect("Input should have content for ranges")
        .expect("Should be able to read valid line for range")
        && !range.is_empty()
    {
        let (start, end) = range
            .split_once('-')
            .expect("Range format should always be {start}-{end}");
        let start: usize = start.parse().expect("Range start should be valid usize");
        let end: usize = end.parse().expect("Range end should be valid usize");

        ranges.insert(start..=end);
    }

    // The rest of the contents of the input is just the IDs
    let ids = lines.map(|line| {
        line.expect("Should read valid line for ID")
            .parse()
            .expect("Input ID line should contain only a single valid usize")
    });

    Input { ranges, ids }
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
            for range in &input.ranges {
                if range.contains(id) {
                    return true;
                }
            }
            false
        })
        .count()
}

fn part_two(_input: impl BufRead) -> anyhow::Result<crate::puzzle::Answer> {
    todo!()
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
}
