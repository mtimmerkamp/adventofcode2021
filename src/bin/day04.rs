use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type BingoNumber = u8;
const BINGO_ROWS: usize = 5;

#[derive(Debug)]
struct Board {
    board: Vec<Vec<BingoNumber>>,
}

#[derive(Debug)]
enum BoardError {InvalidDimensions}

impl Board {
    fn new(rows: Vec<Vec<BingoNumber>>) -> Result<Board, BoardError> {
        if rows.len() != BINGO_ROWS {
            return Err(BoardError::InvalidDimensions);
        }
        for row in &rows {
            if row.len() != BINGO_ROWS {
                return Err(BoardError::InvalidDimensions);
            }
        }

        Ok(Board {
            board: rows,
        })
    }
}

fn board_wins(board: &Board, numbers: &HashSet<BingoNumber>) -> bool {
    let has_full_row = board.board.iter().any(|row| {
        row.iter().all(|n| numbers.contains(n))
    });

    let has_full_column = (0..BINGO_ROWS).any(|column| {
        board.board.iter().all(|row| numbers.contains(&row[column]))
    });

    has_full_row || has_full_column
}

fn calc_score(board: &Board, marked: &HashSet<BingoNumber>) -> u32 {
    board.board.iter().
        map(|row| {
            row.iter().filter(|n| !marked.contains(n)).map(|n| *n as u32).sum::<u32>()
        })
        .sum()
}

fn read_input(filename: &str) -> (Vec<BingoNumber>, Vec<Board>) {
    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let numbers: Vec<BingoNumber>;
    let mut boards: Vec<Board> = Vec::new();

    if let Some(Ok(line)) = lines.next() {
        numbers = line
            .split(',')
            .map(|s| s.parse().expect(&format!("Cannot parse {}", s)))
            .collect();
    } else {
        panic!("Missing first line!");
    }
    lines
        .next()
        .expect("Missing line before first boards")
        .unwrap();

    let mut rows: Vec<Vec<BingoNumber>> = Vec::new();
    for line in lines {
        if let Ok(line) = line {
            if line.len() > 1 {
                let row = line
                    .split_whitespace()
                    .map(|s| {
                        s.parse()
                            .expect(&format!("Cannot parse {:?} in {:?}", s, &line))
                    })
                    .collect();
                rows.push(row);

                if rows.len() == BINGO_ROWS {
                    let board = Board::new(rows).unwrap();
                    boards.push(board);
                    rows = Vec::new();
                }
            }
        }
    }

    (numbers, boards)
}

fn play_bingo(numbers: &Vec<BingoNumber>, boards: &Vec<Board>, break_on_first: bool) -> u32 {
    let mut marked_numbers: HashSet<BingoNumber> = HashSet::new();
    let mut remaining_boards: HashSet<usize> = HashSet::from_iter(0..boards.len());

    let mut last_number = 0;
    let mut score = 0;

    'outer: for number in numbers {
        marked_numbers.insert(*number);

        for (i, board) in boards.iter().enumerate() {
            if remaining_boards.contains(&i) && board_wins(board, &marked_numbers) {
                remaining_boards.remove(&i);
                last_number = *number;
                score = calc_score(board, &marked_numbers);
                if break_on_first {
                    break 'outer;
                }
            }
        }
    }

    (last_number as u32) * score
}

fn part1(numbers: &Vec<BingoNumber>, boards: &Vec<Board>) -> u32 {
    play_bingo(numbers, boards, true)
}

fn part2(numbers: &Vec<BingoNumber>, boards: &Vec<Board>) -> u32 {
    play_bingo(numbers, boards, false)
}


fn main() {
    let (numbers, boards) = read_input("inputs/04.txt");
    println!("Part 1: Product of board score and last number of first winning board: {}", part1(&numbers, &boards));
    println!("Part 2: Product of board score and last number of last winning board: {}", part2(&numbers, &boards));
}

#[cfg(test)]
mod tests04 {
    use super::*;

    #[test]
    fn test01() {
        let (numbers, boards) = read_input("test_inputs/04_01.txt");

        let product = part1(&numbers, &boards);
        assert_eq!(product, 4512);

        let product = part2(&numbers, &boards);
        assert_eq!(product, 1924);
    }
}
