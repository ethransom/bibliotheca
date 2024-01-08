#![feature(test)]

extern crate test;

use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};

const EXAMPLE: &str = include_str!("example14.txt");
const INPUT: &str = include_str!("input14.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let mut map = Map::parse(input);

    println!("Original map:\n{}", map.print());

    let mut map2 = map.clone();

    map2.tilt(Tilts::Up);
    println!("\nTilted UP:\n{}", map2.print());

    let single_tilt_load = map2.total_load();

    println!("\nBeginning spin cycle...");

    let mut load_at_1000000000 = None;

    let mut lookup = HashMap::default();
    let mut load_lookup = HashMap::default();
    for i in 0..200usize {
        map.spin_cycle();
        let m = map.print();
        if let Some(s) = lookup.get(&m) {
            println!("spin {i} is same as spin {s}");
            let equivalent_spin = ((1000000000 - s) % (i - s)) + s - 1;
            let load = load_lookup[&equivalent_spin];
            load_at_1000000000 = Some(load);
            println!("load at 1000000000 was same as {equivalent_spin}, which was {load}");
            break;
        }
        let load = map.total_load();
        if i < 3 {
            println!("\n{}", m);
            println!("{}", load);
        }
        lookup.insert(m, i);
        load_lookup.insert(i, load);
    }

    (single_tilt_load, load_at_1000000000.unwrap())
}

type Point = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    round: HashSet<Point>,
    round_alt: HashSet<Point>,
    cube: HashSet<Point>,
    height: usize,
    width: usize,
}

#[derive(Debug)]
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
        match force {
            Tilts::Down => {
                let (dx, dy) = (0, 1);
                for y in (0..self.height).rev() {
                    for x in 0..self.width {
                        self.shift(y, x, dx, dy)
                    }
                }
            }
            Tilts::Up => {
                let (dx, dy) = (0, -1);
                for y in 0..self.height {
                    for x in 0..self.width {
                        self.shift(y, x, dx, dy)
                    }
                }
            }
            Tilts::Right => {
                let (dx, dy) = (1, 0);
                for x in (0..self.width).rev() {
                    for y in 0..self.height {
                        self.shift(y, x, dx, dy)
                    }
                }
            }
            Tilts::Left => {
                let (dx, dy) = (-1, 0);
                for x in 0..self.width {
                    for y in 0..self.height {
                        self.shift(y, x, dx, dy)
                    }
                }
            }
        }

        self.round.clear();
        std::mem::swap(&mut self.round, &mut self.round_alt);
    }

    fn shift(&mut self, y: usize, x: usize, dx: i64, dy: i64) {
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
            if self.round_alt.contains(&(new_x as usize, new_y as usize)) {
                break;
            }
            (x, y) = (new_x, new_y);
        }
        self.round_alt.insert((x as usize, y as usize));
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
        let mut round = HashSet::default();
        let mut cube = HashSet::default();
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
            round_alt: HashSet::default(),
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

    assert_eq!(
        map.print(),
        ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
    );

    map.spin_cycle();

    assert_eq!(
        map.print(),
        ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
    );

    map.spin_cycle();

    assert_eq!(
        map.print(),
        ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
    );
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (136, 64));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (109939, 101010));
}

/// Performance history:
/// * 3288304 ns/iter (+/- 91758)   # Original (as of sha dbfb8e771515be22c39abc8b85a25a3cf51ad94d)
/// *  823238 ns/iter (+/- 44570)    # Use FxHashSet instead of default hash
///         Aside:
///             (823238 ns * 1 billion) to hours is 228.6772222222 hours :((((((((((
///             (5 minutes / 1 billion) to nanoseconds is 300 ns # we have our target lmao
/// *  581606 ns/iter (+/- 8836)    # Main loop noalloc
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
