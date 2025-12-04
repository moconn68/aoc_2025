use std::io::BufRead;

use anyhow::Context;
use itertools::Itertools;

use crate::puzzle::Puzzle;

pub(crate) fn get_puzzle() -> crate::puzzle::Puzzle {
    Puzzle {
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    }
}

fn part_one(input: Box<dyn BufRead>) -> anyhow::Result<crate::puzzle::Answer> {
    // This day's puzzle input is a single line
    let input: String = input.lines().collect::<Result<_, _>>()?;
    let ans = _part_one(&input)?;
    Ok(Box::new(ans))
}

fn part_two(input: Box<dyn BufRead>) -> anyhow::Result<crate::puzzle::Answer> {
    // This day's puzzle input is a single line
    let input: String = input.lines().collect::<Result<_, _>>()?;
    let ans = _part_two(&input)?;
    Ok(Box::new(ans))
}

fn _part_one(input: &str) -> anyhow::Result<usize> {
    let mut invalid_sum = 0;

    let ranges = input.split(',');
    for range in ranges {
        let (min, max) = range
            .split_once('-')
            .context("Malformed range, missing '-' separator")?;

        let min: usize = min.parse()?;
        let max: usize = max.parse()?;

        for num in min..=max {
            let num_str = num.to_string();
            if num_str.len() % 2 != 0 {
                // Can't evenly split a number that doesn't have an even number of digits
                continue;
            }
            let (front, back) = num_str.split_at(num_str.len() / 2);
            if front == back {
                invalid_sum += num;
            }
        }
    }

    Ok(invalid_sum)
}

fn _part_two(input: &str) -> anyhow::Result<usize> {
    let mut invalid_sum = 0;

    let ranges = input.split(',');
    for range in ranges {
        let (min, max) = range
            .split_once('-')
            .context("Malformed range, missing '-' separator")?;

        let min: usize = min.parse()?;
        let max: usize = max.parse()?;

        for num in min..=max {
            let num_str = num.to_string();
            // Iterate through all possible pattern lengths for the number
            let chunk_sizes = (1..=num_str.len() / 2)
                // Omit those that don't divide the number into whole groups
                .filter(|chunk_size| num_str.len() % chunk_size == 0);

            for size in chunk_sizes {
                let chunks = num_str.chars().chunks(size);
                let mut chunks = chunks.into_iter().map(Vec::from_iter);
                if chunks.all_equal() {
                    invalid_sum += num;
                    break;
                }
            }
        }
    }

    Ok(invalid_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_one() {
        let expected = 1227775554;
        let actual = _part_one(INPUT).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = 4174379265;
        let actual = _part_two(INPUT).unwrap();
        assert_eq!(expected, actual);
    }
}
