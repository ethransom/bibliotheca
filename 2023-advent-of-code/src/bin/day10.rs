// #![feature(test)]

// extern crate test;

use std::collections::HashSet;

const EXAMPLE: &str = include_str!("example10.txt");
const INPUT: &str = include_str!("input10.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let map = Map::parse(input);

    println!("{}", map.serialize());

    (0, 0)
}

#[derive(Debug)]
struct Map {
    edges: HashSet<((i64, i64), (i64, i64))>,
    start: (i64, i64),
}

impl Map {
    fn neighbors(&self, pos: (i64, i64)) -> [bool; 4] {
        [
            (pos.0, pos.1 - 1), // never
            (pos.0 + 1, pos.1), // eat
            (pos.0, pos.1 + 1), // soggy
            (pos.0 - 1, pos.1), // waffles
        ]
        .map(|dst| self.edges.contains(&(pos, dst)))
    }

    fn serialize(&self) -> String {
        let ((x_min, x_max), (y_min, y_max)) = self.edges.iter().fold(
            ((0, 0), (0, 0)),
            |((x_min, x_max), (y_min, y_max)), ((x, y), _)| {
                (
                    (x_min.min(*x), x_max.max(*x)),
                    (y_min.min(*y), y_max.max(*y)),
                )
            },
        );

        let mut buf = String::new();

        for y in y_min..=y_max {
            if y != 0 {
                buf.push('\n');
            }
            for x in x_min..=x_max {
                let pos = (x, y);
                if self.start == pos {
                    buf.push('S');
                } else {
                    match self.neighbors(pos) {
                        // | is a vertical pipe connecting north and south.
                        [true, false, true, false] => buf.push('|'),
                        // - is a horizontal pipe connecting east and west.
                        [false, true, false, true] => buf.push('-'),
                        // L is a 90-degree bend connecting north and east.
                        [true, true, false, false] => buf.push('L'),
                        // J is a 90-degree bend connecting north and west.
                        [true, false, false, true] => buf.push('J'),
                        // 7 is a 90-degree bend connecting south and west.
                        [false, false, true, true] => buf.push('7'),
                        // F is a 90-degree bend connecting south and east.
                        [false, true, true, false] => buf.push('F'),
                        // . is ground; there is no pipe in this tile.
                        [false, false, false, false] => buf.push('.'),

                        _ => panic!("unknown neighbors: {:?}", self.neighbors(pos)),
                    }
                }
            }
        }

        buf
    }

    fn parse(input: &str) -> Map {
        let mut start = None;
        let mut edges = HashSet::new();

        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let pos = (x as i64, y as i64);
                match c {
                    // | is a vertical pipe connecting north and south.
                    '|' => {
                        edges.insert((pos, (pos.0, pos.1 + 1)));
                        edges.insert((pos, (pos.0, pos.1 - 1)));
                    }
                    // - is a horizontal pipe connecting east and west.
                    '-' => {
                        edges.insert((pos, (pos.0 + 1, pos.1)));
                        edges.insert((pos, (pos.0 - 1, pos.1)));
                    }
                    // L is a 90-degree bend connecting north and east.
                    'L' => {
                        edges.insert((pos, (pos.0, pos.1 - 1)));
                        edges.insert((pos, (pos.0 + 1, pos.1)));
                    }
                    // J is a 90-degree bend connecting north and west.
                    'J' => {
                        edges.insert((pos, (pos.0, pos.1 - 1)));
                        edges.insert((pos, (pos.0 - 1, pos.1)));
                    }
                    // 7 is a 90-degree bend connecting south and west.
                    '7' => {
                        edges.insert((pos, (pos.0, pos.1 + 1)));
                        edges.insert((pos, (pos.0 - 1, pos.1)));
                    }
                    // F is a 90-degree bend connecting south and east.
                    'F' => {
                        edges.insert((pos, (pos.0, pos.1 + 1)));
                        edges.insert((pos, (pos.0 + 1, pos.1)));
                    }
                    // . is ground; there is no pipe in this tile.
                    '.' => {}
                    // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
                    'S' => {
                        start = Some(pos);
                        // assume the neighboring pipes will create the edges into this tile
                    }
                    _ => panic!("unknown char: {}", c),
                }
            })
        });

        Map {
            edges,
            start: start.expect("no start found"),
        }
    }
}

#[test]
fn test_parse_unparse() {
    let map = Map::parse(EXAMPLE);
    assert_eq!(map.serialize(), EXAMPLE);
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (0, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (0, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
