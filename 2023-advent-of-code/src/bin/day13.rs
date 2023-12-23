#![feature(let_chains)]
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

    for (i, image) in images.iter().enumerate() {
        println!("image {i}");

        let height = image.len();
        let width = image[0].len();

        // println!("trying vertically");
        let (vertical, smudges) = vertical_axis_reflections(image, height, width);
        if let Some(c) = vertical {
            reflection_sum += c;
            let vertical_reflections: Vec<_> = smudges
                .iter()
                .enumerate()
                .filter(|(_c, smudges)| smudges.len() == 0)
                .collect();
            assert_eq!(vertical_reflections.len(), 1);
            assert_eq!(vertical_reflections[0].0, c - 1);
            let (horizontal, row_smudges) = horizontal_axis_reflections(image, height, width);
            let almost_reflections: Vec<_> = row_smudges
                .iter()
                .enumerate()
                .map(|(r, s)| ((r + 1) * 100, s))
                .chain(smudges.iter().enumerate().map(|(c, s)| (c + 1, s)))
                .filter(|(_, smudges)| smudges.len() == 1)
                .collect();
            println!(
                "image {i} flipped vertically, and horizontal smudges: {almost_reflections:?}"
            );
            assert_eq!(
                almost_reflections.len(),
                1,
                "did not have almost-reflection"
            );
            smudge_sum += almost_reflections[0].0;

            continue;
        }

        // println!("\ntrying horizontally");

        let (horizontal, smudges) = horizontal_axis_reflections(image, height, width);
        if let Some(r) = horizontal {
            reflection_sum += r * 100;
            let vertical_reflections: Vec<_> = smudges
                .iter()
                .enumerate()
                .filter(|(_r, smudges)| smudges.len() == 0)
                .collect();
            assert_eq!(vertical_reflections.len(), 1);
            assert_eq!(vertical_reflections[0].0, r - 1);
            let (vertical, column_smudges) = vertical_axis_reflections(image, height, width);
            let almost_reflections: Vec<_> = column_smudges
                .iter()
                .enumerate()
                .map(|(c, s)| (c + 1, s))
                .chain(smudges.iter().enumerate().map(|(r, s)| ((r + 1) * 100, s)))
                .filter(|(_, smudges)| smudges.len() == 1)
                .collect();
            println!("image {i} flipped horizontally, and almost smudges: {almost_reflections:?}");
            assert_eq!(
                almost_reflections.len(),
                1,
                "did not have almost-reflection"
            );
            smudge_sum += almost_reflections[0].0;

            continue;
        }
    }

    (reflection_sum, smudge_sum)
}

fn horizontal_axis_reflections(
    image: &[&str],
    height: usize,
    width: usize,
) -> (Option<usize>, Vec<Vec<(usize, usize)>>) {
    let mut horizontal_reflection = None;
    let mut row_mismatches = vec![];

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

        if mismatches.is_empty() {
            if horizontal_reflection.is_some() {
                panic!("multiple horizontal reflections");
            }
            horizontal_reflection = Some(r);
        }

        row_mismatches.push(mismatches);
    }

    (horizontal_reflection, row_mismatches)
}

fn vertical_axis_reflections(
    image: &[&str],
    height: usize,
    width: usize,
) -> (Option<usize>, Vec<Vec<(usize, usize)>>) {
    let mut vertical_reflection = None;
    let mut column_mismatches = vec![];

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

        if mismatches.is_empty() {
            if vertical_reflection.is_some() {
                panic!("multiple vertical reflections");
            }
            vertical_reflection = Some(c);
        }

        column_mismatches.push(mismatches);
    }

    (vertical_reflection, column_mismatches)
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
    assert_eq!(solve(INPUT), (35232, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
