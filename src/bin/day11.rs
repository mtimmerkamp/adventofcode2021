use std::collections::LinkedList;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Map = Vec<Vec<u8>>;

fn read_input(filename: &str) -> Map {
    let mut input = Vec::new();

    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            let x: u64 = line.parse().unwrap();
            let row: Vec<u8> = (0..line.len())
                .map(|i| (x / 10u64.pow((line.len() - i - 1) as u32) % 10) as u8)
                .collect();
            input.push(row);
        }
    }

    input
}

#[allow(dead_code)]
fn format_map(map: &Map) -> String {
    let mut s = String::new();
    for row in map {
        for &value in row {
            let rep = if value > 9 {
                String::from("0")
            } else {
                value.to_string()
            };
            s.push_str(&rep);
        }
        s.push('\n');
    }
    s
}

// Simulates a step and returns the number of flashes.
fn simulate_step(map: &mut Map) -> u32 {
    const OFFSETS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut flashes = 0;

    // Increase all counters by one.
    map.iter_mut()
        .for_each(|row| row.iter_mut().for_each(|counter| *counter += 1));

    // Find counters greater than 9 and increase count of all their
    // neighbors until no more new counters are greather than 9.
    let mut flashing_points = LinkedList::new();

    for (i, row) in map.iter_mut().enumerate() {
        for (j, value) in row.iter_mut().enumerate() {
            if *value == 10 {
                // flashes += 1;
                // *value += 1;  // Increase to 11, so it will not be counted again.

                flashing_points.push_back((i, j));
            }
        }
    }

    while let Some((i, j)) = flashing_points.pop_front() {
        map.get_mut(i)
            .unwrap()
            .get_mut(j)
            .map(|v| *v = v.saturating_add(1))
            .unwrap(); // Increase count to (at least) 11, so it will not be counted again.
        flashes += 1;

        // Increase count of all neighbors
        for (di, dj) in OFFSETS {
            let i2 = (i as isize + di) as usize;
            let j2 = (j as isize + dj) as usize;
            let neighbor_value = map.get_mut(i2).and_then(|row| row.get_mut(j2));
            if let Some(neighbor_value) = neighbor_value {
                *neighbor_value = neighbor_value.saturating_add(1);
                if *neighbor_value == 10 {
                    // Signal only neighbors of positions that just became greater than 9.
                    flashing_points.push_back((i2, j2));
                }
            }
        }
    }

    // Reset all flashing points to zero.
    map.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|count| {
            if *count > 9 {
                *count = 0
            }
        })
    });

    flashes
}

fn part1(input: &Map) -> u32 {
    let mut map = input.clone();
    const ROUNDS: u16 = 100;

    let mut flashes = 0;

    for _round in 1..=ROUNDS {
        flashes += simulate_step(&mut map);
        // println!("After step {}:\n{}\n", _round, format_map(&map));
    }

    flashes
}

fn part2(input: &Map) -> u32 {
    let mut map = input.clone();
    let rows = map.len();
    let cols = map.get(0).map_or(0, Vec::len);

    let mut round = 0;
    loop {
        round += 1;
        let flashes = simulate_step(&mut map);
        if flashes as usize == rows * cols {
            break;
        }
    }

    round
}

fn main() {
    let input = read_input("inputs/11.txt");
    // let input = read_input("test_inputs/11.txt");
    // println!("{}", format_map(&input));

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests11 {
    use super::*;

    #[test]
    fn test01() {
        let input = read_input("test_inputs/11.txt");
        assert_eq!(part1(&input), 1656);
    }

    #[test]
    fn test02() {
        let input = read_input("test_inputs/11.txt");
        assert_eq!(part2(&input), 195);
    }
}
