#![feature(test)]
#![feature(array_windows)]

const EXAMPLE: &[u8] = include_bytes!("example01.txt");
const INPUT: &[u8] = include_bytes!("input01.txt");

fn main() {
    println!("example:");
    println!("\t{:?}", solve(EXAMPLE));
    println!("input:");
    println!("\t{:?}", solve(INPUT));
}

fn parse(dat: &[u8]) -> Vec<u64> {
    dat.split(|b| *b == b'\n')
        .map(|line| line.iter().fold(0_u64, |n, b| n * 10 + (b - b'0') as u64))
        .collect::<Vec<u64>>()
}

#[inline(always)]
fn solve(input: &[u8]) -> (usize, usize) {
    let nums = parse(input);
    let simple_increases = nums.windows(2).filter(|w| w[1] > w[0]).count();

    let windowed_increases = nums
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<u64>>()
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count();

    (simple_increases, windowed_increases)
}

#[test]
fn it_handles_the_example_input() {
    assert_eq!(solve(EXAMPLE), (7, 5));
}

extern crate test;

#[bench]
fn bench_current(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(solve(INPUT), (1387, 1362)));
}

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
            .split(|b| *b == b'\n')
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
            .split(|b| *b == b'\n')
            .flat_map(|line| unsafe { std::str::from_utf8_unchecked(line) }.parse::<u32>())
            .collect::<Vec<u32>>()
    });
}

#[bench]
fn bench_parse_04_bytes_unsafe(b: &mut test::Bencher) {
    b.iter(|| {
        INPUT
            .split(|b| *b == b'\n')
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
            .split(|b| *b == b'\n')
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
            .map(|line| line.iter().fold(0_u64, |n, b| n * 10 + (b - b'0') as u64))
            .collect::<Vec<u64>>()
    });
}

#[bench]
fn bench_parse_07_bytes_custom_u16(b: &mut test::Bencher) {
    b.iter(|| {
        INPUT
            .split(|b| *b == '\n' as u8)
            .map(|line| line.iter().fold(0_u16, |n, b| n * 10 + (b - b'0') as u16))
            .collect::<Vec<u16>>()
    });
}

#[bench]
fn bench_parse_08_bytes_size_hint(b: &mut test::Bencher) {
    b.iter(|| {
        let mut vec: Vec<u64> = Vec::with_capacity(2000);
        INPUT.split(|b| *b == '\n' as u8).for_each(|line| {
            let n = line.iter().fold(0_u64, |n, b| n * 10 + (b - b'0') as u64);

            vec.push(n);
        })
    });
}

#[bench]
fn bench_solve_00_original(b: &mut test::Bencher) {
    let nums = parse(INPUT);

    b.iter(|| {
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
    });
}

#[bench]
fn bench_solve_01_windowed(b: &mut test::Bencher) {
    let nums = parse(INPUT);

    b.iter(|| {
        let simple_increases = nums.windows(2).filter(|w| w[1] > w[0]).count();

        let windowed_increases = nums
            .windows(3)
            .map(|w| w[0] + w[1] + w[2])
            .collect::<Vec<u64>>()
            .windows(2)
            .filter(|w| w[1] > w[0])
            .count();

        (simple_increases, windowed_increases)
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
fn bench_solve_03_array_windows(b: &mut test::Bencher) {
    let input = parse(INPUT);

    b.iter(|| {
        let simple_increases = input.array_windows::<2>().filter(|[l, r]| r > l).count();

        let windowed_increases = input
            .array_windows::<3>()
            .map(|[a, b, c]| a + b + c)
            .collect::<Vec<u64>>()
            .array_windows::<2>()
            .filter(|[l, r]| r > l)
            .count();
        (simple_increases, windowed_increases)
    });
}

#[bench]
fn bench_solve_04_incredibly_fast(b: &mut test::Bencher) {
    let nums = parse(INPUT);

    b.iter(|| {
        let mut simple_increases = 0;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                simple_increases += 1;
            }
        }
        let windowed_increases = nums
            .windows(3)
            .map(|w| w[0] + w[1] + w[2])
            .collect::<Vec<u64>>()
            .windows(2)
            .filter(|w| w[1] > w[0])
            .count();

        (simple_increases, windowed_increases)
    });
}
