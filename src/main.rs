mod cli;
mod days;
mod puzzle;

use cli::Cli;

use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let Cli { day, part } = cli;

    let input = Box::new(days::get_input_for_day(day)?);

    let puzzle = days::select_puzzle(day)?;
    let ans = match part {
        1 => (puzzle.part_one)(input),
        2 => (puzzle.part_two)(input),
        _ => unreachable!("There should only be two parts to each puzzle"),
    }?;

    println!("Answer for Day {day}, Part {part}: {ans}");

    Ok(())
}
