// #![feature(test)]

// extern crate test;

use itertools::Itertools;

const EXAMPLE: &str = include_str!("example12.txt");
const INPUT: &str = include_str!("input12.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let rows = parse(input);

    // for (springs, groups) in &rows {
    //     print!("{}", springs);
    //     get_groups(springs).iter().for_each(|l| print!("{l}"));
    //     println!("{groups:?}");
    // }

    let mut sum = 0;

    for (springs, actual_groups) in &rows {
        let mut count = 0;
        println!("{springs}:");
        let ps = possibilities(springs);
        for p in &ps {
            let groups = get_groups(p);
            // println!("\t{p}\t{groups:?}\t{actual_groups:?}");
            if &groups == actual_groups {
                count += 1;
            }
        }
        println!("  -> {count}");
        sum += count;
    }

    (sum, 0)
}

fn possibilities(springs: &str) -> Vec<String> {
    if springs.is_empty() {
        return vec![];
    }
    let (c, springs) = (&springs[..1], &springs[1..]);
    // println!("\t{c:?} {springs:?}");
    let mut ps = possibilities(springs);
    if c == "?" {
        ["#", "."]
            .iter()
            .flat_map(|c| {
                let mut ps = ps.clone();
                if ps.is_empty() {
                    ps.push(c.to_string());
                    ps
                } else {
                    ps.into_iter()
                        .map(|mut p| {
                            p.insert_str(0, c);
                            p
                        })
                        .collect()
                }
            })
            .collect()
    } else {
        // #[warn(clippy::collapsible_else_if)]
        if ps.is_empty() {
            ps.push(c.to_string());
            ps
        } else {
            ps.into_iter()
                // .inspect(|p| println!("\t-> {p}"))
                .map(|mut p| {
                    p.insert_str(0, c);
                    p
                })
                .collect()
        }
    }
}

fn get_groups(springs: &str) -> Vec<usize> {
    springs
        .chars()
        .group_by(|&c| c)
        .into_iter()
        .filter_map(|(c, group)| if c == '#' { Some(group.count()) } else { None })
        .collect()
}

fn parse(input: &str) -> Vec<(&str, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (springs, groups) = line.split_once(' ').unwrap();
            (
                springs,
                groups
                    .split(',')
                    .map(str::parse)
                    .collect::<Result<_, _>>()
                    .unwrap(),
            )
        })
        .collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (21, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (6958, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
