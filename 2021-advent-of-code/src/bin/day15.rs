#![feature(test)]
#![feature(mixed_integer_ops)]
#![feature(map_first_last)]

extern crate test;

const EXAMPLE: &str = include_str!("example15.txt");
const INPUT: &str = include_str!("input15.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (u32, u32) {
    let grid = parse(input);

    (
        min_cost(&grid),
        min_cost(&expand_grid(&grid, 5))
    )
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

const NEIGHBORS: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn min_cost(grid: &[Vec<u32>]) -> u32 {
    let mut distance: Vec<Vec<u32>> = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
    let mut to_visit: Vec<(u32, (usize, usize))> = Vec::new();

    distance[0][0] = 0;
    to_visit.push((0, (0, 0)));

    while let Some((cum_cost, (c, r))) = pop_min(&mut to_visit) {
        // println!("visiting {:?} at cost: {} from queue {:?}", (c, r), cum_cost, to_visit);

        // for row in &distance {
        //     for cell in row {
        //         print!("{:4} ", cell);
        //     }
        //     println!();
        // }

        // if at bottom right corner
        if c == grid[0].len() - 1 && r == grid.len() - 1 {
            return cum_cost;
        }

        // we already know about a cheaper route to this (c, r)
        if cum_cost > distance[r][c] {
            continue;
        }

        for (x, y) in NEIGHBORS {
            let (r, c) = match (r.checked_add_signed(y), c.checked_add_signed(x)) {
                (Some(r), Some(c)) if r != grid.len() && c != grid[r].len() => (r, c),
                _ => continue, // out of bounds
            };

            if cum_cost + grid[r][c] < distance[r][c] {
                // queue (c, r) because we've found a better path than previously known
                distance[r][c] = cum_cost + grid[r][c];
                to_visit.push((cum_cost + grid[r][c], (c, r)));
            }
        }
    }

    unreachable!("no solution to grid!");
}

// ENHANCEMENT: make generic?
// fn pop_min_generic<T>(vec: &mut Vec<T>, cmp: fn(&T, &T) -> std::cmp::Ordering) -> T { ... }
fn pop_min(to_visit: &mut Vec<(u32, (usize, usize))>) -> Option<(u32, (usize, usize))> {
    if let Some((ind, &min)) = to_visit
        .iter()
        .enumerate()
        .min_by(|(_i, (cost, _)), (_j, (cost_b, _))| cost.cmp(cost_b)) {
        to_visit.swap_remove(ind);

        return Some(min);
    }

    None
}

#[test]
fn test_pop_min() {
    assert_eq!(
        pop_min(&mut vec![(3, (1, 2)), (2, (5, 6)), (1, (9, 8))]),
        Some((1, (9, 8)))
    );
    assert_eq!(
        pop_min(&mut vec![]),
        None
    );
}

fn expand_grid(original: &[Vec<u32>], count: usize) -> Vec<Vec<u32>> {
    (0..original.len() * count)
        .map(|row| {
            let source_row = &original[row % original.len()];
            (0..source_row.len() * count)
                .map(|col| {
                    let source_val = source_row[col % source_row.len()];
                    let val = source_val + (row / original.len()) as u32 + (col / source_row.len()) as u32;
                    if val >= 10 { (val % 10) + 1 } else { val }
                })
                .collect()
        })
        .collect()
}

#[test]
fn test_expand_grid() {
    let expected = parse("11637517422274862853338597396444961841755517295286
13813736722492484783351359589446246169155735727126
21365113283247622439435873354154698446526571955763
36949315694715142671582625378269373648937148475914
74634171118574528222968563933317967414442817852555
13191281372421239248353234135946434524615754563572
13599124212461123532357223464346833457545794456865
31254216394236532741534764385264587549637569865174
12931385212314249632342535174345364628545647573965
23119445813422155692453326671356443778246755488935
22748628533385973964449618417555172952866628316397
24924847833513595894462461691557357271266846838237
32476224394358733541546984465265719557637682166874
47151426715826253782693736489371484759148259586125
85745282229685639333179674144428178525553928963666
24212392483532341359464345246157545635726865674683
24611235323572234643468334575457944568656815567976
42365327415347643852645875496375698651748671976285
23142496323425351743453646285456475739656758684176
34221556924533266713564437782467554889357866599146
33859739644496184175551729528666283163977739427418
35135958944624616915573572712668468382377957949348
43587335415469844652657195576376821668748793277985
58262537826937364893714847591482595861259361697236
96856393331796741444281785255539289636664139174777
35323413594643452461575456357268656746837976785794
35722346434683345754579445686568155679767926678187
53476438526458754963756986517486719762859782187396
34253517434536462854564757396567586841767869795287
45332667135644377824675548893578665991468977611257
44961841755517295286662831639777394274188841538529
46246169155735727126684683823779579493488168151459
54698446526571955763768216687487932779859814388196
69373648937148475914825958612593616972361472718347
17967414442817852555392896366641391747775241285888
46434524615754563572686567468379767857948187896815
46833457545794456865681556797679266781878137789298
64587549637569865174867197628597821873961893298417
45364628545647573965675868417678697952878971816398
56443778246755488935786659914689776112579188722368
55172952866628316397773942741888415385299952649631
57357271266846838237795794934881681514599279262561
65719557637682166874879327798598143881961925499217
71484759148259586125936169723614727183472583829458
28178525553928963666413917477752412858886352396999
57545635726865674683797678579481878968159298917926
57944568656815567976792667818781377892989248891319
75698651748671976285978218739618932984172914319528
56475739656758684176786979528789718163989182927419
67554889357866599146897761125791887223681299833479");

    let actual = expand_grid(&parse(EXAMPLE), 5);
    assert_eq!(actual.len(), expected.len());

    for (row_actual, row_expected) in actual.iter().zip(expected.iter()) {
        assert_eq!(row_actual, row_expected);
    }
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (40, 315));
}

#[bench]
fn bench_min_cost_00_current(b: &mut test::Bencher) {
    let grid = parse(INPUT);
    b.iter(|| {
        assert_eq!(min_cost(&grid), 435);
    });
}

// An attempt to use std::collections::BinaryHeap. Failed because it's a max heap, not a min heap.
//
// #[bench]
// fn bench_min_cost_01_btreemap(b: &mut test::Bencher) {
//     use std::collections::BinaryHeap;
//
//     fn min_cost(grid: &[Vec<u32>]) -> u32 {
//         let mut distance: Vec<Vec<u32>> = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
//         let mut to_visit: BinaryHeap<(u32, (usize, usize))> = BinaryHeap::new();
//
//         distance[0][0] = 0;
//         to_visit.push((0, (0, 0)));
//
//         // dbg!(&to_visit);
//
//         while let Some((cum_cost, (c, r))) = to_visit.pop() {
//             println!("visiting {:?} at cost: {} from queue {:?}", (c, r), cum_cost, to_visit);
//
//             for row in &distance {
//                 for cell in row {
//                     print!("{:4} ", cell);
//                 }
//                 println!();
//             }
//
//             // if at bottom right corner
//             if c == grid[0].len() - 1 && r == grid.len() - 1 {
//                 return cum_cost;
//             }
//
//             // we already know about a cheaper route to this (c, r)
//             if cum_cost > distance[r][c] {
//                 continue;
//             }
//
//             for (x, y) in NEIGHBORS {
//                 let (r, c) = match (r.checked_add_signed(y), c.checked_add_signed(x)) {
//                     (Some(r), Some(c)) if r != grid.len() && c != grid[r].len() => (r, c),
//                     _ => continue, // out of bounds
//                 };
//
//                 if cum_cost + grid[r][c] < distance[r][c] {
//                     // queue (c, r) because we've found a better path than previously known
//                     distance[r][c] = cum_cost + grid[r][c];
//                     to_visit.push((cum_cost + grid[r][c], (c, r)));
//                 }
//             }
//
//             // dbg!(&to_visit);
//         }
//
//         // dbg!(distance[9][9]);
//
//         unreachable!("no solution to grid!");
//     }
//
//     let grid = parse(EXAMPLE);
//     b.iter(|| {
//         assert_eq!(min_cost(&grid), 435);
//     });
// }
