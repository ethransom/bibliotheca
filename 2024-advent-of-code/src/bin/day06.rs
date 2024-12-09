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
    let (map, mut guard, height, width) = parse(input);

    let mut dir = CLOCKWISE_DIRS[0];

    let mut visited = HashSet::<((isize, isize), (isize, isize))>::new();

    let mut loops = 0;

    loop {
        // println!();
        // for y in 0..height {
        //     for x in 0..width {
        //         if (x, y) == guard {
        //             print!("^");
        //         }
        //         let c = map.get(&(x, y)).unwrap();
        //         print!("{c}");
        //     }
        //     println!();
        // }

        if visited.contains(&(guard, rotate(dir))) {
            loops += 1;
            let loop_loc = (guard.0 + dir.0, guard.1 + dir.1);
            println!("LOOP LOC: {loop_loc:?}");
            let dirs = visited.iter().map(|(pos, dir)| {
                (
                    pos,
                    match dir {
                        (0, -1) => '|',
                        (1, 0) => '-',
                        (0, 1) => '|',
                        (-1, 0) => '-',
                        _ => panic!(),
                    },
                )
            });
            let mut dirs_dirs = HashMap::<(isize, isize), HashSet<char>>::new();
            for (pos, dir) in dirs {
                dirs_dirs
                    .entry(*pos)
                    .and_modify(|s| {
                        s.insert(dir);
                    })
                    .or_insert(HashSet::from([dir]));
            }
            println!();
            for y in 0isize..=height as isize {
                for x in 0isize..=width as isize {
                    let c = if (x, y) == loop_loc {
                        'O'
                    } else if (x, y) == guard {
                        '^'
                    } else if let Some(dirs) = dirs_dirs.get(&(x, y)) {
                        if dirs.len() == 2 {
                            '+'
                        } else {
                            *dirs.iter().next().unwrap()
                        }
                    } else {
                        *map.get(&(x, y)).unwrap()
                    };
                    print!("{c}");
                }
                println!();
            }
        }

        visited.insert((guard, dir));

        let next = (guard.0 + dir.0, guard.1 + dir.1);
        let Some(next_c) = map.get(&next) else {
            break;
        };
        match next_c {
            '.' => {
                guard = next;
            }
            '#' => {
                dir = rotate(dir);
            }
            _ => panic!(),
        };
    }

    #[cfg(debug_assertions)]
    {
        let dirs = visited.iter().map(|(pos, dir)| {
            (
                pos,
                match dir {
                    (0, -1) => '|',
                    (1, 0) => '-',
                    (0, 1) => '|',
                    (-1, 0) => '-',
                    _ => panic!(),
                },
            )
        });
        let mut dirs_dirs = HashMap::<(isize, isize), HashSet<char>>::new();
        for (pos, dir) in dirs {
            dirs_dirs
                .entry(*pos)
                .and_modify(|s| {
                    s.insert(dir);
                })
                .or_insert(HashSet::from([dir]));
        }
        println!();
        for y in 0isize..=height as isize {
            for x in 0isize..=width as isize {
                let c = if (x, y) == guard {
                    '^'
                } else if let Some(dirs) = dirs_dirs.get(&(x, y)) {
                    if dirs.len() == 2 {
                        '+'
                    } else {
                        *dirs.iter().next().unwrap()
                    }
                } else {
                    *map.get(&(x, y)).unwrap()
                };
                print!("{c}");
            }
            println!();
        }
    }

    (
        visited
            .iter()
            .cloned()
            .map(|(pos, _)| pos)
            .collect::<HashSet<(isize, isize)>>()
            .len(),
        loops,
    )
}

fn rotate(dir: (isize, isize)) -> (isize, isize) {
    CLOCKWISE_DIRS
        [(CLOCKWISE_DIRS.iter().position(|d| d == &dir).unwrap() + 1) % CLOCKWISE_DIRS.len()]
}

#[allow(clippy::type_complexity)]
fn parse(input: &str) -> (HashMap<(isize, isize), char>, (isize, isize), usize, usize) {
    let mut map = HashMap::new();

    let mut start = None;

    let mut height = 0;
    let mut width = 0;

    for (y, line) in input.lines().enumerate() {
        height = height.max(y);
        for (x, mut c) in line.chars().enumerate() {
            width = width.max(x);
            if c == '^' {
                start = Some((x as isize, y as isize));
                c = '.';
            }
            map.insert((x as isize, y as isize), c);
        }
    }

    let start = start.expect("no guard on map");

    (map, start, height, width)
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (41, 6));
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
