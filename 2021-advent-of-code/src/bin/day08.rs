#![feature(test)]

extern crate test;

const EXAMPLE: &str = include_str!("example08.txt");
const INPUT: &str = include_str!("input08.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let part1 = input
        .lines()
        .map(|line| {
            let (_patterns, output) = line.split_once('|').expect("expected '|'");

            let output = output.trim_start().split(' ');

            output
                .filter(|pattern| matches!(pattern.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum();

    let part2 = input
        .lines()
        .map(|line| {
            let (patterns, output) = line.split_once('|').expect("expected '|'");

            let patterns = patterns.trim_end().split(' ').collect::<Vec<&str>>();
            let outputs = output.trim_start().split(' ').collect::<Vec<&str>>();

            decode_line(&patterns, &outputs)
        })
        .sum();

    (part1, part2)
}

fn pattern_to_bits(pattern: &str) -> u8 {
    let mut bits = 0;
    for c in pattern.chars() {
        bits = match c {
            'a' => bits | 0b10000000,
            'b' => bits | 0b01000000,
            'c' => bits | 0b00100000,
            'd' => bits | 0b00010000,
            'e' => bits | 0b00001000,
            'f' => bits | 0b00000100,
            'g' => bits | 0b00000010,
            c => panic!("unknown bit! {}", c),
        }
    }
    bits
}

fn bits_to_pattern(bits: u8) -> String {
    let mut s = String::with_capacity(8);

    if bits & 0b10000000 > 0 {
        s.push('a');
    }
    if bits & 0b01000000 > 0 {
        s.push('b');
    }
    if bits & 0b00100000 > 0 {
        s.push('c');
    }
    if bits & 0b00010000 > 0 {
        s.push('d');
    }
    if bits & 0b00001000 > 0 {
        s.push('e');
    }
    if bits & 0b00000100 > 0 {
        s.push('f');
    }
    if bits & 0b00000010 > 0 {
        s.push('g');
    }

    s
}

fn decode_line(patterns: &[&str], outputs: &[&str]) -> usize {
    let patterns_strs: [&str; 10] = patterns.try_into().unwrap();
    let outputs_strs: [&str; 4] = outputs.try_into().unwrap();

    let mut patterns: [u8; 10] = [0; 10];
    for i in 0..patterns_strs.len() {
        patterns[i] = pattern_to_bits(patterns_strs[i]);
    }
    let mut outputs: [u8; 4] = [0; 4];
    for i in 0..outputs_strs.len() {
        outputs[i] = pattern_to_bits(outputs_strs[i]);
    }

    // BEGIN EASY NUMBERS

    let &one = patterns
        .iter()
        .find(|pattern| pattern.count_ones() == 2)
        .expect("no 1 in patterns");

    let &seven = patterns
        .iter()
        .find(|pattern| pattern.count_ones() == 3)
        .expect("no 7 in patterns");

    let &four = patterns
        .iter()
        .find(|pattern| pattern.count_ones() == 4)
        .expect("no 4 in patterns");

    let &eight = patterns
        .iter()
        .find(|pattern| pattern.count_ones() == 7)
        .expect("no 8 in patterns");

    // BEGIN HARD NUMBERS

    // bottom and bottom left are what's in 8 that aren't in 4 ^ 7
    let bottom_and_bottom_left = (four | seven) ^ eight;

    let &three = patterns
        .iter()
        .find(|&pattern| pattern.count_ones() == 5 && (pattern & one).count_ones() == 2)
        .expect("no 3 in pattern");

    let nine = three | four;

    let two = patterns
        .into_iter()
        .find(|&pattern| {
            pattern.count_ones() == 5
                && pattern != three
                && (pattern & bottom_and_bottom_left).count_ones() == 2
        })
        .expect("no 2 in pattern");

    let five = patterns
        .into_iter()
        .find(|&pattern| pattern.count_ones() == 5 && pattern != three && pattern != two)
        .expect("no 5 in pattern");

    let zero = patterns
        .into_iter()
        .find(|&pattern| {
            pattern.count_ones() == 6 && pattern != nine && (pattern & one).count_ones() == 2
        })
        .expect("no 0 in pattern");

    let six = patterns
        .into_iter()
        .find(|&pattern| pattern.count_ones() == 6 && pattern != nine && pattern != zero)
        .expect("no 6 in pattern");

    let remainder = patterns
        .into_iter()
        .filter(|&p| {
            p != zero
                && p != one
                && p != two
                && p != three
                && p != four
                && p != five
                && p != six
                && p != seven
                && p != eight
                && p != nine
        })
        .count();

    assert_eq!(remainder, 0);

    let key = [zero, one, two, three, four, five, six, seven, eight, nine];

    outputs.into_iter().fold(0_usize, |mut n, output| {
        n *= 10;
        n += match key.into_iter().position(|k| k == output) {
            None => panic!("can't decode {:b} {}", output, bits_to_pattern(output)),
            Some(number) => number as usize,
        };
        n
    })
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (26, 61229));
}

// baby's first unit test 🥺
#[test]
fn test_decode_line() {
    assert_eq!(
        decode_line(
            &[
                "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb",
                "ab"
            ],
            &["cdfeb", "fcadb", "cdfeb", "cdbaf"]
        ),
        5353
    );
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (330, 1010472));
    });
}
