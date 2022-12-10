#![feature(test)]

extern crate test;

const EXAMPLE: &str = include_str!("example08.txt");
const INPUT: &str = include_str!("input08.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let grid = parse(input).expect("couldn't parse input");

    let visible = grid
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|&(c, &cell)| {
                    let directions = [(-1, 0), (0, -1), (0, 1), (1, 0)];

                    directions.iter().any(|direction| {
                        let mut x = c as i32;
                        let mut y = r as i32;

                        while x > 0
                            && y > 0
                            && x < row.len() as i32 - 1
                            && y < grid.len() as i32 - 1
                        {
                            x += direction.0;
                            y += direction.1;

                            if grid[y as usize][x as usize] >= cell {
                                return false;
                            }
                        }

                        true
                    })
                })
                .count()
        })
        .sum();

    let best_view: usize = grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .map(|(c, &cell)| {
                    let directions = [(-1, 0), (0, -1), (0, 1), (1, 0)];

                    directions
                        .iter()
                        .map(|direction| -> usize {
                            let mut x = c as i32;
                            let mut y = r as i32;

                            let mut visible = 0;

                            while x > 0
                                && y > 0
                                && x < row.len() as i32 - 1
                                && y < grid.len() as i32 - 1
                            {
                                x += direction.0;
                                y += direction.1;

                                visible += 1;

                                if grid[y as usize][x as usize] >= cell {
                                    break;
                                    // view PAST, but not including, this cell is blocked
                                }
                            }

                            visible
                        })
                        .product()
                })
                .max()
        })
        .max()
        .expect("no trees with view score");

    (visible, best_view)
}

fn parse(input: &str) -> Option<Vec<Vec<u8>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).map(|d| d as u8))
                .collect()
        })
        .collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (21, 8));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (1_849, 201_600));
    });
}
