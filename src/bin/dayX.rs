use std::fs::File;
use std::io::{BufReader, BufRead};


fn read_input(filename: &str) -> Vec<u32> {
    let mut input = Vec::new();

    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            input.push(1)
        }
    }

    input
}

fn part1(input: &Vec<u32>) -> u32 {
    0
}

fn part2(input: &Vec<u32>) -> u32 {
    0
}

fn main() {
    // let input = read_input("inputs/X.txt");
    let input = read_input("test_inputs/X.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}


#[cfg(test)]
mod testsX {
    use super::*;

    #[test]
    fn test01() {
        let input = read_input("test_inputs/X.txt");
        assert_eq!(part1(&input), 150);
    }

    #[test]
    fn test02() {
        let input = read_input("test_inputs/X.txt");
        assert_eq!(part2(&input), 150);
    }
}
