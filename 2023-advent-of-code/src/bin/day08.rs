// #![feature(test)]

// extern crate test;

use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example08.txt");
const INPUT: &str = include_str!("input08.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let (instructions, network) = parse(input);

    let mut location = "AAA";
    let mut steps = 0;
    let mut instructions = instructions.iter().cycle();
    while location != "ZZZ" {
        let (left, right) = network.get(location).unwrap();

        location = if *instructions.next().unwrap() == 'L' {
            left
        } else {
            right
        };

        steps += 1;
    }

    (steps, 0)
}

fn parse(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let (instructions, network) = input.split_once("\n\n").unwrap();

    let instructions = instructions.chars().collect::<Vec<_>>();

    let network = network
        .lines()
        .map(|line| {
            let (key, value) = line.split_once(" = ").unwrap();
            let (left, right) = (&value[1..=3], &value[6..=8]);

            (key, (left, right))
        })
        .collect();

    (instructions, network)
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (6, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (18827, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
