use crate::SolveInfo;

pub(crate) fn run(input: &str) -> anyhow::Result<SolveInfo> {
    let (report, bit_width) = parse_input(&input);

    let part01 = part01(&report, bit_width)?;
    let part02 = part02(&report, bit_width);

    Ok(SolveInfo {
        challenge: "Binary Diagnostic",
        part01,
        part02,
    })
}

fn parse_input(input: &str) -> (Vec<u16>, usize) {
    let report: Vec<u16> = input
        .lines()
        .map(|line| u16::from_str_radix(line, 2).expect("failed to parse line"))
        .collect();
    let bit_width = input.lines().take(1).next().unwrap().len();
    (report, bit_width)
}

fn part01(report: &Vec<u16>, width: usize) -> anyhow::Result<i64> {
    let mut gamma: u16 = 0;
    let mut epsilon: u16 = 0;

    for i in (0..width).rev() {
        let mut ones = 0;
        let mut zeros = 0;

        for num in report {
            let bit = num >> i & 1;
            if bit == 1 {
                ones += 1;
            } else {
                zeros += 1;
            }
        }

        if ones > zeros {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }

    Ok(gamma as i64 * epsilon as i64)
}

fn part02(report: &Vec<u16>, width: usize) -> i64 {
    oxygen_generator_rating(report, width) as i64 * co2_scrubber_rating(report, width) as i64
}

fn find_rating(report: &Vec<u16>, width: usize, cond: Cond) -> u16 {
    let mut report = report.clone();
    for i in (0..width).rev() {
        let mut ones = 0;
        let mut zeros = 0;

        for num in report.iter() {
            let bit = num >> i & 1;
            if bit == 1 {
                ones += 1;
            } else {
                zeros += 1;
            }
        }

        match cond {
            Cond::MostCommon => {
                if ones > zeros || ones == zeros {
                    report = report.into_iter().filter(|num| num >> i & 1 == 1).collect();
                } else {
                    report = report.into_iter().filter(|num| num >> i & 1 == 0).collect();
                }
            }
            Cond::LeastCommon => {
                if ones < zeros {
                    report = report.into_iter().filter(|num| num >> i & 1 == 1).collect();
                } else {
                    report = report.into_iter().filter(|num| num >> i & 1 == 0).collect();
                }
            }
        }

        if report.len() == 1 {
            return report[0];
        }
    }
    panic!("not found")
}

enum Cond {
    MostCommon,
    LeastCommon,
}

fn oxygen_generator_rating(report: &Vec<u16>, width: usize) -> u16 {
    find_rating(report, width, Cond::MostCommon)
}

fn co2_scrubber_rating(report: &Vec<u16>, width: usize) -> u16 {
    find_rating(report, width, Cond::LeastCommon)
}
