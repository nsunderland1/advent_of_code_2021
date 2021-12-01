use std::path::{Path, PathBuf};

use aoc::*;
use itertools::Itertools;

/// The path to the puzzle's input file
pub fn input_path() -> PathBuf {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    crate_root.join("input")
}

fn parse_line(s: String) -> u32 {
    s.parse().unwrap()
}

fn main() {
    let input: Vec<_> = read_lines(input_path()).map(parse_line).collect();

    println!(
        "Part 1: {}",
        input.iter().tuple_windows().filter(|(l, r)| l < r).count()
    );

    println!(
        "Part 2: {}",
        input
            .iter()
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .tuple_windows()
            .filter(|(l, r)| l < r)
            .count()
    );
}
