use crate::puzzle::Puzzle;

use std::io::BufRead;

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
                .unwrap()
        })
        .sum()
}

fn part_two(_input: Box<dyn BufRead>) -> anyhow::Result<crate::puzzle::Answer> {
    todo!()
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

impl ParsedInput {
    fn from_raw_input(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let input = input.as_ref();
        let mut lines_iter = input.lines().map(|line| line.split_whitespace()).rev();
        let ops_iter = lines_iter.next().unwrap().map(|el| match el {
            "+" => Operator::Add,
            "*" => Operator::Mul,
            _ => unreachable!("Input should only contain add or mul operators"),
        });

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
}
