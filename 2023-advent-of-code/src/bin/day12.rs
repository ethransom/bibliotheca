#![feature(test)]

extern crate test;

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

    let sum = sum_possibilities(&rows);

    // lmfaooooo unfold
    let rows: Vec<_> = rows
        .into_iter()
        .map(|(springs, groups)| (springs.repeat(5), groups.repeat(5)))
        .collect();

    for row in &rows {
        println!("{row:?}");
    }

    let sum_unfolded = sum_possibilities(&rows);

    (sum, sum_unfolded)
}

fn sum_possibilities(rows: &Vec<(String, Vec<usize>)>) -> usize {
    let mut sum = 0;

    for (springs, actual_groups) in rows {
        let mut count = 0;
        println!("{springs}: {actual_groups:?}");
        possibilities(springs, actual_groups, &mut count);
        // println!("  -> {count}");
        sum += count;
    }
    sum
}

fn possibilities(springs: &str, actual_groups: &[usize], count: &mut usize) {
    fn possibilities(
        original_springs: &[u8],
        springs: &mut [u8],
        pos: usize,
        actual_groups: &[usize],
        count: &mut usize,
    ) {
        if pos >= original_springs.len() {
            // println!(
            //     "is_empty {springs:?}",
            //     springs = unsafe { std::str::from_utf8_unchecked(springs) }
            // );
            let group = springs.iter().group_by(|&c| c);
            let groups =
                group
                    .into_iter()
                    .filter_map(|(&c, group)| if c == b'#' { Some(group.count()) } else { None });
            if itertools::equal(groups, actual_groups.iter().cloned()) {
                *count += 1;
            }
            return;
        }

        if original_springs[pos] == b'?' {
            springs[pos] = b'#';
            possibilities(original_springs, springs, pos + 1, actual_groups, count);
            springs[pos] = b'.';
            possibilities(original_springs, springs, pos + 1, actual_groups, count);
        } else {
            possibilities(original_springs, springs, pos + 1, actual_groups, count);
        }
    }

    let mut working_copy = springs.as_bytes().to_owned();

    possibilities(
        springs.as_bytes(),
        &mut working_copy,
        0,
        actual_groups,
        count,
    );
}

fn parse(input: &str) -> Vec<(String, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (springs, groups) = line.split_once(' ').unwrap();
            (
                springs.to_string(),
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
    assert_eq!(solve(EXAMPLE), (21, 525152));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (6958, 0));
}

#[bench]
fn bench_solve_example_02_current(b: &mut test::Bencher) {
    // TODO idk could test different allocation strategies here
    b.iter(|| {
        let rows = parse(EXAMPLE);
        assert_eq!(sum_possibilities(&rows), 21);
    });
}

#[bench]
fn bench_solve_example_01_working_buffer(b: &mut test::Bencher) {
    #[allow(dead_code)]
    fn sum_possibilities(rows: &Vec<(String, Vec<usize>)>) -> usize {
        let mut sum = 0;

        for (springs, actual_groups) in rows {
            let mut count = 0;
            // println!("{springs}: {actual_groups:?}");
            possibilities(springs, actual_groups, &mut count);
            // println!("  -> {count}");
            sum += count;
        }
        sum
    }

    #[allow(dead_code)]
    fn possibilities(springs: &str, actual_groups: &[usize], count: &mut usize) {
        fn possibilities(
            original_springs: &str,
            springs: &mut str,
            pos: usize,
            actual_groups: &[usize],
            count: &mut usize,
        ) {
            // println!("possibilities {springs} {pos}");
            if pos >= original_springs.len() {
                // println!("is_empty {springs}");
                if get_groups(springs) == actual_groups {
                    *count += 1;
                }
                return;
            }

            if original_springs.as_bytes()[pos] == b'?' {
                unsafe { springs.as_bytes_mut()[pos] = b'#' };
                possibilities(original_springs, springs, pos + 1, actual_groups, count);
                unsafe { springs.as_bytes_mut()[pos] = b'.' };
                possibilities(original_springs, springs, pos + 1, actual_groups, count);
            } else {
                possibilities(original_springs, springs, pos + 1, actual_groups, count);
            }
        }

        let mut working_copy = springs.to_owned();

        possibilities(springs, &mut working_copy, 0, actual_groups, count);
    }

    #[allow(dead_code)]
    fn get_groups(springs: &str) -> Vec<usize> {
        springs
            .chars()
            .group_by(|&c| c)
            .into_iter()
            .filter_map(|(c, group)| if c == '#' { Some(group.count()) } else { None })
            .collect()
    }

    // TODO idk could test different allocation strategies here
    b.iter(|| {
        let rows = parse(EXAMPLE);
        assert_eq!(sum_possibilities(&rows), 21);
    });
}

#[bench]
fn bench_solve_example_00_original(b: &mut test::Bencher) {
    #[allow(dead_code)]
    fn sum_possibilities(rows: &Vec<(String, Vec<usize>)>) -> usize {
        let mut sum = 0;

        for (springs, actual_groups) in rows {
            let mut count = 0;
            // println!("{springs}:");
            let ps = possibilities(springs);
            for p in &ps {
                let groups = get_groups(p);
                // println!("\t{p}\t{groups:?}\t{actual_groups:?}");
                if &groups == actual_groups {
                    count += 1;
                }
            }
            // println!("  -> {count}");
            sum += count;
        }
        sum
    }

    #[allow(dead_code)]
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
        } else if ps.is_empty() {
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

    #[allow(dead_code)]
    fn get_groups(springs: &str) -> Vec<usize> {
        springs
            .chars()
            .group_by(|&c| c)
            .into_iter()
            .filter_map(|(c, group)| if c == '#' { Some(group.count()) } else { None })
            .collect()
    }

    // TODO idk could test different allocation strategies here
    b.iter(|| {
        let rows = parse(EXAMPLE);
        assert_eq!(sum_possibilities(&rows), 21);
    });
}
