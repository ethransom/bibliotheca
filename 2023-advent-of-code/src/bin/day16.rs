#![feature(test)]

extern crate test;

use fxhash::FxHashMap as HashMap;
use std::thread;

const EXAMPLE: &str = include_str!("example16.txt");
const INPUT: &str = include_str!("input16.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

type Velocity = (i8, i8);

const RIGHT: Velocity = (1, 0);
const LEFT: Velocity = (-1, 0);
const DOWN: Velocity = (0, 1);
const UP: Velocity = (0, -1);

const UPDOWN: [Velocity; 2] = [UP, DOWN]; // I dunno, what's updown with you?
const RIGHTLEFT: [Velocity; 2] = [RIGHT, LEFT];

fn solve(input: &str) -> (usize, usize) {
    let grid = Grid::try_from(input).unwrap_or_else(|err| panic!("couldn't parse grid: {err}"));

    let top_left_energized = energize(&grid, Point { x: 0, y: 0 }, RIGHT);

    println!("{}", grid);

    println!();

    println!("{}", pretty_print_beams(&grid, &top_left_energized));

    println!();

    println!("{}", pretty_print_energized(&grid, &top_left_energized));

    println!();

    let max_energized = thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(move || {
            let top = (0..grid.width).map(|x| (Point { x, y: 0 }, DOWN));
            let bottom = (0..grid.width).map(|x| {
                (
                    Point {
                        x,
                        y: grid.height - 1,
                    },
                    UP,
                )
            });
            let left = (0..grid.height).map(|y| (Point { x: 0, y }, DOWN));
            let right = (0..grid.height).map(|y| {
                (
                    Point {
                        x: grid.width - 1,
                        y,
                    },
                    DOWN,
                )
            });
            let all_edges = top.chain(bottom).chain(left).chain(right);
            println!(
                "checking all {count} possible beam entry points",
                count = all_edges.clone().count()
            );
            all_edges
                .map(|(start, v)| energize(&grid, start, v).len())
                .max()
                .unwrap_or(0)
        })
        .expect("couldn't spawn big boi thread")
        .join()
        .expect("child thread couldn't compute longest path");

    println!("can get {max_energized} max energized 🌋");

    (top_left_energized.len(), max_energized)
}

fn pretty_print_energized(grid: &Grid, energized: &HashMap<Point, Vec<Velocity>>) -> String {
    let mut output = String::new();
    for y in 0..grid.height {
        if y != 0 {
            output.push('\n');
        }
        for x in 0..grid.width {
            let c = if energized.contains_key(&Point { x, y }) {
                '#'
            } else {
                grid[Point { x, y }]
            };
            output.push(c);
        }
    }
    output
}

fn pretty_print_beams(grid: &Grid, energized: &HashMap<Point, Vec<Velocity>>) -> String {
    let mut output = String::new();
    for y in 0..grid.height {
        if y != 0 {
            output.push('\n');
        }
        for x in 0..grid.width {
            let mut c = grid[Point { x, y }];
            if c == '.' {
                if let Some(v) = energized.get(&Point { x, y }) {
                    c = if v.len() == 1 {
                        match v[0] {
                            RIGHT => '>',
                            LEFT => '<',
                            DOWN => 'v',
                            UP => '^',
                            _ => panic!(),
                        }
                    } else {
                        v.len().to_string().as_bytes()[0] as char
                    };
                }
            }
            output.push(c);
        }
    }
    output
}

fn energize(grid: &Grid, start: Point, v: Velocity) -> HashMap<Point, Vec<Velocity>> {
    let mut energized = HashMap::default();
    energize(grid, &mut energized, start, v);
    return energized;

    fn emit(grid: &Grid, energized: &mut HashMap<Point, Vec<Velocity>>, point: Point, v: Velocity) {
        let next = grid.step(point, v);
        if let Some(next) = next {
            energize(grid, energized, next, v)
        }
    }

    fn energize(
        grid: &Grid,
        energized: &mut HashMap<Point, Vec<Velocity>>,
        point: Point,
        (vx, vy): Velocity,
    ) {
        let beams = energized.entry(point).or_default();
        if beams.contains(&(vx, vy)) {
            return;
        }
        beams.push((vx, vy));

        let cell = grid[point];
        match cell {
            '|' if RIGHTLEFT.contains(&(vx, vy)) => {
                emit(grid, energized, point, UP);
                emit(grid, energized, point, DOWN);
            }
            '-' if UPDOWN.contains(&(vx, vy)) => {
                emit(grid, energized, point, RIGHT);
                emit(grid, energized, point, LEFT);
            }
            _ => {
                let out = if cell == '/' {
                    match (vx, vy) {
                        RIGHT => UP,
                        LEFT => DOWN,
                        DOWN => LEFT,
                        UP => RIGHT,
                        _ => panic!(),
                    }
                } else if cell == '\\' {
                    match (vx, vy) {
                        RIGHT => DOWN,
                        LEFT => UP,
                        DOWN => RIGHT,
                        UP => LEFT,
                        _ => panic!(),
                    }
                } else {
                    (vx, vy)
                };
                emit(grid, energized, point, out);
            }
        }
    }
}

#[derive(Debug)]
struct Grid {
    cells: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn step(&self, point: Point, (vx, vy): Velocity) -> Option<Point> {
        let (x, y) = (point.x as isize, point.y as isize);
        let x = x + vx as isize;
        let y = y + vy as isize;
        if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
            None
        } else {
            Some(Point {
                x: x as usize,
                y: y as usize,
            })
        }
    }
}

