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

    dbg!(&races);

    let ways_to_win_product = races
        .iter()
        .map(|(time, distance)| ways_to_win(time, distance))
        .product();

    let races = races
        .into_iter()
        .reduce(|(time, distance), (t, d)| {
            (
                time * 10usize.pow(t.to_string().len() as u32) + t,
                distance * 10usize.pow(d.to_string().len() as u32) + d,
            )
        })
        .unwrap();

    dbg!(&races);

    let full_ways_to_win = ways_to_win(&races.0, &races.1);

    (ways_to_win_product, full_ways_to_win)
}

fn ways_to_win(time: &usize, distance: &usize) -> usize {
    let mut count = 0;
    for t in 0..=*time {
        let alt_distance = (time - t) * t;
        if alt_distance > *distance {
            count += 1;
        }
    }
    count
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
    assert_eq!(solve(EXAMPLE), (288, 71503));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (293046, 35150181));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
