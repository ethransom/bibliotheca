#![feature(test)]

extern crate test;

use std::collections::HashSet;

const EXAMPLE: &str = include_str!("example25.txt");
const INPUT: &str = include_str!("input25.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let blocks = Region::from(input);

    (0, 0)
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Region {
    east: HashSet<(usize, usize)>,
    south: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
}

impl From<&str> for Region {
    fn from(input: &str) -> Region {
        let mut east: HashSet<(usize, usize)> = HashSet::new();
        let mut south: HashSet<(usize, usize)> = HashSet::new();

        let mut width: usize = 0;
        let mut height: usize = 0;

        for (row, line) in input.lines().enumerate() {
            for (col, char) in line.chars().enumerate() {
                dbg!(col, row, char);
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

        dbg!(&east, &south);

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
fn test_example() {
    assert_eq!(solve(EXAMPLE), (0, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (0, 0));
    });
}
