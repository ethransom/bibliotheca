#![feature(test)]

extern crate test;

const EXAMPLE: &[u8] = include_bytes!("example05.txt");
const INPUT: &[u8] = include_bytes!("input05.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

type Point = (usize, usize);

fn solve(input: &[u8]) -> (usize, usize) {
    let pairs = std::str::from_utf8(input)
        .expect("input was not utf8")
        .split("\n")
        .map(|line| {
            let mut parts = line.split(" -> ");

            fn parse_point(s: &str) -> Point {
                let mut parts = s.split(",");
                (
                    parts.next().unwrap().parse().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                )
            }

            (
                parse_point(parts.next().unwrap()),
                parse_point(parts.next().unwrap()),
            )
        })
        .collect::<Vec<(Point, Point)>>();

    let points = pairs
        .iter()
        .map(|(from, to)| [from.clone(), to.clone()])
        .flatten()
        .collect::<Vec<Point>>();

    // dbg!(points);

    let (_, height) = points.iter().max_by(|(_, y1), (_, y2)| y1.cmp(y2)).unwrap();
    let (width, _) = points.iter().max_by(|(x1, _), (x2, _)| x1.cmp(x2)).unwrap();

    let mut map = vec![vec![0 as usize; *width as usize + 1]; *height as usize + 1];

    for &pair in &pairs {
        let ((from_x, from_y), (to_x, to_y)) = pair;

        if from_x == to_x {
            let x = from_x;
            let ys = if to_y >= from_y {
                from_y..=to_y
            } else {
                to_y..=from_y
            };
            for y in ys {
                map[y as usize][x] += 1;
            }
            continue;
        }
        if from_y == to_y {
            let y = from_y;
            let xs = if to_x >= from_x {
                from_x..=to_x
            } else {
                to_x..=from_x
            };
            for x in xs {
                map[y][x as usize] += 1;
            }
            continue;
        }
        assert_eq!((from_x == to_x || from_y == to_y), false);
    }

    let mut part1 = 0;
    for row in map.iter() {
        for cell in row.iter() {
            if *cell >= 2 {
                part1 += 1;
            }
        }
    }

    for &pair in &pairs {
        let ((mut x, mut y), (x_end, y_end)) = pair;

        if x == x_end || y == y_end {
            // skip for part 2
            continue;
        }

        loop {
            map[y as usize][x as usize] += 1;

            if x == x_end || y == y_end {
                break;
            }

            if x < x_end {
                x += 1;
            } else {
                x -= 1;
            }
            if y < y_end {
                y += 1;
            } else {
                y -= 1;
            }
        }
    }

    let mut part2 = 0;
    for row in map.iter() {
        for cell in row.iter() {
            if *cell >= 2 {
                part2 += 1;
            }
        }
    }

    (part1, part2)
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (5, 12));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (7380, 21373));
    });
}