impl std::ops::Index<Point> for Grid {
    type Output = char;

    fn index(&self, Point { x, y }: Point) -> &Self::Output {
        let i = (self.width * y) + x;
        if i >= self.cells.len() {
            panic!(
                "out of bounds, Grid {{ width: {width}, height: {height} }} asked for ({x}, {y})",
                width = self.width,
                height = self.height
            )
        }
        &self.cells[i]
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for y in 0..self.height {
            if y != 0 {
                writeln!(f)?;
            }
            for x in 0..self.width {
                write!(f, "{}", self[Point { x, y }])?;
            }
        }

        Ok(())
    }
}

impl TryFrom<&str> for Grid {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut cells = vec![];
        for (row, line) in value.lines().enumerate() {
            max_y = max_y.max(row);
            for (col, c) in line.chars().enumerate() {
                max_x = max_x.max(col);
                if !['.', '/', '\\', '-', '|'].contains(&c) {
                    return Err(format!("Unexpected character {c}"));
                }
                cells.push(c);
            }
        }

        let width = max_x + 1;
        let height = max_y + 1;

        Ok(Grid {
            cells,
            width,
            height,
        })
    }
}

#[test]
fn test_try_parse() {
    for (input, name) in [(EXAMPLE, "example"), (INPUT, "input")] {
        assert_eq!(
            format!("{}", Grid::try_from(input).unwrap()),
            input,
            "mismatch in '{name}'"
        );
    }
}

#[test]
fn test_energize_example() {
    let grid = Grid::try_from(EXAMPLE).unwrap();
    let energized = energize(&grid, Point { x: 0, y: 0 }, RIGHT);
    let expected = r#">|<<<\....
|v-.\^....
.v...|->>>
.v...v^.|.
.v...v^...
.v...v^..\
.v../2\\..
<->-/vv|..
.|<<<2-|.\
.v//.|.v.."#;
    let actual = pretty_print_beams(&grid, &energized);
    assert_eq!(actual, expected)
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (46, 51));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (7472, 7716));
}

