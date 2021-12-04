#![feature(test)]

extern crate test;

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
    let (width, nums) = parse(input);

    (part1(width, &nums), part2(width, &nums))
}

fn parse(input: &[u8]) -> (usize, Vec<u32>) {
    let lines = input
        .split(|b| *b == '\n' as u8)
        .map(|line| {
            line.iter().fold(0u32, |acc, b| {
                let bit = (*b == '1' as u8) as u32;
                (acc << 1) + bit
            })
        })
        .collect::<Vec<u32>>();
    dbg!(input.len(), lines.len());
    ((input.len() + 1) / lines.len() - 1, lines)
}

fn part1(width: usize, nums: &[u32]) -> u32 {
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..width {
        gamma = gamma << 1;
        epsilon = epsilon << 1;

        let mask = 1 << (width - 1 - i);

        let ones = nums.iter().filter(|num| (*num & mask) > 0).count();
        if ones >= nums.len() / 2 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    gamma * epsilon
}

fn part2(width: usize, input: &Vec<u32>) -> u32 {
    let oxygen = prune(width, input, Keep::Majority);

    let co2 = prune(width, input, Keep::Minority);

    oxygen * co2
}

enum Keep {
    Majority,
    Minority,
}

fn prune(width: usize, input: &Vec<u32>, keep: Keep) -> u32 {
    let mut kept_values = input.clone();
    for i in 0..width {
        let mask = 1 << (width - 1 - i);

        let ones = kept_values.iter().filter(|num| (*num & mask) > 0).count();
        let majority = if ones * 2 >= kept_values.len() { 1 } else { 0 };
        kept_values = kept_values
            .into_iter()
            .filter(|num| match (majority, &keep) {
                (1, Keep::Majority) => (num & mask) > 0,
                (1, Keep::Minority) => (num & mask) == 0,
                (0, Keep::Majority) => (num & mask) == 0,
                (0, Keep::Minority) => (num & mask) > 0,
                _ => unimplemented!(
                    "just gotta get this working then we can eliminate this match :("
                ),
            })
            .collect();

        if kept_values.len() == 1 {
            break;
        }
    }
    assert!(kept_values.len() == 1);
    kept_values[0]
}
#[test]
fn it_handles_the_example_input() {
    assert_eq!(solve(EXAMPLE), (198, 230));
}

#[bench]
fn bench_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (3923414, 5852595));
    });
}

#[bench]
fn bench_parse_01_str_ints(b: &mut test::Bencher) {
    b.iter(|| {
        INPUT
            .split(|b| *b == '\n' as u8)
            .map(|line| {
                u32::from_str_radix(std::str::from_utf8(line).expect("not a string"), 2)
                    .expect("not a binary number")
            })
            .collect::<Vec<u32>>()
    });
}

#[bench]
fn bench_parse_02_custom_ints(b: &mut test::Bencher) {
    b.iter(|| parse(INPUT));
}

#[bench]
fn bench_solve_00_original(b: &mut test::Bencher) {
    b.iter(|| {
        let lines = parse_original(INPUT);
        (part1_original(&lines), part2_original(&lines))
    });
}

#[bench]
fn bench_solve_01_integers(b: &mut test::Bencher) {
    b.iter(|| {
        let (width, nums) = parse(INPUT);
        assert_eq!(
            (part1(width, &nums), part2(width, &nums)),
            (3923414, 5852595)
        );
    });
}

#[cfg(test)]
fn parse_original(input: &[u8]) -> Vec<&[u8]> {
    input.split(|b| *b == '\n' as u8).collect()
}

#[cfg(test)]
fn part1_original(lines: &[&[u8]]) -> u32 {
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

#[cfg(test)]
fn part2_original(input: &Vec<&[u8]>) -> u32 {
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
    let oxygen = prune(input, Keep::Majority);

    let co2 = prune(input, Keep::Minority);

    oxygen * co2
}

#[test]
fn original_it_handles_the_example_input() {
    assert_eq!(self::solve(EXAMPLE), (198, 230));
}

#[bench]
fn bench_parse_00_original(b: &mut test::Bencher) {
    b.iter(|| parse_original(INPUT));
}

#[bench]
fn bench_part1_current(b: &mut test::Bencher) {
    let (width, nums) = parse(INPUT);
    b.iter(|| part1(width, &nums));
}

#[bench]
fn bench_part1_original(b: &mut test::Bencher) {
    let lines = parse_original(INPUT);
    b.iter(|| part1_original(&lines));
}

#[bench]
fn bench_part2_current(b: &mut test::Bencher) {
    let (width, nums) = parse(INPUT);
    b.iter(|| part2(width, &nums));
}

#[bench]
fn bench_part2_original(b: &mut test::Bencher) {
    let lines = parse_original(INPUT);
    b.iter(|| part2_original(&lines));
}
