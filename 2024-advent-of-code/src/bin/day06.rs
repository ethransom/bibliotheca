#![feature(test)]
#![feature(let_chains)]

extern crate test;

use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};

const EXAMPLE: &str = include_str!("example06.txt");
const INPUT: &str = include_str!("input06.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

const CLOCKWISE_DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Clone)]
struct Map {
    map: HashMap<(isize, isize), char>,
    height: usize,
    width: usize,
}

fn walk(
    map: &Map,
    mut guard: (isize, isize),
    mut dir: (isize, isize),
    obstacle: bool,
    visited: &mut HashSet<((isize, isize), (isize, isize))>,
    loops: &mut usize,
) -> bool {
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
        //
        //

        if !visited.insert((guard, dir)) {
            return true;
        }

        if !obstacle {
            let loop_loc = (guard.0 + dir.0, guard.1 + dir.1);

            let is_empty = if let Some(c) = map.map.get(&(loop_loc)) {
                *c == '.'
            } else {
                false
            };
            let is_unvisited = !visited.iter().any(|(pos, _dir)| pos == &loop_loc);

            if is_empty && is_unvisited {
                let mut map = map.clone();
                map.map.insert(loop_loc, '#');

                if walk(&map, guard, rotate(dir), true, &mut visited.clone(), loops) {
                    *loops += 1;

                    // println!("LOOP LOC: {loop_loc:?}");

                    // let dirs = visited.iter().map(|(pos, dir)| {
                    //     (
                    //         pos,
                    //         match dir {
                    //             (0, -1) => '|',
                    //             (1, 0) => '-',
                    //             (0, 1) => '|',
                    //             (-1, 0) => '-',
                    //             _ => panic!(),
                    //         },
                    //     )
                    // });
                    // let mut dirs_dirs = HashMap::<(isize, isize), HashSet<char>>::new();
                    // for (pos, dir) in dirs {
                    //     dirs_dirs
                    //         .entry(*pos)
                    //         .and_modify(|s| {
                    //             s.insert(dir);
                    //         })
                    //         .or_insert(HashSet::from([dir]));
                    // }
                    // println!();
                    // for y in 0isize..=map.height as isize {
                    //     for x in 0isize..=map.width as isize {
                    //         let c = if (x, y) == loop_loc {
                    //             'O'
                    //         } else if (x, y) == guard {
                    //             '^'
                    //         } else if let Some(dirs) = dirs_dirs.get(&(x, y)) {
                    //             if dirs.len() == 2 {
                    //                 '+'
                    //             } else {
                    //                 *dirs.iter().next().unwrap()
                    //             }
                    //         } else {
                    //             *map.map.get(&(x, y)).unwrap()
                    //         };
                    //         print!("{c}");
                    //     }
                    //     println!();
                    // }
                }
            }
        }

        let next = (guard.0 + dir.0, guard.1 + dir.1);
        let Some(next_c) = map.map.get(&next) else {
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

    false
}

fn solve(input: &str) -> (usize, usize) {
    let (map, guard) = parse(input);

    let dir = CLOCKWISE_DIRS[0];

    let mut visited = HashSet::<((isize, isize), (isize, isize))>::default();

    let mut loops = 0;

    if walk(&map, guard, dir, false, &mut visited, &mut loops) {
        panic!("everything is a loop, guard never leaves");
    }

    // let dirs = visited.iter().map(|(pos, dir)| {
    //     (
    //         pos,
    //         match dir {
    //             (0, -1) => '|',
    //             (1, 0) => '-',
    //             (0, 1) => '|',
    //             (-1, 0) => '-',
    //             _ => panic!(),
    //         },
    //     )
    // });
    // let mut dirs_dirs = HashMap::<(isize, isize), HashSet<char>>::new();
    // for (pos, dir) in dirs {
    //     dirs_dirs
    //         .entry(*pos)
    //         .and_modify(|s| {
    //             s.insert(dir);
    //         })
    //         .or_insert(HashSet::from([dir]));
    // }
    // println!();
    // for y in 0isize..=map.height as isize {
    //     for x in 0isize..=map.width as isize {
    //         let c = if (x, y) == guard {
    //             '^'
    //         } else if let Some(dirs) = dirs_dirs.get(&(x, y)) {
    //             if dirs.len() == 2 {
    //                 '+'
    //             } else {
    //                 *dirs.iter().next().unwrap()
    //             }
    //         } else {
    //             *map.map.get(&(x, y)).unwrap()
    //         };
    //         print!("{c}");
    //     }
    //     println!();
    // }

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

fn parse(input: &str) -> (Map, (isize, isize)) {
    let mut map = HashMap::default();

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

    (Map { map, height, width }, start)
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (41, 6));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (5086, 1770));
}

#[bench]
fn bench_solve_00_original(b: &mut test::Bencher) {
    use std::collections::{HashMap, HashSet};

    #[derive(Clone)]
    struct Map {
        map: HashMap<(isize, isize), char>,
        height: usize,
        width: usize,
    }

    fn walk(
        map: &Map,
        mut guard: (isize, isize),
        mut dir: (isize, isize),
        obstacle: bool,
        visited: &mut HashSet<((isize, isize), (isize, isize))>,
        loops: &mut usize,
    ) -> bool {
        loop {
            if !visited.insert((guard, dir)) {
                return true;
            }

            if !obstacle {
                let loop_loc = (guard.0 + dir.0, guard.1 + dir.1);

                let is_empty = if let Some(c) = map.map.get(&(loop_loc)) {
                    *c == '.'
                } else {
                    false
                };
                let is_unvisited = !visited.iter().any(|(pos, _dir)| pos == &loop_loc);

                if is_empty && is_unvisited {
                    let mut map = map.clone();
                    map.map.insert(loop_loc, '#');

                    if walk(&map, guard, rotate(dir), true, &mut visited.clone(), loops) {
                        *loops += 1;
                    }
                }
            }

            let next = (guard.0 + dir.0, guard.1 + dir.1);
            let Some(next_c) = map.map.get(&next) else {
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

        false
    }

    fn solve(input: &str) -> (usize, usize) {
        let (map, guard) = parse(input);

        let dir = CLOCKWISE_DIRS[0];

        let mut visited = HashSet::<((isize, isize), (isize, isize))>::new();

        let mut loops = 0;

        if walk(&map, guard, dir, false, &mut visited, &mut loops) {
            panic!("everything is a loop, guard never leaves");
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

    fn parse(input: &str) -> (Map, (isize, isize)) {
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

        (Map { map, height, width }, start)
    }

    b.iter(|| {
        assert_eq!(solve(EXAMPLE), (41, 6));
    });
}

#[bench]
fn bench_solve_01_current_fxhash(b: &mut test::Bencher) {
    b.iter(test_example);
}