#[bench]
fn bench_energize_input_06_flat_hanrolled_vectormap(b: &mut test::Bencher) {
    #[derive(Copy, Clone, Eq, PartialEq)]
    enum Velocity {
        Up,
        Down,
        Left,
        Right,
    }

    fn energize(grid: &Grid, start: Point, v: Velocity) -> Vec<[bool; 4]> {
        let mut energized = vec![Default::default(); grid.cells.len()];
        energize(grid, &mut energized, start, v);
        return energized;

        fn step(grid: &Grid, point: Point, v: Velocity) -> Option<Point> {
            let (x, y) = (point.x as isize, point.y as isize);
            let (vx, vy) = match v {
                Velocity::Up => UP,
                Velocity::Down => DOWN,
                Velocity::Left => LEFT,
                Velocity::Right => RIGHT,
            };
            let x = x + vx as isize;
            let y = y + vy as isize;
            if x < 0 || x >= grid.width as isize || y < 0 || y >= grid.height as isize {
                None
            } else {
                Some(Point {
                    x: x as usize,
                    y: y as usize,
                })
            }
        }

        fn energize(grid: &Grid, energized: &mut [[bool; 4]], point: Point, v: Velocity) {
            let Point { x, y } = point;
            let beams = &mut energized[(grid.width * y) + x];
            let i = match v {
                Velocity::Up => 0,
                Velocity::Down => 1,
                Velocity::Left => 2,
                Velocity::Right => 3,
            };
            if beams[i] {
                return;
            }
            beams[i] = true;

            match grid[point] {
                '|' if v == Velocity::Right || v == Velocity::Left => {
                    for v in [Velocity::Up, Velocity::Down] {
                        let next = step(grid, point, v);
                        if let Some(next) = next {
                            energize(grid, energized, next, v)
                        }
                    }
                }
                '-' if v == Velocity::Up || v == Velocity::Down => {
                    for v in [Velocity::Right, Velocity::Left] {
                        let next = step(grid, point, v);
                        if let Some(next) = next {
                            energize(grid, energized, next, v)
                        }
                    }
                }
                '/' => {
                    let out = match v {
                        Velocity::Right => Velocity::Up,
                        Velocity::Left => Velocity::Down,
                        Velocity::Down => Velocity::Left,
                        Velocity::Up => Velocity::Right,
                    };
                    let next = step(grid, point, out);
                    if let Some(next) = next {
                        energize(grid, energized, next, out)
                    }
                }
                '\\' => {
                    let out = match v {
                        Velocity::Right => Velocity::Down,
                        Velocity::Left => Velocity::Up,
                        Velocity::Down => Velocity::Right,
                        Velocity::Up => Velocity::Left,
                    };
                    let next = step(grid, point, out);
                    if let Some(next) = next {
                        energize(grid, energized, next, out)
                    }
                }
                '.' | '|' | '-' => {
                    let next = step(grid, point, v);
                    if let Some(next) = next {
                        energize(grid, energized, next, v)
                    }
                }
                _ => panic!(),
            }
        }
    }

    let grid = Grid::try_from(INPUT).unwrap();
    b.iter(|| {
        assert_eq!(
            energize(&grid, Point { x: 0, y: 0 }, Velocity::Right)
                .iter()
                .filter(|c| c[0] || c[1] || c[2] || c[3])
                .count(),
            7472
        );
    });
}

#[bench]
fn bench_energize_input_05_flatvectormap(b: &mut test::Bencher) {
    use arrayvec::ArrayVec;
    fn energize(grid: &Grid, start: Point, v: Velocity) -> Vec<ArrayVec<Velocity, 4>> {
        let mut energized = vec![ArrayVec::default(); grid.cells.len()];
        energize(grid, &mut energized, start, v);
        return energized;

        fn energize(
            grid: &Grid,
            energized: &mut [ArrayVec<Velocity, 4>],
            point: Point,
            (vx, vy): Velocity,
        ) {
            let Point { x, y } = point;
            let beams = &mut energized[(grid.width * y) + x];
            if beams.contains(&(vx, vy)) {
                return;
            }
            beams.push((vx, vy));

            match grid[point] {
                '.' => {
                    let next = grid.step(point, (vx, vy));
                    if let Some(next) = next {
                        energize(grid, energized, next, (vx, vy))
                    }
                }
                '|' => {
                    if RIGHTLEFT.contains(&(vx, vy)) {
                        for v in UPDOWN {
                            let next = grid.step(point, v);
                            if let Some(next) = next {
                                energize(grid, energized, next, v)
                            }
                        }
                    } else {
                        let next = grid.step(point, (vx, vy));
                        if let Some(next) = next {
                            energize(grid, energized, next, (vx, vy))
                        }
                    }
                }
                '-' => {
                    if UPDOWN.contains(&(vx, vy)) {
                        for v in RIGHTLEFT {
                            let next = grid.step(point, v);
                            if let Some(next) = next {
                                energize(grid, energized, next, v)
                            }
                        }
                    } else {
                        let next = grid.step(point, (vx, vy));
                        if let Some(next) = next {
                            energize(grid, energized, next, (vx, vy))
                        }
                    }
                }
                '/' => {
                    let out = match (vx, vy) {
                        RIGHT => UP,
                        LEFT => DOWN,
                        DOWN => LEFT,
                        UP => RIGHT,
                        _ => panic!(),
                    };
                    let next = grid.step(point, out);
                    if let Some(next) = next {
                        energize(grid, energized, next, out)
                    }
                }
                '\\' => {
                    let out = match (vx, vy) {
                        RIGHT => DOWN,
                        LEFT => UP,
                        DOWN => RIGHT,
                        UP => LEFT,
                        _ => panic!(),
                    };
                    let next = grid.step(point, out);
                    if let Some(next) = next {
                        energize(grid, energized, next, out)
                    }
                }
                _ => panic!(),
            }
        }
    }

    let grid = Grid::try_from(INPUT).unwrap();
    b.iter(|| {
        assert_eq!(
            energize(&grid, Point { x: 0, y: 0 }, RIGHT)
                .iter()
                .filter(|c| !c.is_empty())
                .count(),
            7472
        );
    });
}

