#![feature(test)]

extern crate test;

use std::collections::HashMap;

type Point = (i32, i32);

const EXAMPLE: &str = include_str!("example09.txt");
const INPUT: &str = include_str!("input09.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

const IMMEDIATE_NEIGHBORS: [Point; 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn solve(input: &str) -> (usize, usize) {
    let mut cells: HashMap<Point, u32> = HashMap::new();

    let mut height = 0;
    let mut width = 0;

    input.lines().enumerate().for_each(|(y, line)| {
        let mut row_width = 0;
        line.chars().enumerate().for_each(|(x, c)| {
            let n = c.to_digit(10).expect("not an int");
            cells.insert((x as i32, y as i32), n);

            row_width += 1;
        });

        height += 1;
        if row_width > width {
            width = row_width;
        }
    });

    let low_points = (0..height)
        .flat_map(|row| (0..width).map(move |col| (col, row) as Point))
        .filter(|&(col, row)| {
            let &cell = cells.get(&(col, row)).expect("grid was sparse!");

            IMMEDIATE_NEIGHBORS.into_iter().all(|(x, y)| {
                if let Some(&neighbor) = cells.get(&(col + x, row + y)) {
                    neighbor > cell
                } else {
                    true
                }
            })
        });

    (
        low_points
            .clone()
            .map(|(col, row)| *cells.get(&(col, row)).unwrap() as usize + 1)
            .sum(),
        0,
    )
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (15, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (439, 0));
    });
}
