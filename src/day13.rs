use std::{collections::HashMap, usize};

use crate::SolveInfo;

pub(crate) fn run(input: &str) -> anyhow::Result<SolveInfo> {
    let (points, folds) = parse_input(input);
    Ok(SolveInfo {
        challenge: "Transparent Origami",
        part01: part01(&points, &folds),
        part02: part02(&points, &folds),
    })
}

fn part01(points: &Vec<(usize, usize)>, folds: &Vec<(&str, usize)>) -> i64 {
    execute_folds(points, &folds[0..1]).len() as i64
}

fn part02(points: &Vec<(usize, usize)>, folds: &Vec<(&str, usize)>) -> i64 {
    let grid = execute_folds(points, folds);
    let (max_x, max_y) = grid.iter().fold((0, 0), |(max_x, max_y), ((x, y), _)| {
        (max_x.max(*x), max_y.max(*y))
    });
    print_grid(&grid, max_x, max_y);
    0
}

fn execute_folds(
    points: &[(usize, usize)],
    folds: &[(&str, usize)],
) -> HashMap<(usize, usize), i32> {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut grid = HashMap::new();
    for (x, y) in points {
        grid.insert((*x, *y), 1);
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }

    for (axis, idx) in folds {
        grid = match *axis {
            "x" => {
                let mut new_grid = HashMap::new();

                for y in 0..=max_y {
                    for x in 0..*idx {
                        let left = grid.get(&(x, y));
                        let right = grid.get(&(idx - x + idx, y));

                        match (left, right) {
                            (Some(_), None) | (None, Some(_)) | (Some(_), Some(_)) => {
                                new_grid.insert((x, y), 1);
                            }
                            (None, None) => continue,
                        }
                    }
                }

                new_grid
            }
            "y" => {
                let mut new_grid = HashMap::new();

                for y in 0..*idx {
                    for x in 0..=max_x {
                        let top = grid.get(&(x, y));
                        let bot = grid.get(&(x, idx - y + idx));

                        match (top, bot) {
                            (Some(_), None) | (None, Some(_)) | (Some(_), Some(_)) => {
                                new_grid.insert((x, y), 1);
                            }
                            (None, None) => continue,
                        }
                    }
                }

                new_grid
            }
            _ => unreachable!(),
        };
    }

    grid
}

fn print_grid(grid: &HashMap<(usize, usize), i32>, max_x: usize, max_y: usize) {
    for y in 0..=max_y {
        for x in 0..=max_x {
            let point = grid.get(&(x, y)).map_or(" ", |_| "#");
            print!("{}", point);
        }
        println!();
    }
    println!();
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<(&str, usize)>) {
    let (points, folds) = input.split_once("\n\n").unwrap();
    let points: Vec<(usize, usize)> = points
        .lines()
        .flat_map(|s| s.split_once(","))
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    let folds = folds
        .lines()
        .flat_map(|s| s.strip_prefix("fold along "))
        .flat_map(|s| s.split_once("="))
        .map(|(axis, idx)| (axis, idx.parse().unwrap()))
        .collect();

    (points, folds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day13_example() {
        let input = include_str!("../inputs/13.test.txt");
        let (points, folds) = parse_input(input);

        assert_eq!(17, part01(&points, &folds));
        // have to check stdout
        // assert_eq!(0, part02(&points, &folds));
    }

    #[test]
    fn test_day13() {
        let input = include_str!("../inputs/13.txt");
        let (points, folds) = parse_input(input);

        assert_eq!(610, part01(&points, &folds));
    }
}