#[bench]
fn bench_energize_input_04_vectormap(b: &mut test::Bencher) {
    fn energize(grid: &Grid, start: Point, v: Velocity) -> Vec<Vec<Velocity>> {
        let mut energized = vec![vec![]; grid.cells.len()];
        energize(grid, &mut energized, start, v);
        return energized;

        fn energize(
            grid: &Grid,
            energized: &mut [Vec<Velocity>],
            point: Point,
            (vx, vy): Velocity,
        ) {
            let Point { x, y } = point;
            let beams = &mut energized[(grid.width * y) + x];
            if beams.contains(&(vx, vy)) {
                return;
            }
            beams.push((vx, vy));

            match grid[point] {
                '.' => {
                    let next = grid.step(point, (vx, vy));
                    if let Some(next) = next {
                        energize(grid, energized, next, (vx, vy))
                    }
                }
                '|' => {
                    if RIGHTLEFT.contains(&(vx, vy)) {
                        for v in UPDOWN {
                            let next = grid.step(point, v);
                            if let Some(next) = next {
                                energize(grid, energized, next, v)
                            }
                        }
                    } else {
                        let next = grid.step(point, (vx, vy));
                        if let Some(next) = next {
                            energize(grid, energized, next, (vx, vy))
                        }
                    }
                }
                '-' => {
                    if UPDOWN.contains(&(vx, vy)) {
                        for v in RIGHTLEFT {
                            let next = grid.step(point, v);
                            if let Some(next) = next {
                                energize(grid, energized, next, v)
                            }
                        }
                    } else {
                        let next = grid.step(point, (vx, vy));
                        if let Some(next) = next {
                            energize(grid, energized, next, (vx, vy))
                        }
                    }
                }
                '/' => {
                    let out = match (vx, vy) {
                        RIGHT => UP,
                        LEFT => DOWN,
                        DOWN => LEFT,
                        UP => RIGHT,
                        _ => panic!(),
                    };
                    let next = grid.step(point, out);
                    if let Some(next) = next {
                        energize(grid, energized, next, out)
                    }
                }
                '\\' => {
                    let out = match (vx, vy) {
                        RIGHT => DOWN,
                        LEFT => UP,
                        DOWN => RIGHT,
                        UP => LEFT,
                        _ => panic!(),
                    };
                    let next = grid.step(point, out);
                    if let Some(next) = next {
                        energize(grid, energized, next, out)
                    }
                }
                _ => panic!(),
            }
        }
    }

    let grid = Grid::try_from(INPUT).unwrap();
    b.iter(|| {
        assert_eq!(
            energize(&grid, Point { x: 0, y: 0 }, RIGHT)
                .iter()
                .filter(|c| !c.is_empty())
                .count(),
            7472
        );
    });
}

