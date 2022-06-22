#![feature(test)]

extern crate test;

use std::collections::HashSet;
use std::fmt::{Debug, Formatter};

const EXAMPLE: &str = include_str!("example25.txt");
const INPUT: &str = include_str!("input25.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let region = Region::from(input).step();

    (0, 0)
}

#[derive(Default, PartialEq, Eq)]
struct Region {
    east: HashSet<(usize, usize)>,
    south: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Debug for Region {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let c = if self.east.contains(&(col, row)) {
                    '>'
                } else if self.south.contains(&(col, row)) {
                    'v'
                } else {
                    '.'
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Region {
    fn empty(&self, pos: &(usize, usize)) -> bool {
        self.east.contains(&pos) || self.south.contains(&pos)
    }

    fn step(&self) -> Region {
        // east moves first
        let region = Region {
            east: HashSet::from_iter(self.east.iter().cloned().map(|(x, y)| {
                let next = ((x + 1) % self.width, y);
                if self.empty(&next) {
                    (x, y)
                } else {
                    next
                }
            })),
            south: self.south.clone(),
            width: self.width,
            height: self.height,
        };

        Region {
            east: region.east.clone(),
            south: HashSet::from_iter(self.south.iter().cloned().map(|(x, y)| {
                let next = (x, (y + 1) % self.height);
                if region.empty(&next) {
                    (x, y)
                } else {
                    next
                }
            })),
            width: self.width,
            height: self.height,
        }
    }
}

impl From<&str> for Region {
    fn from(input: &str) -> Region {
        let mut east: HashSet<(usize, usize)> = Default::default();
        let mut south: HashSet<(usize, usize)> = Default::default();

        let mut width: usize = 0;
        let mut height: usize = 0;

        for (row, line) in input.lines().enumerate() {
            for (col, char) in line.chars().enumerate() {
                match char {
                    '>' => &mut east,
                    'v' => &mut south,
                    _ => continue,
                }
                .insert((col, row));
            }

            width = usize::max(width, line.chars().count());
            height += 1;
        }

        Region {
            east,
            south,
            width,
            height,
        }
    }
}

#[test]
fn test_region_from_str() {
    let str = "...>>>>>...";
    assert_eq!(
        Region::from(str),
        Region {
            east: HashSet::from([(3, 0), (4, 0), (5, 0), (6, 0), (7, 0)]),
            south: Default::default(),
            width: str.len(),
            height: 1,
        }
    )
}

#[test]
fn test_region_step() {
    assert_eq!(
        Region::from("...>>>>>...").step(),
        Region::from("...>>>>.>..")
    );
    assert_eq!(
        Region::from("...>>>>>...").step().step(),
        Region::from("...>>>.>.>.")
    );
    assert_eq!(
        Region::from(
            "..........\n\
            .>v....v..\n\
            .......>..\n\
            .........."
        )
        .step(),
        Region::from(
            "..........\n\
            .>........\n\
            ..v....v>.\n\
            .........."
        )
    );
    assert_eq!(Region::from("...>>>>>").step(), Region::from(">..>>>>."));
    assert_eq!(Region::from("..\nv.").step(), Region::from("v.\n.."));
    assert_eq!(
        Region::from(
            "...>...\n\
            .......\n\
            ......>\n\
            v.....>\n\
            ......>\n\
            .......\n\
            ..vvv.."
        )
        .step(),
        Region::from(
            "..vv>..\n\
            .......\n\
            >......\n\
            v.....>\n\
            >......\n\
            .......\n\
            ....v.."
        )
    );
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (0, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (0, 0));
    });
}
