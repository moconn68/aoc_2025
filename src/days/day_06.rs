use anyhow::Context;

use crate::puzzle::Puzzle;

use std::{
    io::BufRead,
    ops::{Add, Mul},
};

pub(crate) fn get_puzzle() -> crate::puzzle::Puzzle {
    Puzzle {
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    }
}

fn part_one(mut input: Box<dyn BufRead>) -> anyhow::Result<crate::puzzle::Answer> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let parsed_input = ParsedInput::from_raw_input(buf)?;
    let ans = _part_one(parsed_input);
    Ok(Box::new(ans))
}

fn _part_one(input: ParsedInput) -> usize {
    input
        .columns
        .into_iter()
        .map(|col| {
            col.nums
                .into_iter()
                .reduce(|acc, el| match col.op {
                    Operator::Add => acc + el,
                    Operator::Mul => acc * el,
                })
                .expect("Input should not be empty")
        })
        .sum()
}

fn part_two(input: Box<dyn BufRead>) -> anyhow::Result<crate::puzzle::Answer> {
    let ans = _part_two(input)?;
    Ok(Box::new(ans))
}

fn _part_two(input: impl BufRead) -> anyhow::Result<usize> {
    let mut ans: usize = 0;
    let mut input_lines: Vec<Vec<char>> = input
        .lines()
        .map(|line_string| line_string.map(|line| line.chars().collect()))
        .collect::<Result<_, _>>()?;
    let mut ops = input_lines
        .pop()
        .expect("Input shouldn't be empty")
        .into_iter()
        .filter(|pat| !pat.is_whitespace())
        .map(Operator::from);

    let mut cur_op = ops.next().context("Operators line is empty")?;
    let mut working_column = vec![];
    for c in 0..input_lines[0].len() {
        let mut col_digits = vec![];
        for row in &input_lines {
            let col_digit = match row[c] {
                empty if empty.is_whitespace() => None,
                digit => digit.to_digit(10).map(|x| x as usize),
            };
            col_digits.push(col_digit);
        }
        let digits_to_sum = col_digits.into_iter().flatten().collect::<Vec<_>>();
        if digits_to_sum.is_empty() {
            // whitespace gap between columns - perform op & advance
            ans += std::mem::take(&mut working_column)
                .into_iter()
                .reduce(|acc, el| cur_op.apply(acc, el))
                .context("Problem buffer is empty")?;

            cur_op = ops
                .next()
                .context("Not enough operators for provided numbers")?;
        } else {
            let val = digits_to_sum.into_iter().fold(0, |acc, el| acc * 10 + el);
            working_column.push(val);
        }
    }

    // Once more for the final problem
    ans += working_column
        .into_iter()
        .reduce(|acc, el| cur_op.apply(acc, el))
        .expect("Should always be Some, given the input format");

    Ok(ans)
}

struct ParsedInput {
    columns: Vec<Column>,
}

struct Column {
    nums: Vec<usize>,
    op: Operator,
}

enum Operator {
    Add,
    Mul,
}

impl Operator {
    fn apply<L, R, O>(&self, lhs: L, rhs: R) -> O
    where
        L: Add<R, Output = O>,
        L: Mul<R, Output = O>,
    {
        match self {
            Self::Add => lhs + rhs,
            Self::Mul => lhs * rhs,
        }
    }
}

impl From<char> for Operator {
    fn from(value: char) -> Self {
        match value {
            '+' => Self::Add,
            '*' => Self::Mul,
            _ => unreachable!("For this puzzle, only add and mul should be present"),
        }
    }
}

impl From<&str> for Operator {
    fn from(value: &str) -> Self {
        match value {
            "+" => Self::Add,
            "*" => Self::Mul,
            _ => unreachable!("For this puzzle, only add and mul should be present"),
        }
    }
}

impl ParsedInput {
    fn from_raw_input(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let input = input.as_ref();
        let mut lines_iter = input.lines().map(|line| line.split_whitespace()).rev();
        let ops_iter = lines_iter.next().unwrap().map(Operator::from);

        let mut nums_list: Vec<Vec<usize>> = vec![];
        let nums_iter = lines_iter.rev().map(|num_str| {
            num_str
                .into_iter()
                .map(|el| el.parse::<usize>().unwrap())
                .enumerate()
        });
        for num_line in nums_iter {
            for (col_num, num) in num_line {
                match nums_list.get_mut(col_num) {
                    Some(list) => list.push(num),
                    None => nums_list.insert(col_num, vec![num]),
                }
            }
        }

        let columns = ops_iter
            .enumerate()
            .map(|(idx, op)| Column {
                nums: std::mem::take(&mut nums_list[idx]),
                op,
            })
            .collect();

        Ok(Self { columns })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part_one() {
        let expected = 4277556;
        let parsed_input = ParsedInput::from_raw_input(INPUT).unwrap();
        let actual = _part_one(parsed_input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = 3263827;
        let actual = _part_two(INPUT.as_bytes()).unwrap();
        assert_eq!(expected, actual);
    }
}
