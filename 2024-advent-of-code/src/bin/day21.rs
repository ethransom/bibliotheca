// #![feature(test)]

// extern crate test;

use std::collections::{HashMap, VecDeque};

const EXAMPLE: &str = include_str!("example21.txt");
const INPUT: &str = include_str!("input21.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

const LAYERS: &[fn(&[char]) -> Vec<char>] = &[
    numeric_sequence,
    directional_sequence,
    directional_sequence,
    directional_sequence,
];

fn solve(input: &str) -> (usize, usize) {
    let codes = parse(input);

    (
        codes
            .iter()
            .map(|code| {
                println!("\n{code:?}", code = s(code));

                let (_activate, numeric) = code.split_last().unwrap();

                dbg!(s(numeric).parse::<usize>().unwrap()) * dbg!(type_through_layers(code).len())
            })
            .sum(),
        0,
    )
}

fn type_through_layers(code: &[char]) -> Vec<char> {
    LAYERS.iter().fold(code.to_owned(), |code, layer| {
        // apply
        let code = layer(&code);
        println!("{code:?}", code = s(&code));
        code
    })
}

#[test]
fn test_type_through_layers() {
    let code = "029A".chars().collect::<Vec<_>>();
    assert_eq!(
        s(&type_through_layers(&code)),
        "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"
    );
}

fn s(input: &[char]) -> String {
    input.iter().collect()
}

const DIRECTIONAL_KEYPAD: &[&[char]] = &[
    // I'm here to force multiline
    &[' ', '^', 'A'],
    &['<', 'v', '>'],
];

fn directional_sequence(input: &[char]) -> Vec<char> {
    sequence(DIRECTIONAL_KEYPAD, input)
}

const NUMERIC_KEYPAD: &[&[char]] = &[
    &['7', '8', '9'],
    &['4', '5', '6'],
    &['1', '2', '3'],
    &[' ', '0', 'A'],
];

fn numeric_sequence(input: &[char]) -> Vec<char> {
    sequence(NUMERIC_KEYPAD, input)
}

fn sequence(keypad: &[&[char]], input: &[char]) -> Vec<char> {
    // TODO: cache this between calls somehow
    let mut positions = HashMap::new();
    let mut buttons = HashMap::new();
    for (y, row) in keypad.iter().enumerate() {
        let y = y as isize;
        for (x, &c) in row.iter().enumerate() {
            let x = x as isize;
            if c == ' ' {
                continue;
            }

            buttons.insert((x, y), c);
            positions.insert(c, (x, y));
        }
    }

    let mut pos = *positions.get(&'A').unwrap();
    input
        .iter()
        .flat_map(|c| {
            let dest = *positions.get(c).expect("char not on keypad");
            let mut path = pathfind(&buttons, pos, dest);

            path.push('A');

            println!("\tpath from {pos:?} to {dest:?}: {path:?}", path = s(&path));

            pos = dest;

            path
        })
        .collect()
}

#[test]
fn test_numeric_sequence() {
    let paths = ["<A^A>^^AvvvA", "<A^A^>^AvvvA", "<A^A^^>AvvvA"].map(str::to_string);
    let seq = "029A".chars().collect::<Vec<_>>();
    assert!(paths.contains(&numeric_sequence(&seq).into_iter().collect()));
}

type Point = (isize, isize);

const NEIGHBORS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn dir_to_char(dir: Point) -> char {
    match dir {
        (0, -1) => '^',
        (1, 0) => '>',
        (0, 1) => 'v',
        (-1, 0) => '<',
        _ => panic!(),
    }
}

// shamelessly plagarized from day18 lol
fn pathfind(keypad: &HashMap<Point, char>, start: Point, end: Point) -> Vec<char> {
    let mut frontier = VecDeque::new();
    frontier.push_back(start);

    let mut dists = HashMap::<Point, usize>::default();
    dists.insert(start, 0);

    let mut prev = HashMap::<Point, Point>::default();

    while let Some(current) = frontier.pop_front() {
        let current_dist = dists[&current];
        if current == end {
            let mut path = vec![];
            let mut pointer = end;
            while pointer != start {
                let last = prev[&pointer];
                let dir = (pointer.0 - last.0, pointer.1 - last.1);
                path.push(dir_to_char(dir));
                pointer = last;
            }

            return path;

            // for y in 0..=end.1 {
            //     for x in 0..=end.0 {
            //         let c = if corrupted.contains(&(x, y)) {
            //             '#'
            //         } else if path.contains(&(x, y)) {
            //             'O'
            //         } else {
            //             '.'
            //         };
            //         print!("{c}");
            //     }
            //     println!();
            // }

            // return Some(current_dist);
        }

        for neighbor in NEIGHBORS.map(|(dx, dy)| (current.0 + dx, current.1 + dy)) {
            let (x, y) = neighbor;

            if keypad.get(&(x, y)).is_none() {
                continue;
            }

            let dist = current_dist + 1;
            if dists.get(&(x, y)).is_none_or(|&d| dist < d) {
                dists.insert((x, y), dist);
                prev.insert(neighbor, current);
                frontier.push_back(neighbor);
            }
        }
    }

    unreachable!("no path from {start:?} to {end:?} on keypad");
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| {
            // foo
            line.chars().collect()
        })
        .collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (126384, 0));
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
