// #![feature(test)]

// extern crate test;

const EXAMPLE: &str = include_str!("example16.txt");
const INPUT: &str = include_str!("input16.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let grid = Grid::try_from(input).unwrap_or_else(|err| panic!("couldn't parse grid: {err}"));

    println!("{}", grid);

    (0, 0)
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

impl std::ops::Index<(usize, usize)> for Grid {
    type Output = char;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let i = (self.width * y) + x;
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
                write!(f, "{}", self[(x, y)])?;
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
