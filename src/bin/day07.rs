use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(filename: &str) -> Vec<u32> {
    let file = File::open(filename).expect("Cannot open file");
    let mut reader = BufReader::new(file);

    let mut line: String = String::new();
    reader
        .read_line(&mut line)
        .expect("Cannot read first line.");

    let input: Vec<u32> = line
        .trim_end_matches("\n")
        .split(',')
        .map(|s| {
            s.parse()
                .expect("Line must contain comma separated integers.")
        })
        .collect();

    input
}

fn calc_abs_deviation(list: &Vec<u32>, v: u32) -> u32 {
    list.iter()
        .map(|&x| if x > v { x - v } else { v - x })
        .sum()
}

fn part1(positions: &Vec<u32>) -> u32 {
    // The solution for this task is to determine the median. The
    // median m minimizes the absolute deviation
    // (i.e. \Sum_{i=1}^{N} |x_i - m| ), which is just the task at hand.
    let mut p = positions.clone();
    p.sort();

    let i_mid = p.len() / 2;
    if p.len() % 2 == 0 {
        // try left and right of middle
        calc_abs_deviation(&p, p[i_mid - 1]).min(calc_abs_deviation(&p, p[i_mid]))
    } else {
        // use middle.
        calc_abs_deviation(&p, p[i_mid])
    }
}

fn calc_fuel(positions: &Vec<u32>, target: u32) -> u32 {
    positions
        .iter()
        .map(|&p| {
            let distance = if p > target { p - target } else { target - p };
            // Use sum formula:
            // \Sum_{i=1}^\text{distance} i = \frac{\text{distance} \cdot (\text{distance} + 1)}{2}
            let fuel = distance * (distance + 1) / 2;
            fuel
        })
        .sum()
}

fn part2(positions: &Vec<u32>) -> u32 {
    // The optimum value lies around the mean position. To be more specific in the 
    // range p_mean - 1/2 <= p_opt <= p_mean + 1/2.
    // As we are working with whole numbers, we extend that range to +/- 1 around 
    // the mean position.
    let mean: u32 = positions.iter().cloned().sum::<u32>() / positions.len() as u32;
    (mean-1..=mean+1)
        .map(|p| {
            let fuel = calc_fuel(&positions, p);
            // println!("Moving to {} requires {} fuel.", p, fuel);
            fuel
        })
        .min()
        .unwrap()
}

fn main() {
    let input = read_input("inputs/07.txt");
    // let input = read_input("test_inputs/07_01.txt");
    // println!("{:?}", input);

    let solution = part1(&input);
    println!(
        "Part 1: The best position for alignment requires {} fuel.",
        solution
    );

    let solution = part2(&input);
    println!(
        "Part 2: The best position for alignment requires {} fuel.",
        solution
    );
}

#[cfg(test)]
mod tests07 {
    use super::*;

    #[test]
    fn test01() {
        let input = read_input("test_inputs/07_01.txt");
        let solution = part1(&input);

        assert_eq!(solution, 37);
    }

    #[test]
    fn test02() {
        let input = read_input("test_inputs/07_01.txt");
        let solution = part2(&input);

        assert_eq!(solution, 168);
    }
}
