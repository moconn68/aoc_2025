use std::io::BufRead;

use itertools::Itertools;

use crate::puzzle::Puzzle;

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

fn part_two(input: Box<dyn BufRead>) -> anyhow::Result<crate::puzzle::Answer> {
    let ans = _part_two(input)?;
    Ok(Box::new(ans))
}

fn _part_one(input: impl BufRead) -> anyhow::Result<u32> {
    input.lines().try_fold(0, |acc, line| {
        let line = line?;

        // (idx, el)
        let mut left = (0, 0);

        let digits = line.chars().filter_map(|c| c.to_digit(10));
        let digits: Vec<_> = digits
            .enumerate()
            .map(|(idx, el)| {
                if el > left.1 && idx != line.len() - 1 {
                    left = (idx, el);
                }
                el
            })
            .collect();
        let right = *digits
            .get(left.0 + 1..line.len())
            .and_then(|digits| digits.iter().max())
            .unwrap();

        let num = left.1 * 10 + right;

        Ok(acc + num)
    })
}

fn _part_two(input: impl BufRead) -> anyhow::Result<usize> {
    input
        .lines()
        .map(|line| {
            let mut line: Vec<_> = line?.chars().filter_map(|c| c.to_digit(10)).collect();
            Ok(max_joltage(&mut line))
        })
        .sum()
}

fn max_joltage(bank: &mut Vec<u32>) -> usize {
    while bank.len() > 12 {
        let remove_idx = bank
            .windows(2)
            .find_position(|pred| pred[0] < pred[1])
            .map(|x| x.0)
            .unwrap_or(bank.len() - 1);
        bank.remove(remove_idx);
    }

    bank.iter().fold(0, |acc, el| acc * 10 + *el as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part_one() {
        let expected = 357;
        let actual = _part_one(INPUT.as_bytes()).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = 3121910778619;
        let actual = _part_two(INPUT.as_bytes()).unwrap();
        assert_eq!(expected, actual);
    }
}
