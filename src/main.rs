use anyhow::{Context, Result};
use structopt::StructOpt;

mod challenge;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

#[derive(Debug, StructOpt)]
#[structopt(name = "adventofcode", about = "Advent of Code solutions: 2021")]
struct Opt {
    #[structopt(name = "DAY")]
    day: Option<usize>,

    #[structopt(about = "Use test input instead of full input.", short, long)]
    test: bool,
}

struct SolveInfo {
    challenge: &'static str,
    part01: i64,
    part02: i64,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let days = [
        day01::run,
        day02::run,
        day03::run,
        day04::run,
        day05::run,
        day06::run,
        day07::run,
        day08::run,
        day09::run,
        day10::run,
        day11::run,
    ];

    if let Some(day) = opt.day {
        if day > days.len() {
            anyhow::bail!("Day {} not yet solved!", day)
        }

        let input = day_input(day, opt.test)?;
        let f = days[day - 1];
        let solve = f(&input)?;
        print_solve(day, solve);
    } else {
        for (day, f) in days.iter().enumerate() {
            let day = day + 1;
            let input = day_input(day, opt.test)?;
            let solve = f(&input)?;
            print_solve(day, solve);
            println!();
        }
    }

    Ok(())
}

fn day_input(day: usize, test_input: bool) -> Result<String> {
    let fname = if test_input {
        format!("inputs/{:0>2}.test.txt", day)
    } else {
        format!("inputs/{:0>2}.txt", day)
    };
    std::fs::read_to_string(&fname).with_context(|| format!("Reading file {}", fname))
}

fn print_solve(day: usize, solve: SolveInfo) {
    println!("--- Day {:02}: {} ---", day, solve.challenge);
    println!("  Part 1: {}", solve.part01);
    println!("  Part 2: {}", solve.part02);
}
