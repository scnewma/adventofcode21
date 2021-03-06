use crate::SolveInfo;

pub(crate) fn run(input: &str) -> anyhow::Result<SolveInfo> {
    let input = parse_input(input)?;

    Ok(SolveInfo {
        challenge: "Lanternfish",
        part01: part01(&input),
        part02: part02(&input),
    })
}

fn part01(initial_pop: &[i64]) -> i64 {
    simulate(initial_pop, 80)
}

fn part02(initial_pop: &[i64]) -> i64 {
    simulate(initial_pop, 256)
}

fn simulate(inital_pop: &[i64], days: i64) -> i64 {
    // pop represents the population of lanternfish by grouping lanternfish into groups by age
    let mut pop = [0; 9];

    // add initial population to aged population array
    inital_pop.iter().for_each(|i| pop[*i as usize] += 1);

    // simulate days
    (0..days).for_each(|_| {
        pop.rotate_left(1);
        pop[6] += pop[8];
    });

    pop.iter().sum()
}

fn parse_input(input: &str) -> anyhow::Result<Vec<i64>> {
    Ok(input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect())
}
