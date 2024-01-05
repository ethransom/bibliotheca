// #![feature(test)]

// extern crate test;

use std::collections::HashMap;

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

    let energized = energize(&grid);

    println!("{}", grid);

    println!();

    println!("{}", pretty_print_beams(&grid, &energized));

    println!();

    println!("{}", pretty_print_energized(&grid, &energized));

    println!();

    (energized.len(), 0)
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

fn energize(grid: &Grid) -> HashMap<Point, Vec<Velocity>> {
    let mut energized = HashMap::new();
    energize(grid, &mut energized, Point { x: 0, y: 0 }, RIGHT);
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
// enum Orientation {
//     NorthSouth,
//     EastWest,
// }
//
// enum Cell {
//     Empty,
//     Mirror(Orientation),
//     Splitter(Orientation),
// }

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
    let energized = energize(&grid);
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
    assert_eq!(solve(EXAMPLE), (46, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (7472, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
