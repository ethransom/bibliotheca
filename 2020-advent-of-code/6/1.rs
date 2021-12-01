#![feature(test)]

extern crate test;

fn main() {
    let input = parse("input.txt");

    println!("{}", solve_extra_fast(&input));
}

fn parse(filename: &str) -> String {
    std::fs::read_to_string(filename).expect("couldn't read file")
}

pub fn solve(input: &String) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut chars: Vec<char> = group.chars().filter(|c| *c != '\n').collect::<Vec<char>>();
            chars.sort();
            chars.dedup();

            chars.len()
        })
        .sum()
}

pub fn solve_fast(input: &String) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut alphabet = [false; 26];
            for c in group.chars() {
                if c != '\n' {
                    let offset = (c as u8) - ('a' as u8);
                    alphabet[offset as usize] = true;
                }
            }

            let mut count = 0;
            for a in alphabet {
                if a {
                    count += 1;
                }
            }

            count
        })
        .sum()
}

pub fn solve_extra_fast(input: &String) -> u32 {
    input
        .split("\n\n")
        .map(|group| {
            let mut alphabet = 0 as u32;
            for c in group.chars() {
                if c != '\n' {
                    let offset = (c as u8) - ('a' as u8);
                    alphabet |= 1 << offset;
                }
            }

            alphabet.count_ones()
        })
        .sum()
}

pub fn solve_farm_to_table(input: &String) -> u32 {
    let mut sum = 0;

    for group in input.split("\n\n") {
        let mut alphabet = 0 as u32;
        for c in group.chars() {
            if c != '\n' {
                let offset = (c as u8) - ('a' as u8);
                alphabet |= 1 << offset;
            }
        }

        sum += alphabet.count_ones()
    }

    sum
}

use test::Bencher;

#[bench]
fn bench_solve(b: &mut Bencher) {
    let input = parse("input.txt");

    b.iter(|| {
        solve(&input);
    });
}

#[bench]
fn bench_solve_fast(b: &mut Bencher) {
    let input = parse("input.txt");

    b.iter(|| {
        solve_fast(&input);
    });
}

#[bench]
fn bench_solve_extra_fast(b: &mut Bencher) {
    let input = parse("input.txt");

    b.iter(|| {
        solve_extra_fast(&input);
    });
}

#[bench]
fn bench_solve_farm_to_table(b: &mut Bencher) {
    let input = parse("input.txt");

    b.iter(|| {
        solve_farm_to_table(&input);
    });
}
