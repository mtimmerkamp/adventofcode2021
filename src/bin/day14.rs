use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Polymer = Vec<char>;
type Rules = HashMap<(char, char), char>;

fn read_input(filename: &str) -> (Polymer, Rules) {
    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    // Read polymer.
    let first_line = lines.next().unwrap().unwrap();
    let polymer = first_line.chars().collect();

    // Skip empty line.
    lines.next();

    // Read polymerization rules.
    let mut polymerization_rules = HashMap::new();
    for line in lines {
        if let Ok(line) = line {
            let mut parts = line.split(" -> ");

            let mut chars = parts.next().unwrap().chars();
            let raw = (chars.next().unwrap(), chars.next().unwrap());

            let product = parts.next().unwrap().chars().next().unwrap();

            polymerization_rules.insert(raw, product);
        }
    }

    (polymer, polymerization_rules)
}

type CacheKey = (char, char, u16);

fn add_counts(dest: &mut HashMap<char, u64>, src: &HashMap<char, u64>) {
    for (c, count) in src.iter() {
        if let Some(old_count) = dest.get_mut(c) {
            *old_count += count;
        } else {
            dest.insert(*c, *count);
        }
    }
}

fn polymerize_internal(
    left: char,
    right: char,
    rules: &Rules,
    steps: u16,
    cache: &mut HashMap<CacheKey, HashMap<char, u64>>,
) -> CacheKey {
    let key = (left, right, steps);
    let mut counts = HashMap::new();

    let mut write_cache = true;
    // Check whether the result is alread available from the cache.
    if let Some(cached_counts) = cache.get(&key) {
        add_counts(&mut counts, cached_counts);
        write_cache = false;
    } else if steps > 0 {
        // Get component that needs to be placed in the middle.
        if let Some(&middle) = rules.get(&(left, right)) {
            let count = *counts.get(&middle).unwrap_or(&0);
            counts.insert(middle, count + 1);

            // Recursively calculate which components must be placed between
            // the left (middle) component and the middle (right) one.
            let key_left = polymerize_internal(left, middle, rules, steps - 1, cache);
            let counts_left = cache.get(&key_left).unwrap();
            add_counts(&mut counts, &counts_left);

            let key_right = polymerize_internal(middle, right, rules, steps - 1, cache);
            let counts_right = cache.get(&key_right).unwrap();
            add_counts(&mut counts, &counts_right);
        }
    }

    if write_cache {
        // Insert count of components to be placed between left & right into the "cache".
        // Also do this when steps == 0 (an empty HashMap will be inserted then).
        cache.insert(key, counts);
    }
    key
}

fn polymerize(polymer: &Polymer, rules: &Rules, steps: u16) -> HashMap<char, u64> {
    let mut counts = HashMap::new();
    let mut cache = HashMap::new();

    // For every two adjacent components, calculate what must be placed in between them.
    for (&left, &right) in polymer.iter().zip(polymer.iter().skip(1)) {
        let count = *counts.get(&left).unwrap_or(&0);
        counts.insert(left, count + 1);

        let key = polymerize_internal(left, right, rules, steps, &mut cache);
        let new_counts = cache.get(&key).unwrap();
        add_counts(&mut counts, new_counts);
    }

    // Add last component of original polymer.
    let last = polymer.get(polymer.len() - 1).unwrap();
    let count = *counts.get(last).unwrap_or(&0);
    counts.insert(*last, count + 1);

    counts
}

fn get_min_max_difference(polymer: &Polymer, rules: &Rules, steps: u16) -> u64 {
    let counts = polymerize(&polymer, &rules, steps);

    // println!("\nFinal:{:?}", counts);

    let min = *counts.values().min().unwrap();
    let max = *counts.values().max().unwrap();

    max - min
}

fn part1(polymer: &Polymer, rules: &Rules) -> u64 {
    get_min_max_difference(polymer, rules, 10)
}

fn part2(polymer: &Polymer, rules: &Rules) -> u64 {
    get_min_max_difference(polymer, rules, 40)
}

fn main() {
    let (polymer, rules) = read_input("inputs/14.txt");
    // let (polymer, rules) = read_input("test_inputs/14.txt");
    // println!("{:?}, {:?}", polymer, rules);

    println!("Part 1: {}", part1(&polymer, &rules));
    println!("Part 2: {}", part2(&polymer, &rules));
}

#[cfg(test)]
mod tests14 {
    use super::*;

    #[test]
    fn test01() {
        let (polymer, rules) = read_input("test_inputs/14.txt");
        assert_eq!(part1(&polymer, &rules), 1588);
    }

    #[test]
    fn test02() {
        let (polymer, rules) = read_input("test_inputs/14.txt");
        assert_eq!(part2(&polymer, &rules), 2188189693529);
    }
}
