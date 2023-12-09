// #![feature(test)]

// extern crate test;

use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example08.txt");
const EXAMPLE2: &str = include_str!("example08_2.txt");
const INPUT: &str = include_str!("input08.txt");

fn main() {
    let (instructions, network) = parse(EXAMPLE);
    let steps = simple_follow(&network, &instructions, "AAA");
    dbg!(steps);

    let (instructions, network) = parse(EXAMPLE2);
    let all_steps = ghost_follow(&network, &instructions);
    dbg!(all_steps);

    let (instructions, network) = parse(INPUT);
    let steps = simple_follow(&network, &instructions, "AAA");
    let all_steps = ghost_follow(&network, &instructions);
    dbg!((steps, all_steps));
}

fn ghost_follow(network: &HashMap<&str, (&str, &str)>, instructions: &[char]) -> usize {
    // god, I love rust:
    network
        .keys()
        .cloned()
        .filter(|&k| k.ends_with('A'))
        .inspect(|&k| println!("start: {}", k))
        .map(|location| simple_follow(network, instructions, location))
        .inspect(|&steps| println!("solved: {}", steps))
        .reduce(lcm)
        .expect("no solutions found")
}

fn simple_follow(
    network: &HashMap<&str, (&str, &str)>,
    instructions: &[char],
    start: &str,
) -> usize {
    let mut location = start;
    let mut steps = 0;
    let mut instructions = instructions.iter().cycle();
    while !location.ends_with('Z') {
        let (left, right) = network.get(location).unwrap();

        let next = if *instructions.next().unwrap() == 'L' {
            left
        } else {
            right
        };

        // println!("{steps}: {location} -> {next}");

        location = next;

        steps += 1;
    }

    steps
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

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    // for shame: copilot wrote this one
    // TODO: we do have this already oxidixed example:
    // https://en.wikipedia.org/wiki/Binary_GCD_algorithm#Implementation
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

#[test]
fn test_example() {
    let (instructions, network) = parse(EXAMPLE);

    let steps = simple_follow(&network, &instructions, "AAA");

    assert_eq!(steps, 6);
}

#[test]
fn test_example2() {
    let (instructions, network) = parse(EXAMPLE2);

    let all_steps = ghost_follow(&network, &instructions);
    assert_eq!(all_steps, 6);
}

#[test]
fn test_input() {
    let (instructions, network) = parse(INPUT);

    let steps = simple_follow(&network, &instructions, "AAA");
    let all_steps = ghost_follow(&network, &instructions);

    assert_eq!((steps, all_steps), (18827, 20220305520997));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
