#![feature(test)]
#![feature(slice_group_by)]

extern crate test;

const EXAMPLE: &str = include_str!("example14.txt");
const INPUT: &str = include_str!("input14.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let (mut polymer, rules) = parse(input);

    for step in 0..10 {
        polymer = apply(polymer, &rules);

        println!(
            "\t-> after step {}: {}",
            step,
            polymer.iter().collect::<String>()
        );
    }

    let mut after_10 = polymer.clone();

    after_10.sort_unstable();

    after_10
        .group_by(|a, b| a == b)
        .for_each(|group| println!("group of {} has {}", group[0], group.len()));

    let after_10_least_common = after_10
        .group_by(|a, b| a == b)
        .map(|group| group.len())
        .min()
        .unwrap();

    let after_10_most_common = after_10
        .group_by(|a, b| a == b)
        .map(|group| group.len())
        .max()
        .unwrap();

    for step in 0..30 {
        polymer = apply(polymer, &rules);

        println!(
            "\t-> after step {}: {}",
            step,
            polymer.iter().collect::<String>()
        );
    }

    let mut after_40 = polymer.clone();

    after_40.sort_unstable();

    after_40
        .group_by(|a, b| a == b)
        .for_each(|group| println!("group of {} has {}", group[0], group.len()));

    let after_40_least_common = after_40
        .group_by(|a, b| a == b)
        .map(|group| group.len())
        .min()
        .unwrap();

    let after_40_most_common = after_40
        .group_by(|a, b| a == b)
        .map(|group| group.len())
        .max()
        .unwrap();

    (
        after_10_most_common - after_10_least_common,
        after_40_most_common - after_40_least_common,
    )
}

fn parse(input: &str) -> (Vec<char>, Vec<([char; 2], char)>) {
    let (polymer, rules) = input.split_once("\n\n").expect("expected two sections");

    let polymer: Vec<char> = polymer.chars().collect();

    let rules = rules
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" -> ").expect("expected ' -> '");

            let mut left = left.chars();

            let mut right = right.chars();

            (
                [left.next().unwrap(), left.next().unwrap()],
                right.next().unwrap(),
            )
        })
        .collect::<Vec<([char; 2], char)>>();

    (polymer, rules)
}

fn apply(polymer: Vec<char>, rules: &[([char; 2], char)]) -> Vec<char> {
    println!("APPLY");
    let mut next_polymer: Vec<char> = vec![];
    next_polymer.push(*polymer.first().unwrap());
    for pairs in polymer.windows(2) {
        for &(rule, element) in rules {
            if pairs == rule {
                // println!("applying rule {:?}", (rule, element));
                next_polymer.push(element);
                next_polymer.push(pairs[1]);
                break;
            }
        }
    }
    next_polymer
}

#[test]
fn test_apply() {
    let (mut polymer, rules) = parse(EXAMPLE);

    assert_eq!(polymer, "NNCB".chars().collect::<Vec<char>>());

    polymer = apply(polymer, &rules);
    assert_eq!(polymer, "NCNBCHB".chars().collect::<Vec<char>>());

    polymer = apply(polymer, &rules);
    assert_eq!(polymer, "NBCCNBBBCBHCB".chars().collect::<Vec<char>>());

    polymer = apply(polymer, &rules);
    assert_eq!(
        polymer,
        "NBBBCNCCNBBNBNBBCHBHHBCHB".chars().collect::<Vec<char>>()
    );

    polymer = apply(polymer, &rules);
    assert_eq!(
        polymer,
        "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
            .chars()
            .collect::<Vec<char>>()
    );

    // This polymer grows quickly. After step 5, it has length 97; After step 10, it has length 3073.

    polymer = apply(polymer, &rules);
    assert_eq!(polymer.len(), 97);

    // After step 10, B occurs 1749 times, C occurs 298 times, H occurs 191 times, and N occurs 865 times
    polymer = apply(polymer, &rules);
    polymer = apply(polymer, &rules);
    polymer = apply(polymer, &rules);
    polymer = apply(polymer, &rules);
    polymer = apply(polymer, &rules);

    assert_eq!(polymer.len(), 3073);

    assert_eq!(polymer.iter().filter(|&c| *c == 'B').count(), 1749);
    assert_eq!(polymer.iter().filter(|&c| *c == 'C').count(), 298);
    assert_eq!(polymer.iter().filter(|&c| *c == 'H').count(), 161); // probably typo in paragraph???
    assert_eq!(polymer.iter().filter(|&c| *c == 'N').count(), 865);

    // ; taking the quantity of the most common element (B, 1749) and subtracting the quantity of the least common element (H, 161) produces 1749 - 161 = 1588.
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (1588, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (0, 0));
    });
}