#[bench]
fn bench_energize_input_03_readability(b: &mut test::Bencher) {
    let grid = Grid::try_from(INPUT).unwrap();
    b.iter(|| assert_eq!(energize(&grid, Point { x: 0, y: 0 }, RIGHT).len(), 7472));
}

#[bench]
fn bench_energize_input_02_fxhasher(b: &mut test::Bencher) {
    fn energize(grid: &Grid, start: Point, v: Velocity) -> HashMap<Point, Vec<Velocity>> {
        let mut energized = HashMap::default();
        energize(grid, &mut energized, start, v);
        return energized;

        fn energize(
            grid: &Grid,
            energized: &mut HashMap<Point, Vec<Velocity>>,
            point: Point,
            (vx, vy): Velocity,
        ) {
            let beams = energized.entry(point).or_default();
            if beams.contains(&(vx, vy)) {
                return;
            }
            beams.push((vx, vy));

            match grid[point] {
                '.' => {
                    let next = grid.step(point, (vx, vy));
                    if let Some(next) = next {
                        energize(grid, energized, next, (vx, vy))
                    }
                }
                '|' => {
                    if RIGHTLEFT.contains(&(vx, vy)) {
                        for v in UPDOWN {
                            let next = grid.step(point, v);
                            if let Some(next) = next {
                                energize(grid, energized, next, v)
                            }
                        }
                    } else {
                        let next = grid.step(point, (vx, vy));
                        if let Some(next) = next {
                            energize(grid, energized, next, (vx, vy))
                        }
                    }
                }
                '-' => {
                    if UPDOWN.contains(&(vx, vy)) {
                        for v in RIGHTLEFT {
                            let next = grid.step(point, v);
                            if let Some(next) = next {
                                energize(grid, energized, next, v)
                            }
                        }
                    } else {
                        let next = grid.step(point, (vx, vy));
                        if let Some(next) = next {
                            energize(grid, energized, next, (vx, vy))
                        }
                    }
                }
                '/' => {
                    let out = match (vx, vy) {
                        RIGHT => UP,
                        LEFT => DOWN,
                        DOWN => LEFT,
                        UP => RIGHT,
                        _ => panic!(),
                    };
                    let next = grid.step(point, out);
                    if let Some(next) = next {
                        energize(grid, energized, next, out)
                    }
                }
                '\\' => {
                    let out = match (vx, vy) {
                        RIGHT => DOWN,
                        LEFT => UP,
                        DOWN => RIGHT,
                        UP => LEFT,
                        _ => panic!(),
                    };
                    let next = grid.step(point, out);
                    if let Some(next) = next {
                        energize(grid, energized, next, out)
                    }
                }
                _ => panic!(),
            }
        }
    }

    let grid = Grid::try_from(INPUT).unwrap();
    b.iter(|| assert_eq!(energize(&grid, Point { x: 0, y: 0 }, RIGHT).len(), 7472));
}

