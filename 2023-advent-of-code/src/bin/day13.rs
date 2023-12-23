// #![feature(test)]

// extern crate test;

const EXAMPLE: &str = include_str!("example13.txt");
const INPUT: &str = include_str!("input13.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let images = parse(input);

    let mut reflection_sum = 0;
    let mut smudge_sum = 0;

    for image in &images {
        let height = image.len();
        let width = image[0].len();

        let vertical_reflections = vertical_axis_reflections(image, height, width);
        let vertical_reflections = vertical_reflections
            .iter()
            .enumerate()
            .map(|(c, s)| (c + 1, s));

        let horizontal_reflections = horizontal_axis_reflections(image, height, width);
        let horizontal_reflections = horizontal_reflections
            .iter()
            .enumerate()
            .map(|(r, s)| ((r + 1) * 100, s));

        let [(perfect_reflection, _empty)] = vertical_reflections
            .clone()
            .chain(horizontal_reflections.clone())
            .filter(|(_c, smudges)| smudges.is_empty())
            .collect::<Vec<_>>()
            .try_into()
            .expect("image did not have exactly one perfect reflection");
        reflection_sum += perfect_reflection;

        let [(almost_reflection, _smudge)] = horizontal_reflections
            .chain(vertical_reflections)
            .filter(|(_, smudges)| smudges.len() == 1)
            .collect::<Vec<_>>()
            .try_into()
            .expect("image did not have exactly one single-smudge reflection");
        smudge_sum += almost_reflection;
    }

    (reflection_sum, smudge_sum)
}

fn horizontal_axis_reflections(
    image: &[&str],
    height: usize,
    width: usize,
) -> Vec<Vec<(usize, usize)>> {
    let mut row_mismatches = vec![];

    // TODO could clean up code duplication here
    for r in 1..height {
        let mut mismatches = vec![];
        for c in 0..width {
            let top = (0..r).map(|r| (c, r));
            let bottom = (r..height).map(|r| (c, r));

            let pairs = top.rev().zip(bottom);
            for ((x1, y1), (x2, y2)) in pairs {
                if image[y1].as_bytes()[x1] != image[y2].as_bytes()[x2] {
                    mismatches.push((x1, y1)); // don't think it matters if we choose 1 or 2
                }
            }
        }

        row_mismatches.push(mismatches);
    }

    row_mismatches
}

fn vertical_axis_reflections(
    image: &[&str],
    height: usize,
    width: usize,
) -> Vec<Vec<(usize, usize)>> {
    let mut column_mismatches = vec![];

    // TODO could clean up code duplication here
    for c in 1..width {
        let mut mismatches = vec![];
        for r in 0..height {
            let left = (0..c).map(|c| (c, r));
            let right = (c..width).map(|c| (c, r));

            let pairs = left.rev().zip(right);
            for ((x1, y1), (x2, y2)) in pairs {
                if image[y1].as_bytes()[x1] != image[y2].as_bytes()[x2] {
                    mismatches.push((x1, y1)); // don't think it matters if we choose 1 or 2
                }
            }
        }

        column_mismatches.push(mismatches);
    }

    column_mismatches
}

fn parse(input: &str) -> Vec<Vec<&str>> {
    input
        .split("\n\n")
        .map(|image| image.lines().collect())
        .collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (405, 400));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (35232, 37982));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
