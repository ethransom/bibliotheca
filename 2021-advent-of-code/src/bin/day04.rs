#![feature(test)]

extern crate test;

const EXAMPLE: &[u8] = include_bytes!("example04.txt");
const INPUT: &[u8] = include_bytes!("input04.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

type Board = [[u8; 5]; 5];

#[derive(Clone, Debug)]
struct MemoryBoard {
    board: Board,
    turns: usize, // only used for 'memoized' and following benchmarks
    marked: [[bool; 5]; 5],
}

impl MemoryBoard {
    fn new(board: &Board) -> MemoryBoard {
        MemoryBoard {
            board: *board,
            turns: 0,
            marked: [[false; 5]; 5],
        }
    }
    fn mark(&mut self, draw: u8) {
        self.turns += 1;
        for row in 0..5 {
            for col in 0..5 {
                if self.board[row][col] == draw {
                    self.marked[row][col] = true;
                }
            }
        }
    }
    fn has_won(&self) -> bool {
        'nextrow: for row in self.marked {
            for cell in row {
                if !cell {
                    continue 'nextrow;
                }
            }
            return true;
        }
        'nextcol: for col in 0..5 {
            for row in self.marked {
                if !row[col] {
                    continue 'nextcol;
                }
            }
            return true;
        }

        false
    }
    fn score(&self, drawn: &[u8]) -> usize {
        let mut sum: usize = 0;
        for row in self.board {
            for cell in row {
                if drawn.iter().position(|r| *r == cell).is_none() {
                    sum += cell as usize;
                }
            }
        }
        sum
    }
}

fn parse(input: &[u8]) -> (Vec<u8>, Vec<Board>) {
    let blocks = std::str::from_utf8(input)
        .expect("input was not utf8")
        .split("\n\n")
        .collect::<Vec<&str>>();
    let numbers: Vec<u8> = blocks[0]
        .split(',')
        .map(|n| n.parse::<u8>().expect("not a number"))
        .collect();

    let boards: Vec<Board> = blocks[1..]
        .iter()
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

    (numbers, boards)
}

fn solve(input: &[u8]) -> (usize, usize) {
    let (numbers, raw_boards) = parse(input);
    let boards: Vec<MemoryBoard> = raw_boards
        .iter()
        .map(MemoryBoard::new)
        .collect();

    let mut boards: Vec<MemoryBoard> = boards;
    for draw in numbers.iter() {
        for board in boards.iter_mut() {
            if !board.has_won() {
                board.mark(*draw);
            }
        }
    }

    let max_board = boards
        .iter()
        .max_by(|board_a, board_b| board_a.turns.cmp(&board_b.turns))
        .expect("no wins");

    let turn = max_board.turns;
    let first_score = max_board.score(&numbers[0..turn]) * numbers[turn - 1] as usize;

    let min_board = boards
        .iter()
        .min_by(|board_a, board_b| board_a.turns.cmp(&board_b.turns))
        .expect("no wins");

    let turn = min_board.turns;
    let last_score = min_board.score(&numbers[0..turn]) * numbers[turn - 1] as usize;

    (first_score, last_score)
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (1924, 4512));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (8468, 39984));
    });
}

#[bench]
fn bench_parse_00_current(b: &mut test::Bencher) {
    b.iter(|| parse(INPUT));
}

#[bench]
fn bench_parse_01_next(b: &mut test::Bencher) {
    fn parse(input: &[u8]) -> (Vec<u8>, Vec<Board>) {
        let mut blocks = std::str::from_utf8(input)
            .expect("input was not utf8")
            .split("\n\n");

        let numbers: Vec<u8> = blocks
            .next()
            .expect("need list of called numbers")
            .split(',')
            .map(|n| n.parse::<u8>().expect("not a number"))
            .collect();

        let boards: Vec<Board> = blocks
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

        (numbers, boards)
    }
    b.iter(|| parse(INPUT));
}

#[bench]
fn bench_solve_00_original(b: &mut test::Bencher) {
    // original solution, reproduced here for posterity

    fn solve(input: &[u8]) -> (usize, usize) {
        let (numbers, boards) = parse(input);

        let mut wins_on: Vec<Option<usize>> = vec![None; boards.len()];
        for turn in 1..numbers.len() {
            let drawn = &numbers[0..turn];

            for (board_num, board) in boards.iter().enumerate() {
                if matches!(wins_on[board_num], None) && has_won(board, drawn) {
                    wins_on[board_num] = Some(turn);
                }
            }
        }

        // do we really need this check? puzzle input seems to always satisfy this...
        let wins_on: Vec<usize> = wins_on
            .into_iter()
            .map(|win| {
                if let Some(win) = win {
                    win
                } else {
                    panic!("not all boards won!")
                }
            })
            .collect();

        let (board_no, turn) = wins_on
            .iter()
            .enumerate()
            .max_by(|(_, turn_a), (_, turn_b)| turn_a.cmp(turn_b))
            .expect("no wins");

        let first_score = score(&boards[board_no], &numbers[0..*turn]) * numbers[turn - 1] as usize;

        let (board_no, turn) = wins_on
            .iter()
            .enumerate()
            .min_by(|(_, turn_a), (_, turn_b)| turn_a.cmp(turn_b))
            .expect("no wins");

        let last_score = score(&boards[board_no], &numbers[0..*turn]) * numbers[turn - 1] as usize;

        (first_score, last_score)
    }

    fn has_won(board: &Board, drawn: &[u8]) -> bool {
        'nextrow: for row in board {
            for cell in row {
                if drawn.iter().position(|&r| r == *cell).is_none() {
                    continue 'nextrow;
                }
            }

            return true;
        }

        'nextcol: for col in 0..5 {
            for row in board {
                if drawn.iter().position(|&r| r == row[col]).is_none() {
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
                if drawn.iter().position(|r| *r == *cell).is_none() {
                    sum += *cell as usize;
                }
            }
        }
        sum
    }

    b.iter(|| assert_eq!(solve(INPUT), (8468, 39984)));
}

