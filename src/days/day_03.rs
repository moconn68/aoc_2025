use std::io::BufRead;

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

fn _part_two(_input: impl BufRead) -> anyhow::Result<usize> {
    todo!()
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
}
