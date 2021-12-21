use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use crate::SolveInfo;

pub(crate) fn run(input: &str) -> anyhow::Result<SolveInfo> {
    let grid = parse_input(input);
    Ok(SolveInfo {
        challenge: "Chiton",
        part01: part01(&grid),
        part02: part02(&grid),
    })
}

fn part01(grid: &HashMap<(i32, i32), u32>) -> i64 {
    lowest_risk_path(grid)
}

fn part02(grid: &HashMap<(i32, i32), u32>) -> i64 {
    lowest_risk_path(&extend(grid))
}

// converts single grid into extended grid that is 5 times the size
fn extend(grid: &HashMap<(i32, i32), u32>) -> HashMap<(i32, i32), u32> {
    // add 1 to each of these to treat them as lengths instead of indexes
    let size_x = grid.iter().map(|((x, _), _)| x).max().unwrap() + 1;
    let size_y = grid.iter().map(|((_, y), _)| y).max().unwrap() + 1;

    // extend the grid to 5 times it's original size
    let mut extended = grid.clone();
    // "big" row #
    for row in 0..5 {
        // "big" col #
        for col in 0..5 {
            // loop over initial grid
            for y in 0..size_y {
                for x in 0..size_x {
                    let initial = grid[&(x, y)];

                    // add 1 for each "big" row or col that you are away from the initial
                    let mut v = initial + (row + col) as u32;

                    // handle 9->1 wrapping
                    if v > 9 {
                        v -= 9;
                    }

                    // convert x,y into "big" x,y
                    let new_x = (col * size_x) + x;
                    let new_y = (row * size_y) + y;

                    extended.insert((new_x, new_y), v);
                }
            }
        }
    }
    extended
}

// returns the total risk of the path with the lowest risk from top-left to bot-right
fn lowest_risk_path(grid: &HashMap<(i32, i32), u32>) -> i64 {
    // keep track of the smallest calculated risks for each point where calculated risk is
    // determined by summing the risks for all nodes on the path to this point.
    let mut risks = HashMap::new();
    risks.insert((0, 0), 0);

    // priority queue sorted by least calculated risk
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0u32), (0, 0)));

    let mut destination = (0, 0);
    for (pos, _) in grid.iter() {
        destination.0 = destination.0.max(pos.0);
        destination.1 = destination.1.max(pos.1);
    }

    // while our priority queue is not empty, check the next node with the smallest risk. the
    // priority queue saves us a lot of processing since we only care to check the shortest path.
    while let Some((Reverse(risk), current)) = queue.pop() {
        // if we are at our destination then it's risk is the smallest path risk
        if current == destination {
            return risk as i64;
        }

        // check all 4 neighbors
        let (col, row) = current;
        for neighbor in [
            (col - 1, row),
            (col + 1, row),
            (col, row - 1),
            (col, row + 1),
        ] {
            // ignore grid positions that don't exist e.g. (-1, 0), (0, -1)
            if !grid.contains_key(&neighbor) {
                continue;
            }

            // if we already found a path to get to this neighbor that's less risky, then bail
            let prev_risk = *risks.get(&neighbor).unwrap_or(&u32::MAX);
            if prev_risk < risk {
                continue;
            }

            // otherwise, the risk to get to the neighbor is our current risk + it's grid risk
            // (from the input). if that risk is lower than it's previously lowest risk then we
            // note it and enqueue the neighbor for checking again
            let nrisk = risk + grid[&neighbor];
            if prev_risk > nrisk {
                risks.insert(neighbor, nrisk);
                queue.push((Reverse(nrisk), neighbor));
            }
        }
    }
    unreachable!()
}

fn parse_input(input: &str) -> HashMap<(i32, i32), u32> {
    let mut grid = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            grid.insert((col as i32, row as i32), ch.to_digit(10).unwrap());
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15_example() {
        let input = include_str!("../inputs/15.test.txt");
        let grid = parse_input(input);

        assert_eq!(40, part01(&grid));
        assert_eq!(315, part02(&grid));
    }
}
