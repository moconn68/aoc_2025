# Advent of Code 2025

My solutions for AoC 2025, written in Rust.

## Running the Solutions

This package is used via the `cli` module, which leverages the [`clap` crate](https://docs.rs/clap/latest/clap/) for a very simple command line interface. After you've [installed Rust](https://rust-lang.org/learn/get-started/), you would run this as follows:

```
cargo run -- <DAY> <PART>
```
The pre-requisite for this is to make sure that you've obtained the input file for the `<DAY>` that you want to run, and placed it within the `inputs` directory named as `day_<DAY>`. __Note__ that if `<DAY>` is a single-digit, you'll need to pad it with a leading 0 ie `day_01`.

## Testing

When testing the code prior to running against actual input, you may simply run `cargo test`. If you want to limit it to tests for a specific day, just run `cargo test day_<DAY>`. Again, you should prefix single-digit day numbers with a 0, in accordance with the file naming conventions in the repository.

## Repository Layout

I tried to quickly come up with a structure that was relatively equally split between simplicity, code re-use, adherence to Rust language idioms, and quick to set up. Compromises in all areas were necessary, but ultimately I think that the end result is a decent layout.

### 'Puzzle' Definition

The definitions in `puzzle.rs` contain the high-level types for what we expect out of a given puzzle. Put simply, a given "puzzle" is something that has two parts, and each part is some runnable that takes some buffered input and returns something that we can print to the user.

### 'Days' Module Structure

The code for the actual solutions themselves lives within the `days` module. Each solution will live within a `day_<DAY>` submodule file (again, prefixed 0s for single-digit day numbers).

The top-level public construct here must adhere to the aforementioned `Puzzle` definition, which is dynamically obtained via each module's `get_puzzle` free function. The actual solutions will typically be implemented in distinct private functions, which are then called at the higher-level abstractions and `Box`-ed, `Ok`-ed, etc.

The actual `mod.rs` file for the `days` module contains the "selector" logic for automatically getting the correct input and solver for the given day. This is mostly manual, and will need to be updated as new solutions are added for each solved puzzle day.