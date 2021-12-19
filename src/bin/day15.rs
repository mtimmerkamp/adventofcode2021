use std::collections::LinkedList;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FusedIterator;
use std::vec;

fn read_input(filename: &str) -> Vec<Vec<u8>> {
    let mut input: Vec<Vec<u8>> = Vec::new();

    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.len() == 0 {
                continue;
            }

            let mut row = Vec::new();
            for i in 0..line.len() {
                row.push(line[i..=i].parse().unwrap())
            }
            input.push(row);
        }
    }

    input
}

type MapElement = u8;
type Map = Vec<Vec<MapElement>>;
type Index2 = (usize, usize);

#[derive(PartialEq)]
enum NeighborhoodIteratorState {
    Top,
    Left,
    Bottom,
    Right,
    Done,
}

struct NeighborhoodIter {
    pos: Index2,
    rows: usize,
    cols: usize,
    state: NeighborhoodIteratorState,
}

impl NeighborhoodIter {
    fn new<T>(home: &Index2, map: &Vec<Vec<T>>) -> NeighborhoodIter {
        let rows = map.len();
        let columns = map.get(0).map_or(0, |row| row.len());

        NeighborhoodIter {
            pos: home.clone(),
            rows,
            cols: columns,
            state: NeighborhoodIteratorState::Top,
        }
    }
}

impl Iterator for NeighborhoodIter {
    type Item = Index2;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let pos = match self.state {
                NeighborhoodIteratorState::Top => {
                    self.state = NeighborhoodIteratorState::Left;
                    (self.pos.0.checked_sub(1), Some(self.pos.1))
                }
                NeighborhoodIteratorState::Left => {
                    self.state = NeighborhoodIteratorState::Bottom;
                    (Some(self.pos.0), self.pos.1.checked_sub(1))
                }
                NeighborhoodIteratorState::Bottom => {
                    self.state = NeighborhoodIteratorState::Right;
                    (self.pos.0.checked_add(1), Some(self.pos.1))
                }
                NeighborhoodIteratorState::Right => {
                    self.state = NeighborhoodIteratorState::Done;
                    (Some(self.pos.0), self.pos.1.checked_add(1))
                }
                _ => (None, None),
            };

            match pos {
                (Some(i), Some(j)) => {
                    if i < self.rows && j < self.cols {
                        break Some((i, j));
                    }
                }
                _ => {
                    if self.state == NeighborhoodIteratorState::Done {
                        break None;
                    }
                }
            }
        }
    }
}

impl FusedIterator for NeighborhoodIter {}

#[allow(dead_code)]
fn format_map<T>(map: &Vec<Vec<T>>) -> String
where
    T: std::fmt::Display,
{
    let str_map: Vec<Vec<String>> = map
        .iter()
        .map(|row| row.iter().map(|v| format!("{}", v)).collect())
        .collect();

    let max_len = str_map
        .iter()
        .map(|row| row.iter().map(|s| s.len()).max().unwrap())
        .max()
        .unwrap();

    str_map
        .iter()
        .map(|row| {
            row.iter()
                .map(|s| format!("{:>1$}", s, max_len))
                .fold(String::new(), |s_row, s| format!("{} {}", s_row, s))
        })
        .fold(String::new(), |s_map, s_row| {
            format!("{}\n{}", s_map, s_row)
        })
}

fn find_lowest_risk(map: &Map) -> u64 {
    fn calc_total_risk(pos: &Index2, total_risks: &Vec<Vec<u64>>, map: &Map) -> u64 {
        let (x, y) = *pos;
        let self_cost = map[x][y] as u64;
        let min_neighbor_cost = NeighborhoodIter::new(&(x, y), &map)
            .map(|(i, j)| total_risks[i][j])
            .min()
            .unwrap();

        self_cost + min_neighbor_cost
    }

    let max_total_risk = (map.len() as u64).pow(2) * 9;
    let mut total_risks: Vec<Vec<u64>> = vec![vec![max_total_risk; map.len()]; map.len()];

    // Initialize start
    total_risks[0][0] = 0;
    // println!("\nStart:\n{}", format_map(&field));

    // Fill all other total risks by looking at the direct neighbors and taking the smallest risk.
    for i in 0..2 * map.len() {
        let x1;
        let x2;
        let y0;
        if i < map.len() {
            x1 = 0;
            x2 = i;
            y0 = i;
        } else {
            x1 = 1 + i - map.len();
            x2 = map.len() - 1;
            y0 = map.len() + (i - map.len());
        }

        for x in x1..=x2 {
            let y = y0 - x;

            let total_cost = calc_total_risk(&(x, y), &total_risks, map);
            if total_cost < total_risks[x][y] {
                total_risks[x][y] = total_cost;
            }
        }

        // println!("\nAfter step {}:\n{}", i, format_map(&field));
    }

    // Check if there are any lower risks paths.
    let mut points_to_recalculate = LinkedList::new();

    for x in 0..map.len() {
        for y in 0..map.len() {
            points_to_recalculate.push_back((x, y));
        }
    }
    while let Some((x, y)) = points_to_recalculate.pop_front() {
        let total_cost = calc_total_risk(&(x, y), &total_risks, map);
        if total_cost < total_risks[x][y] {
            total_risks[x][y] = total_cost;

            NeighborhoodIter::new(&(x, y), map)
                .for_each(|pos| points_to_recalculate.push_back(pos));
        }
    }

    // println!("\nFinally:\n{}", format_map(&total_risks));

    total_risks[map.len() - 1][map.len() - 1]
}

fn part1(map: &Map) -> u64 {
    find_lowest_risk(map)
}

fn part2(map: &Vec<Vec<u8>>) -> u64 {
    let n = map.len();
    // Create a 5 times larger map
    let mut large_map = vec![vec![0; n * 5]; n * 5];

    for i in 0..5 {
        for j in 0..5 {
            let added_risk = (i + j) as u8;

            for k in 0..n {
                for l in 0..n {
                    large_map[k + i * n][l + j * n] = (map[k][l] + added_risk - 1) % 9 + 1;
                }
            }
        }
    }

    // println!("Map:\n{}", format_map(&map));
    // println!("Large Map:\n{}", format_map(&large_map));
    find_lowest_risk(&large_map)
}

fn main() {
    let input = read_input("inputs/15.txt");
    // let input = read_input("test_inputs/15.txt");

    // let d: Vec<Index2> = NeighborhoodIter::new(&(0, 0), &input).collect();
    // println!("{:?}", d);

    // println!("{:?}", input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests15 {
    use super::*;

    #[test]
    fn test01() {
        let input = read_input("test_inputs/15.txt");
        assert_eq!(part1(&input), 40);
    }

    #[test]
    fn test02() {
        let input = read_input("test_inputs/15.txt");
        assert_eq!(part2(&input), 315);
    }
}
