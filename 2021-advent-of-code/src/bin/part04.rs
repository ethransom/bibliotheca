#![feature(test)]

extern crate test;

const EXAMPLE: &[u8] = include_bytes!("example04.txt");
const INPUT: &[u8] = include_bytes!("input04.txt");

fn main() {
    dbg!(solve1(INPUT));
    dbg!(solve2(INPUT));
}

type Board = [[u8; 5]; 5];

fn solve1(input: &[u8]) -> usize {
    let blocks = std::str::from_utf8(input)
        .expect("input was not utf8")
        .split("\n\n")
        .collect::<Vec<&str>>();

    let numbers: Vec<u8> = blocks[0]
        .split(",")
        .map(|n| n.parse::<u8>().expect("not a number"))
        .collect();

    let boards: Vec<Board> = blocks[1..]
        .into_iter()
        .map(|block| {
            let mut rows: Board = [[0; 5]; 5];

            for (row, line) in block.lines().enumerate() {
                for (cell, s) in line.split_whitespace().enumerate() {
                    let n = s.parse::<u8>().expect("not a number");

                    rows[row][cell] = n;
                }
            }

            rows
        })
        .collect();

    for turn in 1..numbers.len() {
        let drawn = &numbers[0..turn];

        for board in boards.iter() {
            if has_won(board, drawn) {
                let score = score(board, drawn);

                let winning_draw = *drawn.last().unwrap();

                return score * winning_draw as usize;
            }
        }
    }

    unreachable!("no solutions!");
}

fn solve2(input: &[u8]) -> usize {
    let blocks = std::str::from_utf8(input)
        .expect("input was not utf8")
        .split("\n\n")
        .collect::<Vec<&str>>();

    let numbers: Vec<u8> = blocks[0]
        .split(",")
        .map(|n| n.parse::<u8>().expect("not a number"))
        .collect();

    let boards: Vec<Board> = blocks[1..]
        .into_iter()
        .map(|block| {
            let mut rows: Board = [[0; 5]; 5];

            for (row, line) in block.split("\n").enumerate() {
                let mut cell = 0;
                for s in line.split(" ") {
                    if s == "" {
                        continue;
                    }

                    let n = s.parse::<u8>().expect("not a number");

                    rows[row][cell] = n;
                    cell += 1;
                }
            }

            rows
        })
        .collect();

    let mut wins_on: Vec<Option<usize>> = vec![None; boards.len()];
    for turn in 1..numbers.len() {
        let drawn = &numbers[0..turn];

        for (board_num, board) in boards.iter().enumerate() {
            if matches!(wins_on[board_num], None) && has_won(board, drawn) {
                wins_on[board_num] = Some(turn);
            }
        }
    }

    let mut losingest_turn = 0;
    let mut losingest_board: Option<&Board> = None;
    for (board_no, winning_turn) in wins_on.iter().enumerate() {
        let turn = winning_turn.expect("board didn't win!");
        if turn > losingest_turn {
            losingest_turn = turn;
            losingest_board = Some(&boards[board_no]);
        }
    }

    let score = score(losingest_board.unwrap(), &numbers[0..losingest_turn]);
    let last_draw = numbers[losingest_turn - 1];

    return score * last_draw as usize;
}

fn has_won(board: &Board, drawn: &[u8]) -> bool {
    'nextrow: for row in board {
        for cell in row {
            if let None = drawn.iter().position(|&r| r == *cell) {
                continue 'nextrow;
            }
        }

        return true;
    }

    'nextcol: for col in 0..5 {
        for row in board {
            if let None = drawn.iter().position(|&r| r == row[col]) {
                continue 'nextcol;
            }
        }

        return true;
    }

    false
}

fn score(board: &Board, drawn: &[u8]) -> usize {
    let mut sum: usize = 0;
    for row in board {
        for cell in row {
            if let None = drawn.iter().position(|r| *r == *cell) {
                sum += *cell as usize;
            }
        }
    }
    sum
}

#[test]
fn test_part_1() {
    assert_eq!(solve1(EXAMPLE), 4512);
}

#[test]
fn test_part_2() {
    assert_eq!(solve2(EXAMPLE), 1924);
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve1(INPUT), 39984);
        assert_eq!(solve2(INPUT), 8468);
    });
}
