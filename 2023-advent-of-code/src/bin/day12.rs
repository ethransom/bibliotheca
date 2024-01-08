#![feature(test)]

extern crate test;

use fxhash::FxHashMap as HashMap;

const EXAMPLE: &str = include_str!("example12.txt");
const INPUT: &str = include_str!("input12.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let rows = parse(input);

    let sum = sum_possibilities(&rows);

    // lmfaooooo unfold
    let rows: Vec<_> = rows
        .into_iter()
        .map(|(springs, groups)| {
            (
                format!("{springs}?{springs}?{springs}?{springs}?{springs}"),
                groups.repeat(5),
            )
        })
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
        let wildcards = springs.chars().filter(|c| c == &'?').count() as u32;
        let est_possibilities = 2_usize.pow(wildcards);
        println!("{springs}: {actual_groups:?} ({wildcards} wildcards, meaning {est_possibilities} naive possibilities)");

        let mut callcount = 0;
        let mut cachecount = 0;
        let count = possibilities(springs, actual_groups, &mut callcount, &mut cachecount);
        println!("  -> {count} (cost of {callcount} calls and {cachecount} cache entries)");
        sum += count;
    }
    sum
}

#[cfg(debug_assertions)]
fn print_recursing(str: String, depth: usize) {
    // return;
    for _ in 0..depth {
        print!("  ");
    }
    println!("{str}");
}
#[cfg(not(debug_assertions))]
fn print_recursing(_str: String, _depth: usize) {}

fn possibilities(
    springs: &str,
    groups: &[usize],
    callcount: &mut usize,
    cachecount: &mut usize,
) -> usize {
    let mut cache = HashMap::default();
    let p = possibilities(springs.as_bytes(), groups, 0, callcount, &mut cache);
    *cachecount = cache.len();
    return p;

    fn possibilities_memo(
        springs: &[u8],
        groups: &[usize],
        depth: usize,
        callcount: &mut usize,
        cache: &mut HashMap<(Vec<u8>, Vec<usize>), usize>,
    ) -> usize {
        if let Some(&cached) = cache.get(&(springs.to_owned(), groups.to_owned())) {
            print_recursing(
                format!(
                    "found in cache: {springs} {groups:?} -> {cached}",
                    springs = unsafe { std::str::from_utf8_unchecked(springs) }
                ),
                depth,
            );
            cached
        } else {
            let val = possibilities(springs, groups, depth, callcount, cache);
            cache.insert((springs.to_owned(), groups.to_owned()), val);
            val
        }
    }

    fn possibilities(
        springs: &[u8],
        groups: &[usize],
        depth: usize,
        callcount: &mut usize,
        cache: &mut HashMap<(Vec<u8>, Vec<usize>), usize>,
    ) -> usize {
        let depth = depth + 1;
        print_recursing(
            format!(
                "{springs} {groups:?}",
                springs = unsafe { std::str::from_utf8_unchecked(springs) }
            ),
            depth,
        );
        *callcount += 1;
        if *callcount > 100 {
            // panic!();
        }

        if springs.is_empty() {
            if groups.is_empty() {
                print_recursing(
                    "springs and groups empty, combo is possible -> 1".to_string(),
                    depth,
                );
                return 1;
            }
            print_recursing(
                "had unsatisfied groups, combo is not possible -> 1".to_string(),
                depth,
            );
            return 0;
        }

        if springs[0] == b'.' {
            print_recursing("head is dot, skipping".to_string(), depth);
            let ret = possibilities_memo(&springs[1..], groups, depth, callcount, cache);
            print_recursing(format!("-> {ret}"), depth);
            return ret;
        }

        let mut i = 0;
        while i < springs.len() && springs[i] == b'#' {
            i += 1;
        }

        if i < springs.len() && springs[i] == b'?' {
            print_recursing("found '?', recursing".to_string(), depth);
            // NOte do NOT advance slice
            let mut count = 0;
            let mut springs = springs.to_owned();
            springs[i] = b'#';
            count += possibilities_memo(&springs, groups, depth, callcount, cache);
            springs[i] = b'.';
            count += possibilities_memo(&springs, groups, depth, callcount, cache);
            print_recursing(format!("recurse found {count} (both branches)"), depth);
            return count;
        }

        print_recursing(format!("lead of '#' of size {i}"), depth);

        if groups.is_empty() {
            print_recursing(
                "no more groups but still ? or #, combo not possible -> 0".to_string(),
                depth,
            );
            return 0;
        }

        // we have a group of '#' of size i, delimited by either '.' or end of slice
        if i == groups[0] {
            print_recursing(format!("successfully matched {i}, recursing"), depth);
            let ret = possibilities_memo(&springs[i..], &groups[1..], depth, callcount, cache);
            print_recursing(format!("recursed with -> {ret}"), depth);
            return ret;
        }

        print_recursing(
            format!(
                "grouping of '#' of {i} when {group} was needed, combo is not possible",
                group = groups[0]
            ),
            depth,
        );

        0
    }
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
fn test_last_example() {
    let rows = parse("?###???????? 3,2,1");
    assert_eq!(sum_possibilities(&rows), 10);
}

#[test]
fn test_last_unfolded_example() {
    let springs = "???.###";
    let groups = [1, 1, 3];
    let rows = vec![(
        format!("{springs}?{springs}?{springs}?{springs}?{springs}?"),
        groups.repeat(5),
    )];
    assert_eq!(sum_possibilities(&rows), 1);
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (21, 525152));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (6958, 6555315065024));
}

#[bench]
fn bench_sum_possibilities_example_02_current(b: &mut test::Bencher) {
    let rows = parse(EXAMPLE);
    b.iter(|| {
        assert_eq!(sum_possibilities(&rows), 21);
    });
}

#[bench]
fn bench_sum_possibilities_example_01_working_buffer(b: &mut test::Bencher) {
    use itertools::Itertools;

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

    let rows = parse(EXAMPLE);
    b.iter(|| {
        assert_eq!(sum_possibilities(&rows), 21);
    });
}

#[bench]
fn bench_sum_possibilities_example_00_original(b: &mut test::Bencher) {
    use itertools::Itertools;

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

    let rows = parse(EXAMPLE);
    b.iter(|| {
        assert_eq!(sum_possibilities(&rows), 21);
    });
}
