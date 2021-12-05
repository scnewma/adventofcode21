use crate::SolveInfo;

pub(crate) fn run(input: &str) -> anyhow::Result<SolveInfo> {
    let input = parse_input(&input)?;

    Ok(SolveInfo {
        challenge: "Sonar Sweep",
        part01: part01(&input)?,
        part02: part02(&input)?,
    })
}

fn part02(input: &[i64]) -> anyhow::Result<i64> {
    let mut prev_win = input[0] + input[1] + input[2];
    let mut inc = 0;
    for i in 1..(input.len() - 2) {
        let sum = input[i] + input[i + 1] + input[i + 2];
        if sum > prev_win {
            inc += 1;
        }
        prev_win = sum
    }
    Ok(inc)
}

fn part01(input: &[i64]) -> anyhow::Result<i64> {
    let mut prev = &input[0];
    let mut inc = 0;
    for n in input.iter().skip(1) {
        if n > &prev {
            inc += 1;
        }
        prev = n;
    }
    Ok(inc)
}

fn parse_input(input: &str) -> anyhow::Result<Vec<i64>> {
    let mut nums = Vec::new();
    for line in input.lines() {
        let n: i64 = line.parse()?;
        nums.push(n);
    }
    Ok(nums)
}
