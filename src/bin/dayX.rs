use std::fs::File;
use std::io::{BufReader, BufRead};


fn read_input(filename: &str) -> Vec<u32> {
    let mut input = Vec::new();

    let file = File::open(filename).expect("Cannot open file");
    let mut reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            input.push(1)
        }
    }

    input
}

fn part1() {
}

fn part2() {
}

fn main() {
    // let input = read_input("inputs/02.txt");
}


#[cfg(test)]
mod testsX {
    use super::*;

    #[test]
    fn test01() {
        // let instructions = read_input("test_inputs/02_01.txt");
        // let product = part1(&instructions);

        // assert_eq!(product, 150);
    }
}
