#![feature(test)]

extern crate test;

// use std::collections::BinaryHeap;

const EXAMPLE: &str = include_str!("example15.txt");
const INPUT: &str = include_str!("input15.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (u32, usize) {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    (min_cost(&grid), 0)
}

const NEIGHBORS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn min_cost(grid: &Vec<Vec<u32>>) -> u32 {
    let mut distance: Vec<Vec<u32>> = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
    let mut to_visit: Vec<(u32, (usize, usize))> = Vec::new();

    distance[0][0] = 0;
    to_visit.push((0, (0, 0)));

    loop {
        let (cum_cost, (c, r)) = match pop_min(&mut to_visit) {
            Some((cum_cost, (c, r))) => (cum_cost, (c, r)),
            None => break,
        };

        // println!("visiting {:?} at cost: {} from queue {:?}", (c, r), cum_cost, to_visit);

        // for row in &distance {
        //     for cell in row {
        //         print!("{:4} ", cell);
        //     }
        //     println!();
        // }

        // if at bottom right corner
        if c == grid[0].len() - 1 && r == grid.len() - 1 {
            return cum_cost;
        }

        // we already know about a cheaper route to this (c, r)
        if cum_cost > distance[r][c] {
            continue;
        }

        for (x, y) in NEIGHBORS {
            let r = r as i32 + y;
            let c = c as i32 + x;

            if r < 0 || c < 0 {
                // skip when off top or left edges
                continue;
            }

            let r = r as usize;
            let c = c as usize;

            if r == grid.len() || c == grid[0].len() {
                // skip when off right or bottom edges
                continue;
            }

            if cum_cost + grid[r][c] < distance[r][c] {
                // queue (c, r) because we've found a better path than previously known
                distance[r][c] = cum_cost + grid[r][c];
                to_visit.push((cum_cost + grid[r][c], (c, r)));
            }
        }
    }

    unreachable!("no solution to grid!");
}

// ENHANCEMENT: make generic?
// fn pop_min_generic<T>(vec: &mut Vec<T>, cmp: fn(&T, &T) -> std::cmp::Ordering) -> T { ... }
fn pop_min(to_visit: &mut Vec<(u32, (usize, usize))>) -> Option<(u32, (usize, usize))> {
    if let Some((ind, &min)) = to_visit
        .iter()
        .enumerate()
        .min_by(|(_i, (cost, _)), (_j, (cost_b, _))| cost.cmp(cost_b)) {
        to_visit.swap_remove(ind);

        return Some(min);
    }

    return None;
}

#[test]
fn test_pop_min() {
    assert_eq!(
        pop_min(&mut vec![(3, (1, 2)), (2, (5, 6)), (1, (9, 8))]),
        Some((1, (9, 8)))
    );
    assert_eq!(
        pop_min(&mut vec![]),
        None
    );
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (40, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (435, 0));
    });
}
