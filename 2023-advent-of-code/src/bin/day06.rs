// #![feature(test)]

// extern crate test;

const EXAMPLE: &str = include_str!("example06.txt");
const INPUT: &str = include_str!("input06.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let races = parse(input);

    let ways_to_win_product = races
        .iter()
        .map(|(time, distance)| {
            let mut count = 0;
            for t in 0..=*time {
                let alt_distance = (time - t) * t;
                if alt_distance > *distance {
                    count += 1;
                }
            }
            count
        })
        .product();

    (ways_to_win_product, 0)
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    let (time, distance) = input.split_once('\n').unwrap();

    fn parse_list<'a>(list: &'a str, prefix: &str) -> impl Iterator<Item = usize> + 'a {
        list.trim_start_matches(prefix)
            .split_whitespace()
            .map(|s| s.parse().unwrap())
    }

    let time = parse_list(time, "Time: ");
    let distance = parse_list(distance, "Distance: ");

    time.zip(distance).collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (288, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (293046, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
