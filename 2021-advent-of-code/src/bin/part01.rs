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

pub fn solve(nums: &[u64]) -> (u64, u64) {
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

// ????????????????????????????????????????????????????????????????????????????
// ??                                                                        ??
// ??  Why in the world is this significantly faster than passing a vector?  ??
// ??                                                                        ??
// ????????????????????????????????????????????????????????????????????????????
pub fn solve_windowed_slice(nums: &[u64]) -> (usize, usize) {
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

pub fn solve_windowed_sum_slice(nums: &[u64]) -> (usize, usize) {
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
fn bench_parse_00(b: &mut test::Bencher) {
    b.iter(|| {
        parse(INPUT);
    });
}

#[bench]
fn bench_parse_01_unsafe_str(b: &mut test::Bencher) {
    let str = unsafe { std::str::from_utf8_unchecked(INPUT) };
    b.iter(|| {
        str.lines()
            .map(|line| line.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    });
}

#[bench]
fn bench_parse_02_bytes(b: &mut test::Bencher) {
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
fn bench_parse_03_bytes_flat_map(b: &mut test::Bencher) {
    b.iter(|| {
        INPUT
            .split(|b| *b == '\n' as u8)
            .flat_map(|line| unsafe { std::str::from_utf8_unchecked(line) }.parse::<u32>())
            .collect::<Vec<u32>>()
    });
}

#[bench]
fn bench_parse_04_bytes_unsafe(b: &mut test::Bencher) {
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
fn bench_parse_05_bytes_u16(b: &mut test::Bencher) {
    b.iter(|| {
        INPUT
            .split(|b| *b == '\n' as u8)
            .map(|line| {
                unsafe { std::str::from_utf8_unchecked(line) }
                    .parse::<u16>()
                    .expect("not a number")
            })
            .collect::<Vec<u16>>()
    });
}

#[bench]
fn bench_parse_06_bytes_custom(b: &mut test::Bencher) {
    b.iter(|| {
        INPUT
            .split(|b| *b == '\n' as u8)
            .map(|line| {
                line.iter()
                    .fold(0 as u64, |n, b| n * 10 + (b - ('0' as u8)) as u64)
            })
            .collect::<Vec<u64>>()
    });
}

#[bench]
fn bench_parse_07_bytes_custom_u16(b: &mut test::Bencher) {
    b.iter(|| {
        INPUT
            .split(|b| *b == '\n' as u8)
            .map(|line| {
                line.iter()
                    .fold(0 as u16, |n, b| n * 10 + (b - ('0' as u8)) as u16)
            })
            .collect::<Vec<u16>>()
    });
}

#[bench]
fn bench_parse_08_bytes_size_hint(b: &mut test::Bencher) {
    b.iter(|| {
        let mut vec: Vec<u64> = Vec::with_capacity(2000);
        INPUT.split(|b| *b == '\n' as u8).for_each(|line| {
            let n = line
                .iter()
                .fold(0 as u64, |n, b| n * 10 + (b - ('0' as u8)) as u64);

            vec.push(n);
        })
    });
}

#[bench]
fn bench_solve_00(b: &mut test::Bencher) {
    let input = parse(INPUT);

    b.iter(|| {
        solve(&input);
    });
}

#[bench]
fn bench_solve_01_windowed(b: &mut test::Bencher) {
    let input = parse(INPUT);

    b.iter(|| {
        solve_windowed(&input);
    });
}

#[bench]
fn bench_solve_02_windowed_sum(b: &mut test::Bencher) {
    let input = parse(INPUT);

    b.iter(|| {
        let simple_increases = input.windows(2).filter(|w| w[1] > w[0]).count();

        let windowed_increases = input
            .windows(3)
            .map(|w| w.iter().sum())
            .collect::<Vec<u64>>()
            .windows(2)
            .filter(|w| w[1] > w[0])
            .count();
        (simple_increases, windowed_increases)
    });
}

#[bench]
fn bench_solve_03_windowed_sum_slice(b: &mut test::Bencher) {
    let input = parse(INPUT);

    b.iter(|| {
        solve_windowed_sum_slice(&input);
    });
}

#[bench]
fn bench_solve_04_windowed_slice(b: &mut test::Bencher) {
    let input = parse(INPUT);

    b.iter(|| {
        solve_windowed_slice(&input);
    });
}