#[bench]
fn bench_energize_input_01_ahash(b: &mut test::Bencher) {
    use ahash::AHashMap;

    fn energize(grid: &Grid, start: Point, v: Velocity) -> AHashMap<Point, Vec<Velocity>> {
        let mut energized = AHashMap::new();
        energize(grid, &mut energized, start, v);
        return energized;

        fn energize(
            grid: &Grid,
            energized: &mut AHashMap<Point, Vec<Velocity>>,
            point: Point,
            (vx, vy): Velocity,
        ) {
            let beams = energized.entry(point).or_default();
            if beams.contains(&(vx, vy)) {
                return;
            }
            beams.push((vx, vy));

            match grid[point] {
                '.' => {
                    let next = grid.step(point, (vx, vy));
                    if let Some(next) = next {
                        energize(grid, energized, next, (vx, vy))
                    }
                }
                '|' => {
                    if RIGHTLEFT.contains(&(vx, vy)) {
                        for v in UPDOWN {
                            let next = grid.step(point, v);
                            if let Some(next) = next {
                                energize(grid, energized, next, v)
                            }
                        }
                    } else {
                        let next = grid.step(point, (vx, vy));
                        if let Some(next) = next {
                            energize(grid, energized, next, (vx, vy))
                        }
                    }
                }
                '-' => {
                    if UPDOWN.contains(&(vx, vy)) {
                        for v in RIGHTLEFT {
                            let next = grid.step(point, v);
                            if let Some(next) = next {
                                energize(grid, energized, next, v)
                            }
                        }
                    } else {
                        let next = grid.step(point, (vx, vy));
                        if let Some(next) = next {
                            energize(grid, energized, next, (vx, vy))
                        }
                    }
                }
                '/' => {
                    let out = match (vx, vy) {
                        RIGHT => UP,
                        LEFT => DOWN,
                        DOWN => LEFT,
                        UP => RIGHT,
                        _ => panic!(),
                    };
                    let next = grid.step(point, out);
                    if let Some(next) = next {
                        energize(grid, energized, next, out)
                    }
                }
                '\\' => {
                    let out = match (vx, vy) {
                        RIGHT => DOWN,
                        LEFT => UP,
                        DOWN => RIGHT,
                        UP => LEFT,
                        _ => panic!(),
                    };
                    let next = grid.step(point, out);
                    if let Some(next) = next {
                        energize(grid, energized, next, out)
                    }
                }
                _ => panic!(),
            }
        }
    }

    let grid = Grid::try_from(INPUT).unwrap();
    b.iter(|| assert_eq!(energize(&grid, Point { x: 0, y: 0 }, RIGHT).len(), 7472));
}

#[bench]
fn bench_energize_input_00_std_hasher(b: &mut test::Bencher) {
    use std::collections::HashMap;

    fn energize(grid: &Grid, start: Point, v: Velocity) -> HashMap<Point, Vec<Velocity>> {
        let mut energized = HashMap::default();
        energize(grid, &mut energized, start, v);
        return energized;

        fn energize(
            grid: &Grid,
            energized: &mut HashMap<Point, Vec<Velocity>>,
            point: Point,
            (vx, vy): Velocity,
        ) {
            let beams = energized.entry(point).or_default();
            if beams.contains(&(vx, vy)) {
                return;
            }
            beams.push((vx, vy));

            match grid[point] {
                '.' => {
                    let next = grid.step(point, (vx, vy));
                    if let Some(next) = next {
                        energize(grid, energized, next, (vx, vy))
                    }
                }
                '|' => {
                    if RIGHTLEFT.contains(&(vx, vy)) {
                        for v in UPDOWN {
                            let next = grid.step(point, v);
                            if let Some(next) = next {
                                energize(grid, energized, next, v)
                            }
                        }
                    } else {
                        let next = grid.step(point, (vx, vy));
                        if let Some(next) = next {
                            energize(grid, energized, next, (vx, vy))
                        }
                    }
                }
                '-' => {
                    if UPDOWN.contains(&(vx, vy)) {
                        for v in RIGHTLEFT {
                            let next = grid.step(point, v);
                            if let Some(next) = next {
                                energize(grid, energized, next, v)
                            }
                        }
                    } else {
                        let next = grid.step(point, (vx, vy));
                        if let Some(next) = next {
                            energize(grid, energized, next, (vx, vy))
                        }
                    }
                }
                '/' => {
                    let out = match (vx, vy) {
                        RIGHT => UP,
                        LEFT => DOWN,
                        DOWN => LEFT,
                        UP => RIGHT,
                        _ => panic!(),
                    };
                    let next = grid.step(point, out);
                    if let Some(next) = next {
                        energize(grid, energized, next, out)
                    }
                }
                '\\' => {
                    let out = match (vx, vy) {
                        RIGHT => DOWN,
                        LEFT => UP,
                        DOWN => RIGHT,
                        UP => LEFT,
                        _ => panic!(),
                    };
                    let next = grid.step(point, out);
                    if let Some(next) = next {
                        energize(grid, energized, next, out)
                    }
                }
                _ => panic!(),
            }
        }
    }

    let grid = Grid::try_from(INPUT).unwrap();
    b.iter(|| assert_eq!(energize(&grid, Point { x: 0, y: 0 }, RIGHT).len(), 7472));
}

// we will have to work up to this once we get fast enough?
// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(test_input);
// }
