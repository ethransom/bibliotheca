#![feature(test)]

extern crate test;

const EXAMPLE: &str = include_str!("example19.txt");
const INPUT: &str = include_str!("input19.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let blocks: Vec<Vec<(i32, i32, i32)>> = input
        .split("\n\n")
        .map(|scanner| {
            let mut lines = scanner.lines();

            lines.next().expect("expected new scanner block");

            lines
                .map(|lines| {
                    let mut coords = lines.split(',');

                    (
                        coords
                            .next()
                            .expect("expected x coord")
                            .parse::<i32>()
                            .expect("x coord not a number"),
                        coords
                            .next()
                            .expect("expected y coord")
                            .parse::<i32>()
                            .expect("y coord not a number"),
                        if let Some(z) = coords.next() {
                            z.parse::<i32>().expect("z coord not a number")
                        } else {
                            0
                        },
                    )
                })
                .collect()
        })
        .collect();

    for (i, block) in blocks.iter().enumerate() {
        println!("probe {} sees {} beacons", i, block.len());
    }

    (0, 0)
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (79, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (0, 0));
    });
}
