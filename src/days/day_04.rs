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
    let ans = _part_one(&matrix);
    Ok(Box::new(ans))
}

fn part_two(input: Box<dyn BufRead>) -> anyhow::Result<crate::puzzle::Answer> {
    let mut matrix = parse_input(input)?;
    let ans = _part_two(&mut matrix);
    Ok(Box::new(ans))
}

fn _part_one(input: &[Vec<Space>]) -> usize {
    get_candidate_positions(input).count()
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

fn _part_two(input: &mut [Vec<Space>]) -> usize {
    let mut total = 0;
    while let to_remove = get_candidate_positions(input).collect::<Vec<_>>()
        && !to_remove.is_empty()
    {
        total += to_remove.len();
        for (ridx, cidx) in to_remove {
            input[ridx][cidx] = Space::Empty;
        }
    }
    total
}

/// For a given matix state, obtain the positions which are candidates for removal
/// per the problem's definition (there are fewer than four rolls of paper in the eight adjacent positions).
fn get_candidate_positions(input: &[Vec<Space>]) -> impl Iterator<Item = (usize, usize)> {
    input.iter().enumerate().flat_map(move |(ridx, row)| {
        row.iter().enumerate().filter_map(move |(cidx, space)| {
            if !matches!(space, Space::Roll) {
                return None;
            }

            let adjacent_pos = get_adjacent_positions_checked(ridx, cidx);

            let adjacent_rolls = adjacent_pos.filter(|(r, c)| {
                let pos_space = input.get(*r).and_then(|r| r.get(*c));

                pos_space.is_some_and(|space| space == &Space::Roll)
            });

            if adjacent_rolls.count() < 4 {
                Some((ridx, cidx))
            } else {
                None
            }
        })
    })
}

/// In a 2D matrix, for any given position (ridx, cidx), return the indices in the matrix which are
/// adjacent to it. This uses checked subtration to filter out any invalid negative indices.
fn get_adjacent_positions_checked(
    ridx: usize,
    cidx: usize,
) -> impl Iterator<Item = (usize, usize)> {
    [
        (ridx.checked_sub(1), cidx.checked_sub(1)),
        (ridx.checked_sub(1), Some(cidx)),
        (ridx.checked_sub(1), Some(cidx + 1)),
        (Some(ridx), cidx.checked_sub(1)),
        (Some(ridx), Some(cidx + 1)),
        (Some(ridx + 1), cidx.checked_sub(1)),
        (Some(ridx + 1), Some(cidx)),
        (Some(ridx + 1), Some(cidx + 1)),
    ]
    .into_iter()
    .filter_map(|(a, b)| a.zip(b))
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

    #[test]
    fn test_part_two() {
        let expected = 43;
        let mut input_vec = parse_input(INPUT.as_bytes()).unwrap();
        let actual = _part_two(&mut input_vec);
        assert_eq!(expected, actual);
    }
}
