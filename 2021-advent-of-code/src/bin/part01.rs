#![feature(test)]

const EXAMPLE: &[u8] = include_bytes!("example01.txt");
const INPUT: &[u8] = include_bytes!("input01.txt");

fn main() {
    let example = parse(EXAMPLE);
    println!("example:");
    println!("\t{:?}", solve(&example));
    println!("\t{:?}", solve_windowed(&example));
    let input = parse(INPUT);
    println!("input:");
    println!("\t{:?}", solve(&input));
    println!("\t{:?}", solve_windowed(&input));
}

fn parse(dat: &[u8]) -> Vec<u64> {
    std::str::from_utf8(dat)
        .expect("bad input file!")
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect()
}

pub fn solve(nums: &Vec<u64>) -> (u64, u64) {
    let mut simple_increases = 0;
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            simple_increases += 1;
        }
    }

    let mut sums: Vec<u64> = Vec::new();

    for i in 0..nums.len() - 2 {
        sums.push(nums[i] + nums[i + 1] + nums[i + 2]);
    }

    let mut windowed_increases = 0;
    for i in 1..sums.len() {
        if sums[i] > sums[i - 1] {
            windowed_increases += 1;
        }
    }

    (simple_increases, windowed_increases)
}

pub fn solve_windowed(nums: &Vec<u64>) -> (usize, usize) {
    let simple_increases = nums.windows(2).filter(|w| w[1] > w[0]).count();

    let windowed_increases = nums
        .windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .collect::<Vec<u64>>()
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count();

    (simple_increases, windowed_increases)
}

extern crate test;

#[bench]
fn bench_parse(b: &mut test::Bencher) {
    b.iter(|| {
        parse(INPUT);
    });
}

#[bench]
fn bench_parse_bytes(b: &mut test::Bencher) {
    b.iter(|| {
        INPUT
            .split(|b| *b == '\n' as u8)
            .map(|line| {
                std::str::from_utf8(line)
                    .expect("not utf8")
                    .parse::<u64>()
                    .expect("not a number")
            })
            .collect::<Vec<u64>>()
    });
}

#[bench]
fn bench_parse_bytes_unsafe(b: &mut test::Bencher) {
    b.iter(|| {
        INPUT
            .split(|b| *b == '\n' as u8)
            .map(|line| {
                unsafe { std::str::from_utf8_unchecked(line) }
                    .parse::<u64>()
                    .expect("not a number")
            })
            .collect::<Vec<u64>>()
    });
}

#[bench]
fn bench_solve(b: &mut test::Bencher) {
    let input = parse(INPUT);

    b.iter(|| {
        solve(&input);
    });
}

#[bench]
fn bench_solve_windowed(b: &mut test::Bencher) {
    let input = parse(INPUT);

    b.iter(|| {
        solve_windowed(&input);
    });
}
