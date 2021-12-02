fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/01-sonar-sweep.txt")?;
    let input = parse_input(&input)?;

    // part 1
    let increases = count_increases(&input)?;
    println!("1: {}", increases);

    // part 2
    let increases = windowed_increases(&input)?;
    println!("2: {}", increases);
    Ok(())
}

fn windowed_increases(input: &[i64]) -> anyhow::Result<i64> {
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

fn count_increases(input: &[i64]) -> anyhow::Result<i64> {
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
