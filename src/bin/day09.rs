use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Map = Vec<Vec<u8>>;

fn read_input(filename: &str) -> Map {
    let mut input = Vec::new();

    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            let row: Vec<u8> = line.chars().map(|c| c as u8 - '0' as u8).collect();
            input.push(row);
        }
    }

    input
}

fn get_neighborhood(map: &Map, col: usize, row: usize) -> [u8; 4] {
    const DEFAULT: u8 = 9;
    let get = |row, col| match (row, col) {
        (Some(row), Some(col)) => map
            .get(row)
            .map_or(DEFAULT, |r: &Vec<u8>| *r.get(col).unwrap_or(&DEFAULT)),
        _ => DEFAULT,
    };
    let rows = map.len();
    let columns = map.get(row).map_or(0, |row| row.len());

    if row > rows || col > columns {
        panic!("Index ({}, {}) exceeds map dimensions.", col, row);
    }

    let top = get(row.checked_sub(1), Some(col));
    let left = get(Some(row), col.checked_sub(1));
    let bottom = get(row.checked_add(1), Some(col));
    let right = get(Some(row), col.checked_add(1));

    [top, left, right, bottom]
}

fn find_minima(map: &Map) -> Vec<(usize, usize)> {
    let mut minima = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            let neighbors = get_neighborhood(map, j, i);
            if neighbors.iter().all(|v| v > value) {
                minima.push((i, j));
            }
        }
    }

    minima
}

fn part1(map: &Map) -> u32 {
    let minima = find_minima(map);
    let risk = minima.iter().map(|(i, j)| map[*i][*j] as u32 + 1).sum();
    risk
}

fn part2(map: &Map) -> u32 {
    const OFFSETS: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

    let mut sizes = Vec::new();

    let minima = find_minima(map);
    for &(row, col) in &minima {
        let mut new_points: VecDeque<(usize, usize)> = VecDeque::new();
        let mut old_points: HashSet<(usize, usize)> = HashSet::new();
        new_points.push_back((row, col));
        // println!(" Checking ({}, {})", row, col);

        let mut size: u32 = 1;

        loop {
            let (i, j) = match new_points.pop_front() {
                Some(v) => v,
                None => break,
            };
            old_points.insert((i, j));
            // println!("  Check ({}, {})", i, j);

            for (&neighbor, &(dj, di)) in get_neighborhood(map, j, i).iter().zip(OFFSETS.iter())
            {
                let new_row = (i as isize + di) as usize;
                let new_col = (j as isize + dj) as usize;
                let coords = (new_row, new_col);
                if neighbor != 9 {
                    if !old_points.contains(&coords) {
                        old_points.insert(coords);
                        // println!("    Adding {:?}", coords);
                        new_points.push_back(coords);
                        size += 1;
                    }
                } else {
                    old_points.insert(coords);
                }
            }
        }

        // println!("-> Size: {}", size);
        sizes.push(size);
    }

    sizes.sort_unstable();

    sizes.iter().rev().take(3).fold(1, |product, &size| product * size)
}

fn main() {
    let input = read_input("inputs/09.txt");
    // let input = read_input("test_inputs/09.txt");
    // println!("{:?}", input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests09 {
    use super::*;

    #[test]
    fn test01() {
        let input = read_input("test_inputs/09.txt");
        assert_eq!(part1(&input), 15);
    }

    #[test]
    fn test01_solution() {
        let input = read_input("inputs/09.txt");
        assert_eq!(part1(&input), 512);
    }

    #[test]
    fn test02() {
        let input = read_input("test_inputs/09.txt");
        assert_eq!(part2(&input), 1134);
    }
}