#[bench]
// Hilariously, this is NOT faster.
// Maybe we're paying too much to sort, maybe real bottleneck is scanning the cells.
fn bench_solve_01_binary(b: &mut test::Bencher) {
    fn score(board: &Board, drawn: &[u8]) -> usize {
        let mut sum: usize = 0;
        for row in board {
            for cell in row {
                if drawn.iter().position(|r| *r == *cell).is_none() {
                    sum += *cell as usize;
                }
            }
        }
        sum
    }

    fn has_won(board: &Board, drawn: &[u8]) -> bool {
        'nextrow: for row in board {
            for cell in row {
                if let Err(_idx) = drawn.binary_search_by(|&r| r.cmp(cell)) {
                    continue 'nextrow;
                }
            }

            return true;
        }

        'nextcol: for col in 0..5 {
            for row in board {
                if let Err(_idx) = drawn.binary_search_by(|&r| r.cmp(&row[col])) {
                    continue 'nextcol;
                }
            }

            return true;
        }

        false
    }

    b.iter(|| {
        let (numbers, boards) = parse(INPUT);

        // `numbers` would be needed for scoring later on
        let mut lookup = numbers.clone();
        let mut wins_on: Vec<Option<usize>> = vec![None; boards.len()];
        for turn in 1..numbers.len() {
            let drawn = &mut lookup[0..turn];
            drawn.sort(); // hopefully this is incremental

            for (board_num, board) in boards.iter().enumerate() {
                if matches!(wins_on[board_num], None) && has_won(board, drawn) {
                    wins_on[board_num] = Some(turn);
                }
            }
        }

        // do we really need this check? puzzle input seems to always satisfy this...
        let wins_on: Vec<usize> = wins_on
            .into_iter()
            .map(|win| {
                if let Some(win) = win {
                    win
                } else {
                    panic!("not all boards won!")
                }
            })
            .collect();

        let (board_no, turn) = wins_on
            .iter()
            .enumerate()
            .max_by(|(_, turn_a), (_, turn_b)| turn_a.cmp(turn_b))
            .expect("no wins");

        let first_score = score(&boards[board_no], &numbers[0..*turn]) * numbers[turn - 1] as usize;

        let (board_no, turn) = wins_on
            .iter()
            .enumerate()
            .min_by(|(_, turn_a), (_, turn_b)| turn_a.cmp(turn_b))
            .expect("no wins");

        let last_score = score(&boards[board_no], &numbers[0..*turn]) * numbers[turn - 1] as usize;

        assert_eq!((first_score, last_score), (8468, 39984))
    });
}

#[bench]
fn bench_solve_02_memory(b: &mut test::Bencher) {
    b.iter(|| {
        let (numbers, raw_boards) = parse(INPUT);
        let boards: Vec<MemoryBoard> = raw_boards
            .iter()
            .map(MemoryBoard::new)
            .collect();
        let mut boards: Vec<MemoryBoard> = boards;
        let mut wins_on: Vec<Option<usize>> = vec![None; boards.len()];
        for (i, draw) in numbers.iter().enumerate() {
            for (board_num, board) in boards.iter_mut().enumerate() {
                if !board.has_won() {
                    board.mark(*draw);
                    if board.has_won() {
                        wins_on[board_num] = Some(i + 1);
                    }
                }
            }
        }

        // do we really need this check? puzzle input seems to always satisfy this...
        let wins_on: Vec<usize> = wins_on
            .into_iter()
            .map(|win| {
                if let Some(win) = win {
                    win
                } else {
                    panic!("not all boards won!")
                }
            })
            .collect();

        let (board_no, turn) = wins_on
            .iter()
            .enumerate()
            .max_by(|(_, turn_a), (_, turn_b)| turn_a.cmp(turn_b))
            .expect("no wins");

        let first_score = boards[board_no].score(&numbers[0..*turn]) * numbers[turn - 1] as usize;

        let (board_no, turn) = wins_on
            .iter()
            .enumerate()
            .min_by(|(_, turn_a), (_, turn_b)| turn_a.cmp(turn_b))
            .expect("no wins");

        let last_score = boards[board_no].score(&numbers[0..*turn]) * numbers[turn - 1] as usize;

        assert_eq!((first_score, last_score), (8468, 39984))
    });
}

#[bench]
fn bench_solve_03_memory_memoize_turns(b: &mut test::Bencher) {
    fn solve(input: &[u8]) -> (usize, usize) {
        let (numbers, raw_boards) = parse(input);
        let boards: Vec<MemoryBoard> = raw_boards
            .iter()
            .map(MemoryBoard::new)
            .collect();

        let mut boards: Vec<MemoryBoard> = boards;
        for draw in numbers.iter() {
            for board in boards.iter_mut() {
                if !board.has_won() {
                    board.mark(*draw);
                }
            }
        }

        let max_board = boards
            .iter()
            .max_by(|board_a, board_b| board_a.turns.cmp(&board_b.turns))
            .expect("no wins");

        let turn = max_board.turns;
        let first_score = max_board.score(&numbers[0..turn]) * numbers[turn - 1] as usize;

        let min_board = boards
            .iter()
            .min_by(|board_a, board_b| board_a.turns.cmp(&board_b.turns))
            .expect("no wins");

        let turn = min_board.turns;
        let last_score = min_board.score(&numbers[0..turn]) * numbers[turn - 1] as usize;

        (first_score, last_score)
    }

    b.iter(|| assert_eq!(solve(INPUT), (8468, 39984)));
}
