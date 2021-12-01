use std::io::{BufReader, BufRead};
use std::fs::File;

fn read_input(filename: &str) -> Vec<u16> {
    let file = File::open(filename).expect("Cannot read file");
    let reader = BufReader::new(file);

    let mut numbers: Vec<u16> = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let number: u16 = line.parse().expect("File must only contain numbers.");
            numbers.push(number);
        }
    }

    numbers
}

fn part1(heights: &Vec<u16>) -> u32 {
    heights.iter()
        .zip(heights.iter().skip(1))
        .fold(0, |n, (last, current)| if current > last {n + 1} else {n})
}

fn part2(heights: &Vec<u16>) -> u32 {
    let mut three_sums = heights.iter().zip(heights.iter().skip(1)).zip(heights.iter().skip(2)).map(|((h1, h2), h3)| h1 + h2 + h3);
    if let Some(first) = three_sums.next() {
        let (n, _) = three_sums.fold((0, first), |(n, last), current| (if current > last {n + 1} else {n}, current));
        n
    }
    else {
        0
    }
}

fn main() {
    let height_values = read_input("inputs/01.txt");
    // println!("{:?}", height_values);

    let bigger_values = part1(&height_values);
    println!("Part 1: There are {} values bigger than their precursors.", bigger_values);

    let bigger_values = part2(&height_values);
    println!("Part 2: There are {} 3-sums bigger than their precursors.", bigger_values);
}


#[cfg(test)]
mod tests01 {
    use super::*;

    #[test]
    fn test01() {
        let height_values = read_input("test_inputs/01_01.txt");
        let bigger_values = part1(&height_values);

        assert_eq!(bigger_values, 7);
    }

    #[test]
    fn test02() {
        let height_values = read_input("test_inputs/01_01.txt");
        let bigger_values = part2(&height_values);

        assert_eq!(bigger_values, 5);
    }
}
