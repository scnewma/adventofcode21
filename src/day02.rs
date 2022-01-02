use crate::SolveInfo;

pub(crate) fn run(input: &str) -> anyhow::Result<SolveInfo> {
    let commands = parse_input(input)?;
    Ok(SolveInfo {
        challenge: "Dive!",
        part01: part01(&commands)?,
        part02: part02(&commands)?,
    })
}

enum Command {
    Forward(i64),
    Up(i64),
    Down(i64),
}

fn part01(commands: &[Command]) -> anyhow::Result<i64> {
    let mut hor = 0;
    let mut depth = 0;
    for command in commands {
        match command {
            Command::Forward(n) => hor += n,
            Command::Up(n) => depth -= n,
            Command::Down(n) => depth += n,
        }
    }
    Ok(hor * depth)
}

fn part02(commands: &[Command]) -> anyhow::Result<i64> {
    let mut hor = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in commands {
        match command {
            Command::Forward(n) => {
                hor += n;
                depth += aim * n;
            }
            Command::Up(n) => aim -= n,
            Command::Down(n) => aim += n,
        }
    }
    Ok(hor * depth)
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Command>> {
    let mut commands = Vec::new();
    for line in input.lines() {
        let (cmd, units) = line.split_once(' ').unwrap();
        let units: i64 = units.parse()?;

        let command = match cmd {
            "forward" => Command::Forward(units),
            "up" => Command::Up(units),
            "down" => Command::Down(units),
            _ => panic!("unknown command {}", cmd),
        };
        commands.push(command);
    }
    Ok(commands)
}
