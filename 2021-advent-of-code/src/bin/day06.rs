#![feature(test)]

extern crate test;

const EXAMPLE: &[u8] = include_bytes!("example06.txt");
const INPUT: &[u8] = include_bytes!("input06.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn simulate(initial: &Vec<u8>, rounds: usize) -> usize {
    let mut counts = vec![0 as usize; 9];

    for &fish in initial {
        counts[fish as usize] += 1;
    }

    println!("initial: {:?}", counts);

    for i in 0..rounds {
        let breeders = counts[0];

        // shift down
        for i in 1..9 {
            counts[i - 1] = counts[i];
        }

        counts[6] += breeders;

        counts[8] = breeders;

        println!("After day {}: {}", i + 1, counts.iter().sum::<usize>());
    }

    return counts.iter().sum();
}

fn solve(input: &[u8]) -> (usize, usize) {
    let fishes = std::str::from_utf8(input)
        .expect("input was not utf8")
        .split(",")
        .map(|n| n.parse().expect("not a number"))
        .collect::<Vec<u8>>();

    println!("Initial state: {}", fishes.len());

    (simulate(&fishes, 80), simulate(&fishes, 256))
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (5934, 26_984_457_539));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (354_564, 1_609_058_859_115));
    });
}
