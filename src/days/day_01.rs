use std::{
    io::BufRead,
    ops::{Add, Sub},
    str::FromStr,
};

use anyhow::Context;

use crate::puzzle::Puzzle;

pub fn get_puzzle() -> Puzzle {
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
    let ans = _part_two(input);
    Ok(Box::new(ans))
}

fn _part_one(input: impl BufRead) -> anyhow::Result<u32> {
    let mut pointer = SafeNum::default();
    let mut zero_ct = 0;

    for line in input.lines() {
        let line = line?;
        let (dir, mag) = line.split_at(1);

        let mag = mag
            .parse::<SafeNum>()
            .context("Could not parse valid magnitude int")?;

        match dir {
            "L" => pointer = pointer - mag,
            "R" => pointer = pointer + mag,
            _ => unreachable!("dir should be only L or R; actually is {}", dir),
        };

        if pointer.0 == 0 {
            zero_ct += 1;
        }
    }

    Ok(zero_ct)
}

#[derive(Clone, Copy, Debug)]
struct SafeNum(u8);

impl Default for SafeNum {
    fn default() -> Self {
        Self(50)
    }
}

impl SafeNum {
    /// Max value to bound rotation
    const MAX: u8 = 100;
}

impl FromStr for SafeNum {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_int = s
            .parse::<usize>()
            .context("Cannot parse SafeNum candidate string as number")?;
        let wrapped = raw_int % Self::MAX as usize;
        Ok(Self(wrapped as u8))
    }
}

impl Add for SafeNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = (self.0 + rhs.0) % Self::MAX;
        Self(sum)
    }
}

impl Sub for SafeNum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = if self.0 >= rhs.0 {
            self.0 - rhs.0
        } else {
            Self::MAX - (rhs.0 - self.0)
        };
        Self(diff)
    }
}

fn _part_two(_input: impl BufRead) -> i32 {
    todo!("Actual logic implementation goes here")
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part_one() {
        let expected: u32 = 3;
        let input_reader = BufReader::new(TEST_INPUT.as_bytes());
        let actual = _part_one(input_reader).unwrap();
        assert_eq!(expected, actual);
    }
}
