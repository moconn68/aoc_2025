mod day_01;
mod day_02;
mod day_03;
mod day_04;

use crate::puzzle::Puzzle;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

const INPUTS_DIR_NAME: &str = "inputs";
const INPUT_FILE_NAME_PREFIX: &str = "day_";
const INPUT_FILE_EXTENSION: &str = "txt";

/// Get the input data for the provided day, in the form of a [`BufRead`].
///
/// This data must already exist on disk in the `inputs/` directory, named
/// as `day_N` where `N` equals the `day` argument. If N < 10, it the input
/// file name should have a leading 0 ie `day_01`.
pub fn get_input_for_day(day: u8) -> std::io::Result<impl BufRead> {
    let root_dir = std::env!("CARGO_MANIFEST_DIR");
    let mut input_path = PathBuf::new();
    input_path.push(root_dir);
    input_path.push(INPUTS_DIR_NAME);
    input_path.push(format!("{INPUT_FILE_NAME_PREFIX}{day:02}"));
    input_path.set_extension(INPUT_FILE_EXTENSION);

    File::open(input_path).map(BufReader::new)
}

/// Obtain the [`Puzzle`] for the given `day` 1-12.
///
/// If that day's puzzle has not yet been implemented, will return an error.
pub fn select_puzzle(day: u8) -> anyhow::Result<Puzzle> {
    Ok(match day {
        1 => day_01::get_puzzle(),
        2 => day_02::get_puzzle(),
        3 => day_03::get_puzzle(),
        4 => day_04::get_puzzle(),
        // TODO: add more implementations for each new day
        _ => anyhow::bail!("Day {day} is not yet completed!"),
    })
}
