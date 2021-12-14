use std::collections::{HashSet, LinkedList};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Fold {
    X(u32),
    Y(u32),
}

fn read_input(filename: &str) -> (Vec<(u32, u32)>, Vec<Fold>) {
    let mut input = Vec::new();

    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);

    // Read points
    let mut lines = reader.lines();
    while let Some(Ok(line)) = lines.next() {
        if line.len() == 0 {
            break; // Break on first empty line. After this there are only fold instructions.
        }

        let parts: Vec<u32> = line.split(',').map(|s| s.parse().unwrap()).collect();
        if parts.len() == 2 {
            input.push((parts[0], parts[1]));
        } else {
            panic!("Parsing error. Expected two numbers separated by a ','.")
        }
    }

    // Read folding instructions
    let mut folds = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        let parts: Vec<&str> = line.split('=').collect();

        if parts.len() != 2 {
            panic!("Parsing error. Expected a '='.")
        }
        let coordinate = parts[1].parse().unwrap();

        let len = parts[0].len();
        let fold = match &parts[0][len - 1..len] {
            "x" => Fold::X(coordinate),
            "y" => Fold::Y(coordinate),
            a => panic!("Unknown fold axis {:?}", a),
        };
        folds.push(fold);
    }

    (input, folds)
}

fn fold(points: &mut HashSet<(u32, u32)>, instruction: &Fold) {
    let change_sets: LinkedList<_> = match instruction {
        &Fold::Y(yf) => points
            .iter()
            .filter(|(_x, y)| *y > yf)  // Only handle points below yf
            .map(|(x, y)| ((*x, *y), (*x, yf - (*y - yf))))// Return old and new positions
            .collect(),
        &Fold::X(xf) => points
            .iter()
            .filter(|(x, _y)| *x > xf)  // Only handle points right of xf
            .map(|(x, y)| ((*x, *y), (xf - (*x - xf), *y)))  // Return old and new positions
            .collect(),
    };

    // Remove old points and add reflected ones.
    for (old_point, new_point) in change_sets {
        points.remove(&old_point);
        points.insert(new_point);
    }
}

fn format_points(points: &HashSet<(u32, u32)>) -> String {
    let mut s = String::new();

    let x_min = *points.iter().map(|(x, _y)| x).min().unwrap();
    let y_min = *points.iter().map(|(_x, y)| y).min().unwrap();
    let x_max = *points.iter().map(|(x, _y)| x).max().unwrap();
    let y_max = *points.iter().map(|(_x, y)| y).max().unwrap();

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let c = if points.contains(&(x, y)) { '#' } else { '.' };
            s.push(c);
        }
        s.push('\n');
    }

    s
}

fn part1(points: &Vec<(u32, u32)>, folds: &Vec<Fold>) -> u32 {
    let mut points: HashSet<(u32, u32)> = HashSet::from_iter(points.iter().cloned());

    let first_fold = folds.get(0).unwrap();

    // println!("Paper:\n{}", format_points(&points));
    fold(&mut points, first_fold);
    // println!("Paper after {:?}\n{}", first_fold, format_points(&points));

    // for instruction in folds {
    //     fold(&mut points, instruction);
    //     println!("Paper after {:?}\n{}", instruction, format_points(&points));
    // }

    points.len() as u32
}

fn part2(points: &Vec<(u32, u32)>, folds: &Vec<Fold>) -> u32 {
    let mut points: HashSet<(u32, u32)> = HashSet::from_iter(points.iter().cloned());

    // println!("Paper:\n{}", format_points(&points));
    for instruction in folds {
        fold(&mut points, instruction);
        // println!("Paper after {:?}\n{}", instruction, format_points(&points));
    }
    println!("Final paper:\n{}", format_points(&points));

    0
}

fn main() {
    let (points, folds) = read_input("inputs/13.txt");
    // let (points, folds) = read_input("test_inputs/13.txt");
    // println!("Points: {:?}\nFolds: {:?}", points, folds);

    println!("Part 1: {}", part1(&points, &folds));
    println!("Part 2: {}", part2(&points, &folds));
}

#[cfg(test)]
mod tests13 {
    use super::*;

    #[test]
    fn test01() {
        let (points, folds) = read_input("test_inputs/13.txt");
        assert_eq!(part1(&points, &folds), 17);
    }
}
