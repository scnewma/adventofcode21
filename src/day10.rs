use crate::SolveInfo;

pub(crate) fn run(input: &str) -> anyhow::Result<SolveInfo> {
    let input = parse_input(input);

    Ok(SolveInfo {
        challenge: "Syntax Scoring",
        part01: part01(&input),
        part02: part02(&input),
    })
}

fn part01(lines: &[&str]) -> i64 {
    lines
        .iter()
        .map(|l| process_line(l))
        // drop incomplete lines
        .filter_map(|line_type| match line_type {
            LineType::Invalid(c) => Some(c),
            _ => None,
        })
        // calculate score
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        })
        .sum()
}

fn part02(lines: &[&str]) -> i64 {
    let mut scores: Vec<i64> = lines
        .iter()
        .map(|l| process_line(l))
        // drop invalid lines
        .filter_map(|line_type| match line_type {
            LineType::Incomplete(ending) => Some(ending),
            _ => None,
        })
        // calculate score
        .map(|ending_chars| {
            ending_chars.iter().fold(0, |acc, c| {
                acc * 5
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    }
            })
        })
        .collect();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[derive(PartialEq)]
enum LineType {
    Invalid(char),
    Incomplete(Vec<char>),
}

fn process_line(line: &str) -> LineType {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            closing => {
                if closing != pair(&stack.pop().unwrap()) {
                    return LineType::Invalid(c);
                }
            }
        }
    }

    let ending = stack.iter().rev().map(|opening| pair(opening)).collect();
    LineType::Incomplete(ending)
}

fn pair(c: &char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}
