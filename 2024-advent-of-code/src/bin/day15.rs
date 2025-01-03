// #![feature(test)]

// extern crate test;

use std::collections::HashSet;

const EXAMPLE: &str = include_str!("example15.txt");
const INPUT: &str = include_str!("input15.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let (warehouse, instructions) = input.split_once("\n\n").unwrap();

    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();
    let mut robot = None;
    let mut width = 0;
    let mut height = 0;
    for (y, line) in warehouse.trim().lines().enumerate() {
        let y = y as isize;
        height = height.max(y);
        for (x, c) in line.trim().chars().enumerate() {
            let x = x as isize;
            width = width.max(x);
            match c {
                '#' => {
                    walls.insert((x, y));
                }
                '@' => {
                    robot = Some((x, y));
                }
                'O' => {
                    boxes.insert((x, y));
                }
                '.' => {}
                _ => panic!("unknown char: {c}"),
            };
        }
    }

    let robot = robot.expect("no robot on map");
    let walls = walls;
    let boxes = boxes;

    let instructions: Vec<Move> = instructions
        .bytes()
        .filter(|b| b != &b'\n')
        .map(|b| match b {
            b'^' => Move::Up,
            b'>' => Move::Right,
            b'v' => Move::Down,
            b'<' => Move::Left,
            _ => panic!("unknown char: {b}"),
        })
        .collect();

    let part1 = {
        let mut robot = robot;
        let mut boxes = boxes.clone();

        for instr in instructions {
            let delta = instr.get_deltas();

            println!("moving robot {instr:?} ({delta:?})");

            let new = (robot.0 + delta.0, robot.1 + delta.1);
            if walls.contains(&new) {
                // println!("\tcannot move, hit wall");
                continue;
            }
            if boxes.contains(&new) {
                fn push(
                    boxes: &mut HashSet<Point>,
                    walls: &HashSet<Point>,
                    loc: Point,
                    dir: Point,
                ) -> bool {
                    let new = (loc.0 + dir.0, loc.1 + dir.1);
                    if walls.contains(&new) {
                        return false;
                    }
                    if boxes.contains(&new) {
                        if !push(boxes, walls, new, dir) {
                            return false;
                        }
                    }

                    boxes.remove(&loc);
                    boxes.insert(new);

                    true
                }

                // println!("\tpushing boxes...");

                if !push(&mut boxes, &walls, new, delta) {
                    continue;
                }
            }

            robot = new;

            // for y in 0..=height {
            //     for x in 0..=width {
            //         let c = if (x, y) == robot {
            //             '@'
            //         } else if walls.contains(&(x, y)) {
            //             '#'
            //         } else if boxes.contains(&(x, y)) {
            //             'O'
            //         } else {
            //             '.'
            //         };
            //         print!("{c}");
            //     }
            //     println!();
            // }
        }

        boxes.iter().map(|&(x, y)| x + y * 100).sum::<isize>()
    };

    (part1 as usize, 0)
}

type Point = (isize, isize);

#[derive(Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn get_deltas(&self) -> Point {
        match self {
            Move::Up => (0, -1),
            Move::Right => (1, 0),
            Move::Down => (0, 1),
            Move::Left => (-1, 0),
        }
    }
}

#[test]
fn test_small_example() {
    let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    assert_eq!(solve(input), (2028, 0));
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (10092, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (1514333, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
