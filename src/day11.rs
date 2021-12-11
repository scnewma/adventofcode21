use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use crate::SolveInfo;

pub(crate) fn run(input: &str) -> anyhow::Result<SolveInfo> {
    let grid: Grid = input.parse().unwrap();
    Ok(SolveInfo {
        challenge: "Smoke Basin",
        part01: part01(&grid),
        part02: part02(&grid),
    })
}

const MAX_X: i64 = 10;
const MAX_Y: i64 = 10;
const GRID_AREA: i64 = MAX_X * MAX_Y;

fn part01(grid: &Grid) -> i64 {
    let mut grid = grid.clone();

    (0..100).map(|_| grid.step()).sum::<i64>()
}

fn part02(grid: &Grid) -> i64 {
    let mut grid = grid.clone();

    (1..)
        .find_map(|step| match grid.step() {
            GRID_AREA => Some(step),
            _ => None,
        })
        .unwrap()
}

#[derive(Clone)]
struct Grid(HashMap<(i64, i64), u32>);

impl Grid {
    // runs a single step of the problem returning the number of squid that flashed that step
    fn step(&mut self) -> i64 {
        // increment energy of each squid
        let mut flashes = self.inc_energy();

        // while we have squids that have flashed, increment the energy around them in a 3x3 grid,
        // a squid can only flash once per step
        let mut flashed = HashSet::new();
        while let Some(point) = flashes.pop_front() {
            if !flashed.insert(point) {
                continue;
            }

            flashes.append(&mut self.handle_flash(point));
        }

        // end of this step, reset energy levels for flashed squids
        flashed.iter().for_each(|(y, x)| {
            self.0.insert((*y, *x), 0);
        });

        flashed.len() as i64
    }

    // increments energy levels of each squid, returns squids that have "flashed"
    fn inc_energy(&mut self) -> VecDeque<(i64, i64)> {
        self.inc_energy_subgrid((0, 0), (MAX_Y - 1, MAX_X - 1))
    }

    // increments a 3x3 area around the point that flashed, returns squids that have "flashed" but
    // doesn't track whether the squid was already flashed before fn was called
    fn handle_flash(&mut self, (y, x): (i64, i64)) -> VecDeque<(i64, i64)> {
        self.inc_energy_subgrid((y - 1, x - 1), (y + 1, x + 1))
    }

    // increments energy of each squid in the grid defined by the input points, returns any squids
    // that are currently "flashed"
    fn inc_energy_subgrid(
        &mut self,
        (y1, x1): (i64, i64),
        (y2, x2): (i64, i64),
    ) -> VecDeque<(i64, i64)> {
        let mut flashes = VecDeque::new();
        for y in y1..=y2 {
            for x in x1..=x2 {
                let point = (y, x);
                // ensure this is a valid point
                if self.0.contains_key(&point) {
                    // increase energy level by one
                    self.0.entry(point).and_modify(|e| *e += 1);

                    // if flashed, add to queue
                    if self.0[&point] > 9 {
                        flashes.push_back(point);
                    }
                }
            }
        }
        flashes
    }
}

impl FromStr for Grid {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let energy = char.to_digit(10).unwrap();
                points.insert((y as i64, x as i64), energy);
            }
        }
        Ok(Self(points))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day11() {
        let input = include_str!("../inputs/11.txt");
        let solve = run(&input).unwrap();

        assert_eq!(1721, solve.part01);
        assert_eq!(298, solve.part02);
    }
}
