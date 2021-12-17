use std::{collections::HashSet, usize};

use crate::SolveInfo;

pub(crate) fn run(input: &str) -> anyhow::Result<SolveInfo> {
    let (points, folds) = parse_input(input);
    Ok(SolveInfo {
        challenge: "Transparent Origami",
        part01: part01(&points, &folds),
        part02: part02(&points, &folds),
    })
}

fn part01(points: &Vec<(usize, usize)>, folds: &Vec<Fold>) -> i64 {
    execute_folds(points, &folds[0..1]).len() as i64
}

fn part02(points: &Vec<(usize, usize)>, folds: &Vec<Fold>) -> i64 {
    let grid = execute_folds(points, folds);
    print_grid(&grid);
    0
}

enum Fold {
    X(usize),
    Y(usize),
}

fn execute_folds(points: &[(usize, usize)], folds: &[Fold]) -> HashSet<(usize, usize)> {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut grid = HashSet::new();
    for (x, y) in points {
        grid.insert((*x, *y));
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }

    for fold in folds {
        grid = match fold {
            Fold::X(idx) => {
                let mut new_grid = HashSet::new();

                for y in 0..=max_y {
                    for x in 0..*idx {
                        let left = grid.get(&(x, y));
                        let right = grid.get(&(2 * idx - x, y));

                        match (left, right) {
                            (Some(_), None) | (None, Some(_)) | (Some(_), Some(_)) => {
                                new_grid.insert((x, y));
                            }
                            (None, None) => continue,
                        }
                    }
                }

                new_grid
            }
            Fold::Y(idx) => {
                let mut new_grid = HashSet::new();

                for y in 0..*idx {
                    for x in 0..=max_x {
                        let top = grid.get(&(x, y));
                        let bot = grid.get(&(x, 2 * idx - y));

                        match (top, bot) {
                            (Some(_), None) | (None, Some(_)) | (Some(_), Some(_)) => {
                                new_grid.insert((x, y));
                            }
                            (None, None) => continue,
                        }
                    }
                }

                new_grid
            }
        };
    }

    grid
}

fn print_grid(grid: &HashSet<(usize, usize)>) {
    let (max_x, max_y) = grid.iter().fold((0, 0), |(max_x, max_y), (x, y)| {
        (max_x.max(*x), max_y.max(*y))
    });

    for y in 0..=max_y {
        for x in 0..=max_x {
            let point = grid.get(&(x, y)).map_or(" ", |_| "#");
            print!("{}", point);
        }
        println!();
    }
    println!();
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<Fold>) {
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
        .map(|(axis, idx)| {
            let idx = idx.parse().unwrap();
            match axis {
                "x" => Fold::X(idx),
                "y" => Fold::Y(idx),
                _ => unreachable!(),
            }
        })
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
