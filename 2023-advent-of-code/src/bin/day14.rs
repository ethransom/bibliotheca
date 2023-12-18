#![feature(test)]

extern crate test;

use fxhash::FxHashSet as HashSet;

const EXAMPLE: &str = include_str!("example14.txt");
const INPUT: &str = include_str!("input14.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let mut map = Map::parse(input);

    println!("{}", map.print());

    let mut map2 = map.clone();

    map2.tilt(Tilts::Up);
    println!("\n{}", map2.print());

    let single_tilt_load = map2.total_load();

    map.spin_cycle();
    println!("\n{}", map.print());
    println!("{}", map.total_load());
    map.spin_cycle();
    println!("\n{}", map.print());
    println!("{}", map.total_load());
    map.spin_cycle();
    println!("\n{}", map.print());
    println!("{}", map.total_load());

    for i in 3..1_000_000 {
        map.spin_cycle();
        // println!("{}", map.total_load());
    }

    (single_tilt_load, 0)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    round: HashSet<(usize, usize)>,
    cube: HashSet<(usize, usize)>,
    height: usize,
    width: usize,
}

enum Tilts {
    Up,
    Right,
    Down,
    Left,
}

impl Map {
    fn spin_cycle(&mut self) {
        self.tilt(Tilts::Up);

        self.tilt(Tilts::Left);

        self.tilt(Tilts::Down);

        self.tilt(Tilts::Right);
    }

    fn tilt(&mut self, force: Tilts) {
        let mut next = HashSet::default();

        match force {
            Tilts::Down => {
                let (dx, dy) = (0, 1);
                for y in (0..self.height).rev() {
                    for x in 0..self.width {
                        self.shift(y, x, dx, dy, &mut next)
                    }
                }
            },
            Tilts::Up => {
                let (dx, dy) = (0, -1);
                for y in 0..self.height {
                    for x in 0..self.width {
                        self.shift(y, x, dx, dy, &mut next)
                    }
                }
            },
            Tilts::Right => {
                let (dx, dy) = (1, 0);
                for x in (0..self.width).rev() {
                    for y in 0..self.height {
                        self.shift(y, x, dx, dy, &mut next)
                    }
                }
            },
            Tilts::Left => {
                let (dx, dy) = (-1, 0);
                for x in 0..self.width {
                    for y in 0..self.height {
                        self.shift(y, x, dx, dy, &mut next)
                    }
                }
            }
        }

        self.round = next;
    }

    fn shift(&mut self, y: usize, x: usize, dx: i64, dy: i64, next: &mut HashSet<(usize, usize)>) {
        if !self.round.contains(&(x, y)) {
            return;
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
        let mut round = HashSet::<(usize, usize)>::default();
        let mut cube = HashSet::<(usize, usize)>::default();
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
fn test_spin_cycle() {
    let mut map = Map::parse(EXAMPLE);

    map.spin_cycle();

    assert_eq!(map.print(), ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....");
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (136, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (109939, 0));
}

/// Performance history:
/// * 3288304 ns/iter (+/- 91758)   # Original (as of sha dbfb8e771515be22c39abc8b85a25a3cf51ad94d)
/// *  823238 ns/iter (+/- 44570)    # Use FxHashSet instead of default hash
/// (823238 ns * 1 billion) to hours is 228.6772222222 hours :((((((((((
/// (5 minutes / 1 billion) to nanoseconds is 300 ns # we have our target lmao
#[bench]
fn bench_spin_cycle(b: &mut test::Bencher) {
    let mut map = Map::parse(INPUT);
    b.iter(|| map.spin_cycle());
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
