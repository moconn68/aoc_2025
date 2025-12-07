use std::io::BufRead;

use crate::puzzle::Puzzle;

pub(crate) fn get_puzzle() -> crate::puzzle::Puzzle {
    Puzzle {
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    }
}

fn parse_input(input: impl BufRead) -> std::io::Result<Vec<Vec<Space>>> {
    input
        .lines()
        .map(|line| Ok(line?.chars().map(Space::from).collect()))
        .collect()
}

fn part_one(input: Box<dyn BufRead>) -> anyhow::Result<crate::puzzle::Answer> {
    let matrix = parse_input(input)?;
    let ans = _part_one(matrix);
    Ok(Box::new(ans))
}

fn part_two(input: Box<dyn BufRead>) -> anyhow::Result<crate::puzzle::Answer> {
    let ans = _part_two(input)?;
    Ok(Box::new(ans))
}

fn _part_one<T, U>(input: T) -> usize
where
    T: AsRef<[U]>,
    U: AsRef<[Space]>,
{
    let input = input.as_ref();
    input
        .iter()
        .enumerate()
        .flat_map(|(ridx, row)| {
            row.as_ref()
                .iter()
                .enumerate()
                .filter(move |(cidx, space)| {
                    if !matches!(space, Space::Roll) {
                        return false;
                    }

                    let adjacent_pos = [
                        (ridx.checked_sub(1), cidx.checked_sub(1)),
                        (ridx.checked_sub(1), Some(*cidx)),
                        (ridx.checked_sub(1), Some(cidx + 1)),
                        (Some(ridx), cidx.checked_sub(1)),
                        (Some(ridx), Some(cidx + 1)),
                        (Some(ridx + 1), cidx.checked_sub(1)),
                        (Some(ridx + 1), Some(*cidx)),
                        (Some(ridx + 1), Some(cidx + 1)),
                    ]
                    .into_iter()
                    .filter_map(|(a, b)| a.zip(b));

                    let adjacent_rolls = adjacent_pos.filter(|(r, c)| {
                        let pos_space = input.get(*r).and_then(|r| r.as_ref().get(*c));

                        pos_space.is_some_and(|space| space == &Space::Roll)
                    });

                    adjacent_rolls.count() < 4
                })
        })
        .count()
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Space {
    Roll,
    Empty,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '@' => Self::Roll,
            '.' => Self::Empty,
            _ => unreachable!("Input should only contain '@' or '.' characters in a line"),
        }
    }
}

fn _part_two(_input: impl BufRead) -> anyhow::Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part_one() {
        let expected = 13;
        let input_vec = parse_input(INPUT.as_bytes()).unwrap();
        let actual = _part_one(&input_vec);
        assert_eq!(expected, actual);
    }
}
