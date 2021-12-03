#![feature(test)]

const EXAMPLE: &[u8] = include_bytes!("example03.txt");
const INPUT: &[u8] = include_bytes!("input03.txt");

fn main() {
    let example: Vec<&[u8]> = EXAMPLE.split(|b| *b == '\n' as u8).collect();
    let input: Vec<&[u8]> = INPUT.split(|b| *b == '\n' as u8).collect();
    println!("EXAMPLE:");
    println!("\t1: {}", part1(&example));
    println!("\t2: {}", part2(&example));
    println!("INPUT:");
    println!("\t1: {}", part1(&input));
    println!("\t2: {}", part2(&input));
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
    let example: Vec<&[u8]> = EXAMPLE.split(|b| *b == '\n' as u8).collect();
    assert_eq!(part1(&example), 198);
    assert_eq!(part2(&example), 230);
}

extern crate test;

#[bench]
fn bench_current(b: &mut test::Bencher) {
    b.iter(|| {
        let input: Vec<&[u8]> = INPUT.split(|b| *b == '\n' as u8).collect();
        assert_eq!(part1(&input), 3923414);
        assert_eq!(part2(&input), 5852595);
    });
}
