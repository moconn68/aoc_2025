use clap::Parser;

/// CLI runner for my solutions to AoC 2025, written in Rust.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Which day you want to run
    #[arg(value_parser = clap::value_parser!(u8).range(1..=12))]
    pub day: u8,
    /// Which part you want to run
    #[arg(value_parser = clap::value_parser!(u8).range(1..=2))]
    pub part: u8,
}
