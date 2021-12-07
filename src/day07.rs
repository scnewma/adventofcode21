use crate::SolveInfo;

pub(crate) fn run(input: &str) -> anyhow::Result<SolveInfo> {
    let input = parse_input(input);

    Ok(SolveInfo {
        challenge: "The Treachery of Whales",
        part01: part01(&input),
        part02: part02(&input),
    })
}

fn part01(positions: &[i64]) -> i64 {
    solve(positions, |start, end| end - start)
}

fn part02(positions: &[i64]) -> i64 {
    // adjusts the range to begin at 1 then calculates the sum of integers
    solve(positions, |start, end| {
        let a = 1;
        let i = end - start; // adjustment for 1-based range
        let n = i;
        (n * (a + i)) / 2
    })
}

fn solve<F>(positions: &[i64], fuel_calc: F) -> i64
where
    F: Fn(i64, i64) -> i64,
{
    let min_pos = *positions.iter().min().unwrap();
    let max_pos = *positions.iter().max().unwrap();
    let mut min_fuel = i64::MAX;
    (min_pos..max_pos).for_each(|desired_pos| {
        let mut fuel = 0;
        positions.iter().for_each(|crab_pos| {
            let start = desired_pos.min(*crab_pos);
            let end = desired_pos.max(*crab_pos);
            fuel += fuel_calc(start, end);
        });
        min_fuel = min_fuel.min(fuel);
    });
    min_fuel
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}
