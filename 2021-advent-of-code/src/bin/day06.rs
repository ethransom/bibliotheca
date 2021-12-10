#![feature(test)]

extern crate test;

const EXAMPLE: &str = include_str!("example06.txt");
const INPUT: &str = include_str!("input06.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn simulate(initial: &[u8], rounds: usize) -> usize {
    let mut counts = vec![0; 9];

    for &fish in initial {
        counts[fish as usize] += 1;
    }

    for _round in 0..rounds {
        let breeders = counts[0];

        // shift down
        for i in 1..9 {
            counts[i - 1] = counts[i];
        }

        counts[6] += breeders;

        counts[8] = breeders;
    }

    return counts.iter().sum();
}

fn solve(input: &str) -> (usize, usize) {
    let fishes = input
        .split(',')
        .map(|n| n.parse().expect("not a number"))
        .collect::<Vec<u8>>();

    (simulate(&fishes, 80), simulate(&fishes, 256))
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (5_934, 26_984_457_539));
}

#[bench]
fn bench_solve_00_original(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (354_564, 1_609_058_859_115));
    });
}

#[bench]
fn bench_solve_01_shift(b: &mut test::Bencher) {
    fn simulate(initial: &Vec<u8>, rounds: usize) -> usize {
        let mut counts = vec![0; 9];

        for &fish in initial {
            counts[fish as usize] += 1;
        }

        for _round in 0..rounds {
            let breeders = counts[0];

            counts.rotate_left(1);

            counts[6] += breeders;

            counts[8] = breeders;
        }

        return counts.iter().sum();
    }

    fn solve(input: &str) -> (usize, usize) {
        let fishes = input
            .split(',')
            .map(|n| n.parse().expect("not a number"))
            .collect::<Vec<u8>>();

        (simulate(&fishes, 80), simulate(&fishes, 256))
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (354_564, 1_609_058_859_115));
    });
}

#[bench]
fn bench_solve_02_stack_alloc(b: &mut test::Bencher) {
    fn simulate(initial: &Vec<u8>, rounds: usize) -> usize {
        let mut counts = vec![0; 9];

        for &fish in initial {
            counts[fish as usize] += 1;
        }

        for _round in 0..rounds {
            let breeders = counts[0];

            // shift down
            for i in 1..9 {
                counts[i - 1] = counts[i];
            }
            // don't use this instead, as it seems to prevent moving
            // the array elements to registers
            // counts.rotate_left(1);

            counts[6] += breeders;
            counts[8] = breeders;
        }

        return counts.iter().sum();
    }

    fn solve(input: &str) -> (usize, usize) {
        let fishes = input
            .split(',')
            .map(|n| n.parse().expect("not a number"))
            .collect::<Vec<u8>>();

        (simulate(&fishes, 80), simulate(&fishes, 256))
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (354_564, 1_609_058_859_115));
    });
}

#[bench]
fn bench_solve_03_memoize(b: &mut test::Bencher) {
    fn simulate(initial: &Vec<u8>, results: &mut [usize]) {
        let mut counts = vec![0; 9];

        for &fish in initial {
            counts[fish as usize] += 1;
        }

        for round in 0..results.len() {
            let breeders = counts[0];
            // shift down
            for i in 1..9 {
                counts[i - 1] = counts[i];
            }
            counts[6] += breeders;
            counts[8] = breeders;

            results[round] = counts.iter().sum();
        }
    }

    fn solve(input: &str) -> (usize, usize) {
        let fishes = input
            .split(',')
            .map(|n| n.parse().expect("not a number"))
            .collect::<Vec<u8>>();

        let mut results = [0 as usize; 256];

        simulate(&fishes, &mut results);

        (results[80 - 1], results[256 - 1])
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (354_564, 1_609_058_859_115));
    });
}

#[bench]
fn bench_solve_04_memoize_no_alloc(b: &mut test::Bencher) {
    fn simulate(mut counts: [usize; 9], results: &mut [usize]) {
        for round in 0..results.len() {
            let breeders = counts[0];
            // shift down
            for i in 1..9 {
                counts[i - 1] = counts[i];
            }
            counts[6] += breeders;
            counts[8] = breeders;

            results[round] = counts.iter().sum();
        }
    }

    fn solve(input: &str) -> (usize, usize) {
        let mut initial = [0 as usize; 9];
        input
            .split(',')
            .for_each(|n| initial[n.parse::<usize>().expect("not a number")] += 1);

        let mut results = [0 as usize; 256];

        simulate(initial, &mut results);

        (results[80 - 1], results[256 - 1])
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (354_564, 1_609_058_859_115));
    });
}

// TODO: could we leave uints in the count vec in place and rotate
//       some pointer or index around the vec? something like:
// ```
// fn simulate(mut counts: [usize; 6], results: &mut [usize]) {
//     let mut young = [0 as usize; 2];

//     for round in 0..results.len() {
//         let breeders = counts[round % 6];

//         counts[round % 6] += young[round % 2];

//         young[round % 2] = breeders;

//         results[round] = counts.iter().sum::<usize>() + young[0] + young[1];
//     }
// }
// ```
