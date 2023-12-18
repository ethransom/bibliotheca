#![feature(let_chains)]
// #![feature(test)]

// extern crate test;

use std::collections::HashSet;

const EXAMPLE: &str = include_str!("example14.txt");
const INPUT: &str = include_str!("input14.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let mut map = Map::parse(input);

    println!("{}", map.print());
    println!("{}", map.round.len());

    let force = (0, -1);

    map.tilt(force);

    println!("\n{}", map.print());
    println!("{}", map.round.len());

    (map.total_load(), 0)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    round: HashSet<(usize, usize)>,
    cube: HashSet<(usize, usize)>,
    height: usize,
    width: usize,
}

impl Map {
    fn tilt(&mut self, force: (i64, i64)) {
        let (dx, dy) = force;
        let mut next = HashSet::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if !self.round.contains(&(x, y)) {
                    continue;
                }
                let (mut x, mut y) = (x as i64, y as i64);
                loop {
                    let (new_x, new_y) = (x + dx, y + dy);
                    if new_x < 0 || new_y < 0 {
                        break;
                    }
                    if new_x >= self.width as i64 || new_y >= self.height as i64 {
                        break;
                    }
                    if self.cube.contains(&(new_x as usize, new_y as usize)) {
                        break;
                    }
                    if next.contains(&(new_x as usize, new_y as usize)) {
                        break;
                    }
                    (x, y) = (new_x, new_y);
                }
                next.insert((x as usize, y as usize));
            }
        }
        self.round = next;
    }

    fn total_load(&self) -> usize {
        let mut load = 0;
        for (_x, y) in &self.round {
            load += y.abs_diff(self.height);
        }
        load
    }

    fn print(&self) -> String {
        let mut output = String::new();

        for y in 0..self.height {
            if y != 0 {
                output.push('\n');
            }
            for x in 0..self.width {
                let c = if self.cube.contains(&(x, y)) {
                    '#'
                } else if self.round.contains(&(x, y)) {
                    'O'
                } else {
                    '.'
                };
                output.push(c);
            }
        }

        output
    }

    fn parse(input: &str) -> Map {
        let mut round = HashSet::<(usize, usize)>::new();
        let mut cube = HashSet::<(usize, usize)>::new();
        let (mut height, mut width) = (0, 0);

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => {}
                    '#' => {
                        cube.insert((x, y));
                    }
                    'O' => {
                        round.insert((x, y));
                    }
                    _ => panic!("invalid character"),
                }
                height = height.max(y);
                width = width.max(x);
            }
        }

        Map {
            round,
            cube,
            height: height + 1,
            width: width + 1,
        }
    }
}

#[test]
fn test_parse_print() {
    assert_eq!(Map::parse(EXAMPLE).print(), EXAMPLE);

    assert_eq!(Map::parse(INPUT).print(), INPUT);
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (136, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (109939, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
