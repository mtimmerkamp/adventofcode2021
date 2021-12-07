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
        .split(',')
        .map(|s| {
            s.parse()
                .expect("Line must contain comma separated integers.")
        })
        .collect();

    input
}

#[allow(dead_code)]
fn simulate_fish_population_naive(timers: &Vec<u32>, days: u32) -> u32 {
    let reproduction_time: u32 = 7;
    let infertility_time: u32 = 2;

    let mut timers = timers.clone();
    let mut new_fishes = Vec::new();

    for _day in 1..=days {
        for timer in timers.iter_mut() {
            if *timer == 0 {
                new_fishes.push(reproduction_time + infertility_time - 1);
                *timer = reproduction_time;  // One will be subtracted afterwards
            }

            *timer = *timer - 1;
        }

        timers.append(&mut new_fishes);
        // println!("After {} days: {:?}", _day, timers);
    }

    timers.len() as u32
}

fn simulate_fish_population(timers: &Vec<u32>, days: u32) -> u64 {
    const REPRODUCTION_TIME: usize = 7;
    const INFERTILITY_TIME: usize = 2;

    const N: usize = REPRODUCTION_TIME + INFERTILITY_TIME;
    let mut fish_counts: [u64; N] = [0; N];
    for &timer in timers {
        fish_counts[timer as usize] += 1;
    }

    for _day in 1..=days {
        let new_fishes = fish_counts[0];
        for i in 1..fish_counts.len() {
            fish_counts[i - 1] = fish_counts[i];
        }
        fish_counts[REPRODUCTION_TIME - 1] += new_fishes;
        fish_counts[REPRODUCTION_TIME + INFERTILITY_TIME - 1] = new_fishes;
    }

    fish_counts.iter().sum()
}

fn part1(timers: &Vec<u32>) -> u64 {
    simulate_fish_population(timers, 80)
}

fn part2(timers: &Vec<u32>) -> u64 {
    simulate_fish_population(timers, 256)
}

fn main() {
    let input = read_input("inputs/06.txt");
    // println!("Initial state: {:?}", input);

    let solution = part1(&input);
    println!("Part 1: After 80 days there are {} lanternfish", solution);
    let solution = part2(&input);
    println!("Part 2: After 256 days there are {} lanternfish", solution);
}

#[cfg(test)]
mod tests06 {
    use super::*;

    #[test]
    fn test01() {
        let input = read_input("test_inputs/06_01.txt");
        let solution = simulate_fish_population_naive(&input, 80);

        assert_eq!(solution, 5934);
    }

    #[test]
    fn test02() {
        let input = read_input("test_inputs/06_01.txt");
        let solution = simulate_fish_population(&input, 80);

        assert_eq!(solution, 5934);
    }

    #[test]
    fn test03() {
        let input = read_input("test_inputs/06_01.txt");
        let solution = simulate_fish_population(&input, 256);

        assert_eq!(solution, 26984457539);
    }
}
