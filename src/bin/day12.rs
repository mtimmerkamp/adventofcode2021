use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

#[derive(PartialEq, Eq, Hash)]
struct Cave {
    name: String,
    small: bool,
}

impl Cave {
    fn new(name: &str) -> Cave {
        let small = name == name.to_ascii_lowercase();
        Cave {
            name: String::from(name),
            small,
        }
    }
}

impl std::fmt::Debug for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

#[derive(Debug)]
struct CaveSystem {
    caves: HashSet<Rc<Cave>>,
    connections: HashSet<(Rc<Cave>, Rc<Cave>)>,
}

impl CaveSystem {
    fn new() -> CaveSystem {
        CaveSystem {
            caves: HashSet::new(),
            connections: HashSet::new(),
        }
    }

    fn add(&mut self, left: Cave, right: Cave) {
        let left = Rc::new(left);
        let right = Rc::new(right);
        self.caves.insert(Rc::clone(&left));
        self.caves.insert(Rc::clone(&right));
        self.connections.insert((left, right));
    }

    fn get_connections(&self, cave: &Cave) -> Vec<Rc<Cave>> {
        let mut connections = Vec::new();
        for (left, right) in &self.connections {
            if left.as_ref() == cave {
                connections.push(right.clone());
            } else if right.as_ref() == cave {
                connections.push(left.clone());
            }
        }
        connections
    }

    fn get(&self, name: &str) -> Option<Rc<Cave>> {
        for cave in &self.caves {
            if cave.name == name {
                return Some(Rc::clone(&cave));
            }
        }
        None
    }
}

fn read_input(filename: &str) -> CaveSystem {
    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut system = CaveSystem::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let lr: Vec<&str> = line.split('-').collect();

            let left = Cave::new(lr[0]);
            let right = Cave::new(lr[1]);

            system.add(left, right);
        }
    }

    system
}

fn format_path(path: &Vec<Rc<Cave>>) -> String {
    let mut s = String::new();
    for (i, cave) in path.iter().enumerate() {
        if i < path.len() - 1 {
            s.push_str(&format!("{:?} -> ", cave.name));
        } else {
            s.push_str(&format!("{:?}", cave.name));
        }
    }
    s
}

fn find_path(
    system: &CaveSystem,
    start: Rc<Cave>,
    end: Rc<Cave>,
    exclude: &mut HashSet<Rc<Cave>>,
) -> Vec<Vec<Rc<Cave>>> {
    // println!("find {:?} -> ... -> {:?} without {:?}", start, end, exclude);
    let mut paths = Vec::new();
    if start == end {
        return vec![vec![end]];
    }

    if start.small {
        exclude.insert(Rc::clone(&start));
    }

    let adjacent_caves = system.get_connections(&start);
    for next in adjacent_caves {
        if !exclude.contains(&next) {
            let mut other = find_path(system, Rc::clone(&next), Rc::clone(&end), exclude);

            // println!("Got:");
            // for path in &paths {
            //     println!("{}", format_path(path));
            // }

            for path in other.iter_mut() {
                path.insert(0, Rc::clone(&start));
            }
            for path in other {
                paths.push(path);
            }
        }
    }
    if start.small {
        exclude.remove(&start);
    }

    paths
}

fn part1(input: &CaveSystem) -> u32 {
    let start = input.get("start").unwrap();
    let end = input.get("end").unwrap();
    let mut exclude = HashSet::new();
    let paths = find_path(input, start, end, &mut exclude);

    // for path in &paths {
    //     println!("{}", format_path(path));
    // }

    paths.len() as u32
}

fn find_path2(
    system: &CaveSystem,
    start: Rc<Cave>,
    end: Rc<Cave>,
    exclude: &mut HashSet<Rc<Cave>>,
    skip_exclude: bool,
) -> Vec<Vec<Rc<Cave>>> {
    // println!("find {:?} -> ... -> {:?} without {:?}", start, end, exclude);
    let mut paths = Vec::new();
    if start == end {
        return vec![vec![end]];
    }

    let skip_choices = if skip_exclude && start.small {
        vec![true, false]
    } else {
        vec![false]
    };
    for skip in skip_choices {
        if start.small && !skip {
            exclude.insert(Rc::clone(&start));
        }

        let adjacent_caves = system.get_connections(&start);
        for next in adjacent_caves {
            if !exclude.contains(&next) {
                let mut other = find_path2(
                    system,
                    Rc::clone(&next),
                    Rc::clone(&end),
                    exclude,
                    skip_exclude && !skip,
                );

                // println!("Got:");
                // for path in &paths {
                //     println!("{}", format_path(path));
                // }

                for path in other.iter_mut() {
                    path.insert(0, Rc::clone(&start));
                }
                for path in other {
                    paths.push(path);
                }
            }
        }
        if start.small && !skip {
            exclude.remove(&start);
        }
    }

    paths
}

fn part2(input: &CaveSystem) -> u32 {
    let start = input.get("start").unwrap();
    let end = input.get("end").unwrap();
    let mut exclude = HashSet::new();
    exclude.insert(Rc::clone(&start));
    let paths = find_path2(input, start, end, &mut exclude, true);

    // for path in &paths {
    //     println!("{}", format_path(path));
    // }

    // Filter paths that appear twice.
    let mut set = HashSet::new();
    set.extend(paths.iter());
    set.len() as u32
}

fn main() {
    let input = read_input("inputs/12.txt");
    // let input = read_input("test_inputs/12_01.txt");
    // println!("{:?}", input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests12 {
    use super::*;

    #[test]
    fn test01() {
        for (filename, answer) in [
            ("test_inputs/12_01.txt", 10),
            ("test_inputs/12_02.txt", 19),
            ("test_inputs/12_03.txt", 226),
        ] {
            let input = read_input(filename);
            assert_eq!(part1(&input), answer);
        }
    }

    #[test]
    fn test02() {
        for (filename, answer) in [
            ("test_inputs/12_01.txt", 36),
            ("test_inputs/12_02.txt", 103),
            ("test_inputs/12_03.txt", 3509),
        ] {
            let input = read_input(filename);
            assert_eq!(part2(&input), answer);
        }
    }
}
