use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::read;

use clap::{Parser, ValueEnum};

extern crate proc_macro;
use std::collections::HashMap;

use seq_macro::seq;

seq!(DAY in 01..=31 {
    use aoc_rs::solutions::y2024::day~DAY;
});

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, ValueEnum)]
pub enum PuzzlePart {
    A,
    B,
}

impl Display for PuzzlePart {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            PuzzlePart::A => write!(f, "a"),
            PuzzlePart::B => write!(f, "b"),
        }
    }
}

type SolutionMap = HashMap<(u8, PuzzlePart), fn(&[u8]) -> u32>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct PuzzleSpec {
    #[arg(short, long, default_value_t = 2024)]
    year: u16,

    #[arg(short, long)]
    day: u8,

    #[arg(short, long)]
    part: PuzzlePart,
}

fn get_solution_map() -> SolutionMap {
    let mut solutions = HashMap::new();
    seq!(DAY in 01..=31 {
        solutions.insert((DAY, PuzzlePart::A), day~DAY::solve_a as fn(&[u8]) -> u32);
        solutions.insert((DAY, PuzzlePart::B), day~DAY::solve_b as fn(&[u8]) -> u32);
    });
    solutions
}

fn main() {
    let spec = PuzzleSpec::parse();
    let input = read(format!("data/y{:04}/day{:02}.txt", spec.year, spec.day)).unwrap();
    let solution_map = get_solution_map();
    let solution_fn = solution_map.get(&(spec.day, spec.part)).unwrap();
    println!(
        "{} day{:02}{}: {}",
        spec.year,
        spec.day,
        &spec.part,
        solution_fn(&input)
    );
}
