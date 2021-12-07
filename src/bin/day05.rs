use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;
use std::str::FromStr;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl FromStr for Point {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();
        let x = coords[0].parse()?;
        let y = coords[1].parse()?;

        Ok(Point { x, y })
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn points(&self) -> LinePoints {
        LinePoints::new(self)
    }
}

impl FromStr for Line {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<&str> = s.split(" -> ").collect();
        let start = Point::from_str(points[0])?;
        let end = Point::from_str(points[1])?;

        Ok(Line { start, end })
    }
}

struct LinePoints {
    current_x: i32,
    current_y: i32,
    last_x: i32,
    last_y: i32,
    last: bool,
    finished: bool,
}

impl LinePoints {
    fn new(line: &Line) -> LinePoints {
        LinePoints {
            current_x: line.start.x,
            current_y: line.start.y,
            last_x: line.end.x,
            last_y: line.end.y,
            last: false,
            finished: false,
        }
    }
}

impl Iterator for LinePoints {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        // This is only correct for straight horizontal, vertical and diagonal (i.e. 45°) lines.
        if self.finished {
            None
        }
        else if self.last {
            self.finished = true;
            Some(Point{x: self.current_x, y: self.current_y})
        }
        else {
            let x_dir = (self.last_x - self.current_x).signum();
            let y_dir = (self.last_y - self.current_y).signum();

            let point = Point {
                x: self.current_x,
                y: self.current_y,
            };

            self.current_x += x_dir;
            self.current_y += y_dir;

            self.last = self.current_x == self.last_x && self.current_y == self.last_y;

            Some(point)
        }
    }
}

/// Test whether a line is horizontal or vertical.
fn is_hvline(line: &Line) -> bool {
    line.start.x == line.end.x || line.start.y == line.end.y
}

fn read_input(filename: &str) -> Vec<Line> {
    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut input = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let line =
                Line::from_str(&line).expect(&format!("Cannot read line specification {:?}", line));
            input.push(line);
        }
    }

    input
}

#[allow(dead_code)]
fn print_map(map: &HashMap<(i32, i32), u32>) {
    let minx = map.iter().map(|(&(x, _), _)| x).min().unwrap_or(0);
    let miny = map.iter().map(|(&(_, y), _)| y).min().unwrap_or(0);
    let maxx = map.iter().map(|(&(x, _), _)| x).max().unwrap_or(10);
    let maxy = map.iter().map(|(&(_, y), _)| y).max().unwrap_or(10);

    for y in miny..=maxy {
        for x in minx..=maxx {
            if let Some(count) = map.get(&(x, y)) {
                print!("{}", count);
            }
            else {
                print!(".");
            }
        }
        println!("");
    }
}

fn count_overlaps(lines: &Vec<&Line>) -> u32 {
    let mut map: HashMap<(i32, i32), u32> = HashMap::new();

    for line in lines {
        for point in line.points() {
            let (x, y) = point.as_tuple();
            // println!("{:?}: {:?}", (x, y), map.get(&(x, y)));
            
            let count = match map.get(&(x, y)) {
                Some(&count) => count + 1,
                None => 1,
            };
            map.insert((x, y), count);
        }
    }

    // print_map(&map);

    map.iter()
        .map(|(_, &count)| count)
        .filter(|&count| count > 1)
        .count() as u32
}

fn part1(lines: &Vec<Line>) -> u32 {
    // Only consider vertical or horizontal lines.
    let lines: Vec<&Line> = lines.iter().filter(|&l| is_hvline(l)).collect();

    count_overlaps(&lines)
}

fn part2(lines: &Vec<Line>) -> u32 {
    // Consider all lines
    let lines: Vec<&Line> = lines.iter().collect();
    count_overlaps(&lines)
}

fn main() {
    let input = read_input("inputs/05.txt");
    // let input = read_input("test_inputs/05_01.txt");
    // println!("{:?}", input);

    let solution = part1(&input);
    println!(
        "Part 1: Number of tiles with more than with a cloud density higher than one: {}",
        solution
    );

    let solution = part2(&input);
    println!(
        "Part 1: Number of tiles with more than with a cloud density higher than one: {}",
        solution
    );
}

#[cfg(test)]
mod tests05 {
    use super::*;

    #[test]
    fn test01() {
        let input = read_input("test_inputs/05_01.txt");
        let solution = part1(&input);

        assert_eq!(solution, 5);
    }

    #[test]
    fn test02() {
        let input = read_input("test_inputs/05_01.txt");
        let solution = part2(&input);

        assert_eq!(solution, 12);
    }
}
