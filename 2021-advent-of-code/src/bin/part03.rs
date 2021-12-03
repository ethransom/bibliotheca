#![feature(test)]

const EXAMPLE: &[u8] = include_bytes!("example03.txt");
const INPUT: &[u8] = include_bytes!("input03.txt");

fn main() {
    println!("Example:");
    let (one, two) = solve(EXAMPLE);
    println!("\tpart1: {}\n\tpart2: {}", one, two);
    println!("Input:");
    let (one, two) = solve(INPUT);
    println!("\tpart1: {}\n\tpart2: {}", one, two);
}

fn solve(input: &[u8]) -> (u32, u32) {
    let lines = parse(input);

    (part1(&lines), part2(&lines))
}

fn parse(input: &[u8]) -> Vec<&[u8]> {
    input.split(|b| *b == '\n' as u8).collect()
}

fn part1(lines: &[&[u8]]) -> u32 {
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..lines[0].len() {
        gamma = gamma << 1;
        epsilon = epsilon << 1;

        let ones = lines.iter().filter(|line| line[i] == '1' as u8).count();
        if ones >= lines.len() / 2 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    gamma * epsilon
}

fn part2(input: &Vec<&[u8]>) -> u32 {
    let oxygen = prune(input, Keep::Majority);

    let co2 = prune(input, Keep::Minority);

    oxygen * co2
}

enum Keep {
    Majority,
    Minority,
}

fn prune(input: &Vec<&[u8]>, keep: Keep) -> u32 {
    let mut kept_values = input.clone();
    for i in 0..input[0].len() {
        let ones = kept_values
            .iter()
            .filter(|line| line[i] == '1' as u8)
            .count();
        let majority: char;
        if ones * 2 >= kept_values.len() {
            majority = '1';
        } else {
            majority = '0';
        }
        kept_values = kept_values
            .into_iter()
            .filter(|line| match keep {
                Keep::Majority => line[i] == majority as u8,
                Keep::Minority => line[i] != majority as u8,
            })
            .collect::<Vec<&[u8]>>();

        if kept_values.len() == 1 {
            break;
        }
    }
    assert!(kept_values.len() == 1);
    u32::from_str_radix(
        std::str::from_utf8(kept_values[0]).expect("not a string"),
        2,
    )
    .expect("not a binary number")
}

#[test]
fn it_handles_the_example_input() {
    assert_eq!(solve(EXAMPLE), (198, 230));
}

extern crate test;

#[bench]
fn bench_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (3923414, 5852595));
    });
}

#[bench]
fn bench_parse_00_original(b: &mut test::Bencher) {
    b.iter(|| parse(INPUT));
}

#[bench]
fn bench_solve_00_original(b: &mut test::Bencher) {
    let lines = parse(INPUT);
    b.iter(|| (part1(&lines), part2(&lines)));
}
