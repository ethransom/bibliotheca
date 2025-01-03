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

        for instr in &instructions {
            let delta = instr.get_deltas();

            // println!("moving robot {instr:?} ({delta:?})");

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
                    if boxes.contains(&new) && !push(boxes, walls, new, dir) {
                        return false;
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

    let part2 = {
        let mut robot = (robot.0 * 2, robot.1);
        let mut boxes = HashSet::from_iter(boxes.iter().map(|(x, y)| (x * 2, *y)));
        let walls = HashSet::from_iter(walls.iter().map(|(x, y)| (x * 2, *y)));

        for instr in &instructions {
            let delta = instr.get_deltas();

            // println!("moving robot {instr:?} ({delta:?})");

            let new = (robot.0 + delta.0, robot.1 + delta.1);
            if walls.contains(&new) || walls.contains(&(new.0 - 1, new.1)) {
                // println!("\tcannot move, hit wall");
                continue;
            }
            let occupied_by_box = boxes.contains(&new);
            let new_offset = (new.0 - 1, new.1);
            let occupied_by_offset_box = boxes.contains(&new_offset);
            if occupied_by_box || occupied_by_offset_box {
                fn push(
                    boxes: &mut HashSet<Point>,
                    walls: &HashSet<Point>,
                    loc: Point,
                    dir: Point,
                ) -> bool {
                    boxes.remove(&loc);
                    let new = (loc.0 + dir.0, loc.1 + dir.1);
                    let new_ass = (loc.0 + dir.0 + 1, loc.1 + dir.1);
                    if (walls.contains(&new) || walls.contains(&(new.0 - 1, new.1)))
                        || (walls.contains(&new_ass) || walls.contains(&(new_ass.0 - 1, new_ass.1)))
                    {
                        return false;
                    }
                    let new_offset = (new.0 - 1, new.1);
                    // TODO: how come we don't need this?
                    // let new_ass_offset = (new_offset.0 - 1, new_offset.1);
                    if boxes.contains(&new) && !push(boxes, walls, new, dir) {
                        return false;
                    }
                    if boxes.contains(&new_offset) && !push(boxes, walls, new_offset, dir) {
                        return false;
                    }
                    if boxes.contains(&new_ass) && !push(boxes, walls, new_ass, dir) {
                        return false;
                    }

                    boxes.insert(new);

                    // println!("AFTER PUSH:");
                    // print(boxes);

                    true
                }

                // println!("\tpushing boxes...");

                let mut new_boxes = boxes.clone();

                let to_push = if occupied_by_box {
                    new
                } else if occupied_by_offset_box {
                    new_offset
                } else {
                    unreachable!();
                };
                if push(&mut new_boxes, &walls, to_push, delta) {
                    boxes = new_boxes;
                } else {
                    continue;
                }
            }

            robot = new;

            // for y in 0..=height {
            //     for x in 0..=(width * 2 + 1) {
            //         let c = if (x, y) == robot {
            //             '@'
            //         } else if walls.contains(&(x, y)) || walls.contains(&(x - 1, y)) {
            //             '#'
            //         } else if boxes.contains(&(x, y)) {
            //             '['
            //         } else if boxes.contains(&(x - 1, y)) {
            //             ']'
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

    (part1 as usize, part2 as usize)
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
fn test_small_example_part1() {
    let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    assert_eq!(solve(input).0, 2028);
}

#[test]
fn test_small_example_part2() {
    let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    solve(input);
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (10092, 9021));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (1514333, 1528453));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
