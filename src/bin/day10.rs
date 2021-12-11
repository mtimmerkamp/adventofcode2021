use std::collections::LinkedList;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Bracket {
    ParenOpen,
    ParenClose,
    SquareOpen,
    SquareClose,
    AngledOpen,
    AngledClose,
    CurledOpen,
    CurledClose,
}

impl Bracket {
    fn from_char(c: char) -> Option<Bracket> {
        match c {
            '(' => Some(Bracket::ParenOpen),
            ')' => Some(Bracket::ParenClose),
            '[' => Some(Bracket::SquareOpen),
            ']' => Some(Bracket::SquareClose),
            '{' => Some(Bracket::CurledOpen),
            '}' => Some(Bracket::CurledClose),
            '<' => Some(Bracket::AngledOpen),
            '>' => Some(Bracket::AngledClose),
            _ => None,
        }
    }

    /// Match opening bracket to a closing bracket.
    fn closed_by(&self, other: &Bracket) -> bool {
        match (self, other) {
            (Bracket::ParenOpen, Bracket::ParenClose)
            | (Bracket::SquareOpen, Bracket::SquareClose)
            | (Bracket::CurledOpen, Bracket::CurledClose)
            | (Bracket::AngledOpen, Bracket::AngledClose) => true,
            _ => false,
        }
    }

    fn closes(&self, other: &Bracket) -> bool {
        other.closed_by(self)
    }

    const fn closing(&self) -> bool {
        match self {
            Bracket::ParenClose
            | Bracket::SquareClose
            | Bracket::CurledClose
            | Bracket::AngledClose => true,
            _ => false,
        }
    }

    const fn opening(&self) -> bool {
        !self.closing()
    }
}

impl Debug for Bracket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match &self {
            Bracket::ParenOpen => '(',
            Bracket::ParenClose => ')',
            Bracket::SquareOpen => '[',
            Bracket::SquareClose => ']',
            Bracket::CurledOpen => '{',
            Bracket::CurledClose => '}',
            Bracket::AngledOpen => '<',
            Bracket::AngledClose => '>',
        };
        f.write_fmt(format_args!("{}", c))
    }
}

fn read_input(filename: &str) -> Vec<Vec<Bracket>> {
    let mut input = Vec::new();

    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            let brackets = line
                .chars()
                .map(|c| Bracket::from_char(c).unwrap())
                .collect();
            input.push(brackets);
        }
    }

    input
}

fn get_error_score(bracket: &Bracket) -> u64 {
    match *bracket {
        Bracket::ParenClose => 3,
        Bracket::SquareClose => 57,
        Bracket::CurledClose => 1197,
        Bracket::AngledClose => 25137,
        _ => 0,
    }
}

fn part1(input: &Vec<Vec<Bracket>>) -> u64 {
    let mut total_score = 0;

    for brackets in input {
        let mut stack: LinkedList<Bracket> = LinkedList::new();

        for bracket in brackets {
            if bracket.closing() {
                if let Some(innermost) = stack.pop_back() {
                    let error = !innermost.closed_by(bracket);
                    if error {
                        total_score += get_error_score(bracket);
                        break;
                    }
                } else {
                    // More closing than opening brackets. Ignore.
                }
            } else {
                stack.push_back(*bracket);
            }
        }
    }

    total_score
}

fn get_value(bracket: &Bracket) -> u64 {
    match *bracket {
        Bracket::ParenOpen => 1,
        Bracket::SquareOpen => 2,
        Bracket::CurledOpen => 3,
        Bracket::AngledOpen => 4,
        _ => 0,
    }
}

fn part2(input: &Vec<Vec<Bracket>>) -> u64 {
    let mut scores: Vec<u64> = Vec::new();

    'outer: for brackets in input {
        let mut stack: LinkedList<Bracket> = LinkedList::new();

        for bracket in brackets {
            if bracket.closing() {
                if let Some(innermost) = stack.pop_back() {
                    let error = !innermost.closed_by(bracket);
                    if error {
                        continue 'outer;
                    }
                } else {
                    // More closing than opening brackets. Error.
                    continue 'outer;
                }
            } else {
                stack.push_back(*bracket);
            }
        }

        let score = stack
            .iter()
            .rev()
            .fold(0, |score, bracket| score * 5 + get_value(bracket));
        scores.push(score);
    }

    // Return middle score, i.e., median.
    scores.sort();
    scores[scores.len() / 2]
}

fn main() {
    let input = read_input("inputs/10.txt");
    // let input = read_input("test_inputs/10.txt");
    // println!("{:?}", input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests10 {
    use super::*;

    #[test]
    fn test01() {
        let input = read_input("test_inputs/10.txt");
        assert_eq!(part1(&input), 26397);
    }

    #[test]
    fn test02() {
        let input = read_input("test_inputs/10.txt");
        assert_eq!(part2(&input), 288957);
    }
}
