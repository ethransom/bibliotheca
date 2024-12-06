// #![feature(test)]

// extern crate test;

use std::collections::{HashMap, HashSet};

const EXAMPLE: &str = include_str!("example06.txt");
const INPUT: &str = include_str!("input06.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

const CLOCKWISE_DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn solve(input: &str) -> (usize, usize) {
    let (map, mut guard) = parse(input);

    let mut dir = (0, -1);

    let mut visited = HashSet::<(isize, isize)>::new();

    loop {
        // println!();
        // for y in 0..10 {
        //     for x in 0..10 {
        //         if (x, y) == guard {
        //             print!("^");
        //         }
        //         let c = map.get(&(x, y)).unwrap();
        //         print!("{c}");
        //     }
        //     println!();
        // }

        visited.insert(guard);

        let next = (guard.0 + dir.0, guard.1 + dir.1);
        let Some(next_c) = map.get(&next) else {
            break;
        };
        match next_c {
            '.' => {
                guard = next;
            }
            '#' => {
                dir = CLOCKWISE_DIRS[(CLOCKWISE_DIRS.iter().position(|d| d == &dir).unwrap() + 1)
                    % CLOCKWISE_DIRS.len()];
            }
            _ => panic!(),
        };
    }

    (visited.len(), 0)
}

fn parse(input: &str) -> (HashMap<(isize, isize), char>, (isize, isize)) {
    let mut map = HashMap::new();

    let mut start = None;

    for (y, line) in input.lines().enumerate() {
        for (x, mut c) in line.chars().enumerate() {
            if c == '^' {
                start = Some((x as isize, y as isize));
                c = '.';
            }
            map.insert((x as isize, y as isize), c);
        }
    }

    let start = start.expect("no guard on map");

    (map, start)
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (41, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (5086, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
