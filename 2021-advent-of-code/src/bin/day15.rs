#![feature(test)]

extern crate test;

use std::collections::BinaryHeap;

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
    // let visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

    let mut distance: Vec<Vec<u32>> = vec![vec![u32::MAX; grid[0].len()]; grid.len()];

    let mut to_visit: Vec<(u32, (usize, usize))> = Vec::new();

    distance[0][0] = 0;
    to_visit.push((0, (0, 0)));

    while let Some((cum_cost, (c, r))) = to_visit
        .iter()
        .min_by(|(cost, _), (cost_b, _)| cost.cmp(cost_b))
    {
        println!(
            "visiting {:?} at cost: {} from queue {:?}",
            (c, r),
            cum_cost,
            to_visit
        );

        for row in &distance {
            for cell in row {
                print!("{:4} ", cell);
            }
            println!();
        }

        if c == grid[0].len() - 1 && r == grid.len() - 1 {
            return cum_cost;
        }

        if cum_cost > distance[r][c] {
            continue;
        }

        for (x, y) in NEIGHBORS {
            let r = r as i32 + y;
            let c = c as i32 + x;

            if r < 0 || c < 0 {
                println!("skipping neighbor < 0 {}, {}", r, c);
                continue;
            }

            let r = r as usize;
            let c = c as usize;

            if r == grid.len() || c == grid[0].len() {
                println!("skipping neighbor > len {}, {}", r, c);
                continue;
            }

            println!("evaluating neighbor {}, {}", r, c);

            if cum_cost + grid[r][c] < distance[r][c] {
                println!(
                    "queuing neighbor {}, {} because of cost {} vs prev best known {}",
                    r,
                    c,
                    cum_cost + grid[r][c],
                    distance[r][c]
                );

                distance[r][c] = cum_cost + grid[r][c];
                to_visit.push((cum_cost + grid[r][c], (c, r)));
            }
        }
    }

    dbg!(to_visit);

    unreachable!("no solution to grid!");
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (40, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (0, 0));
    });
}
