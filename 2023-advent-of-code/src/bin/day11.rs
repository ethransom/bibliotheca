// #![feature(test)]

// extern crate test;

use std::collections::HashSet;

const EXAMPLE: &str = include_str!("example11.txt");
const INPUT: &str = include_str!("input11.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let image = parse(input);

    print(&image);

    let expanded_1 = expand(&image, 2);

    print(&expanded_1);

    let expanded_1_000_000 = expand(&image, 1_000_000);

    (
        lengths_of_paths(&expanded_1),
        lengths_of_paths(&expanded_1_000_000),
    )
}

fn lengths_of_paths(image: &HashSet<(usize, usize)>) -> usize {
    let image = image.iter().collect::<Vec<_>>();

    let mut pairs = vec![];
    let mut image = &image[..];
    while image.len() > 1 {
        let a = &image[0];
        for b in &image[1..] {
            pairs.push((a, b));
        }
        image = &image[1..];
    }

    let mut sum = 0;
    for (a, b) in pairs {
        let manhattan_dist =
            (a.0 as i64 - b.0 as i64).unsigned_abs() + (a.1 as i64 - b.1 as i64).unsigned_abs();

        sum += manhattan_dist as usize;
    }
    sum
}

fn expand(image: &HashSet<(usize, usize)>, expansion: usize) -> HashSet<(usize, usize)> {
    let ((x_min, x_max), (y_min, y_max)) = min_max(&image);

    let y_expansions = (y_min..=y_max)
        .filter(|y| (x_min..=x_max).all(|x| !image.contains(&(x, *y))))
        .collect::<Vec<_>>();
    let x_expansions = (x_min..=x_max)
        .filter(|x| (y_min..=y_max).all(|y| !image.contains(&(*x, y))))
        .collect::<Vec<_>>();

    println!("y_expansions: {:?}", y_expansions);
    println!("x_expansions: {:?}", x_expansions);

    let image = image
        .iter()
        .copied()
        .map(|(x, y)| {
            (
                x + ((expansion - 1) * x_expansions.iter().filter(|&&x2| x >= x2).count()),
                y + ((expansion - 1) * y_expansions.iter().filter(|&&y2| y >= y2).count()),
            )
        })
        .collect::<HashSet<_>>();
    image
}

fn print(image: &HashSet<(usize, usize)>) {
    let ((x_min, x_max), (y_min, y_max)) = min_max(image);

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            print!("{}", if image.contains(&(x, y)) { '#' } else { '.' });
        }
        println!();
    }
}

fn min_max(image: &HashSet<(usize, usize)>) -> ((usize, usize), (usize, usize)) {
    image.iter().fold(
        ((0, 0), (0, 0)),
        |((x_min, x_max), (y_min, y_max)), (x, y)| {
            (
                (x_min.min(*x), x_max.max(*x)),
                (y_min.min(*y), y_max.max(*y)),
            )
        },
    )
}

fn parse(input: &str) -> HashSet<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().flat_map(move |(x, c)| match c {
                '.' => None,
                '#' => Some((x, y)),
                _ => panic!("invalid char: {}", c),
            })
        })
        .collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE).0, 374);
}

#[test]
fn test_more_expansions() {
    let image = parse(EXAMPLE);

    print(&image);

    let expanded_10 = expand(&image, 10);
    print(&expanded_10);
    assert_eq!(lengths_of_paths(&expanded_10), 1030);

    let expanded_100 = expand(&image, 100);
    print(&expanded_100);
    assert_eq!(lengths_of_paths(&expanded_100), 8410);
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (9563821, 827009909817));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
