//! This is a solution to day 8 but the second part is quite extensive 
//! because I missed the point that the first ten numbers are the 
//! numbers from zero to nine. The algorithm implemented here assumes 
//! only that the given encoded numbers can be uniquely resolved. 
//! Otherwise, the algirithm is quite general and will find a solution,
//! possibly not even considering all numbers. It generates rules first
//! based on the few numbers directly recognizable (1, 4, 7 
//! and 8 [although 8 is actually no use to generate rules]). These 
//! numbers can be used to generate rules which are the possible 
//! (and impossible) connections of cables and segments. Then, it
//! goes through all numbers and tests whether a given number could
//! be encoded by a given pattern considering the already found rules.
//! 
//! In the end we retrieve a mapping of the encoded segments to the 
//! clear segments, which is used to calculate the displayed value.

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Segment {
    const fn members() -> [Segment; 7] {
        [
            Segment::A,
            Segment::B,
            Segment::C,
            Segment::D,
            Segment::E,
            Segment::F,
            Segment::G,
        ]
    }

    fn get_segments(number: u8) -> Vec<Segment> {
        match number {
            0 => [
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::E,
                Segment::F,
                Segment::G,
            ]
            .to_vec(),
            1 => [Segment::C, Segment::F].to_vec(),
            2 => [Segment::A, Segment::C, Segment::D, Segment::E, Segment::G].to_vec(),
            3 => [Segment::A, Segment::C, Segment::D, Segment::F, Segment::G].to_vec(),
            4 => [Segment::B, Segment::C, Segment::D, Segment::F].to_vec(),
            5 => [Segment::A, Segment::B, Segment::D, Segment::F, Segment::G].to_vec(),
            6 => [
                Segment::A,
                Segment::B,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G,
            ]
            .to_vec(),
            7 => [Segment::A, Segment::C, Segment::F].to_vec(),
            8 => Self::members().to_vec(),
            9 => [
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::F,
                Segment::G,
            ]
            .to_vec(),
            _ => [].to_vec(),
        }
    }
}

impl Debug for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letter = match self {
            Segment::A => 'a',
            Segment::B => 'b',
            Segment::C => 'c',
            Segment::D => 'd',
            Segment::E => 'e',
            Segment::F => 'f',
            Segment::G => 'g',
        };
        f.write_str(&letter.to_string())
    }
}

#[derive(Debug)]
enum SegmentError {
    InvalidName(char),
}

impl TryFrom<char> for Segment {
    type Error = SegmentError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let segment = match c {
            'a' => Segment::A,
            'b' => Segment::B,
            'c' => Segment::C,
            'd' => Segment::D,
            'e' => Segment::E,
            'f' => Segment::F,
            'g' => Segment::G,
            c => return Err(SegmentError::InvalidName(c)),
        };

        Ok(segment)
    }
}

struct Pattern {
    pattern: Vec<Segment>,
}

impl Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for a in &self.pattern {
            a.fmt(f)?;
        }
        Ok(())
    }
}

impl FromStr for Pattern {
    type Err = SegmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pattern = Vec::new();
        for c in s.chars() {
            pattern.push(c.try_into()?);
        }
        Ok(Pattern { pattern })
    }
}

impl Pattern {
    fn has_unique_digit_count(&self) -> bool {
        let n = self.pattern.len();
        n == 2 || n == 3 || n == 4 || n == 7
    }

    fn get_unique_number(&self) -> Option<u8> {
        match self.pattern.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        }
    }
}

struct Entry {
    patterns: Vec<Pattern>,
    display: Vec<Pattern>,
}

impl Entry {
    fn format_patterns(patterns: &Vec<Pattern>) -> String {
        let patterns_strs: Vec<String> = patterns
            .iter()
            .map(|pattern| format!("{:?}", pattern))
            .collect();
        patterns_strs[..].join(" ")
    }
}

impl Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!(
            "{} | {}",
            Entry::format_patterns(&self.patterns),
            Entry::format_patterns(&self.display)
        );
        f.write_str(&s)
    }
}

fn read_input(filename: &str) -> Result<Vec<Entry>, SegmentError> {
    let mut input = Vec::new();

    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            let mut parts = line.split(" | ");
            let patterns_str = parts.next().expect("Each line must contain patterns.");
            let display_str = parts
                .next()
                .expect("Each line must contain a output value.");

            let mut patterns: Vec<Pattern> = Vec::new();
            for p in patterns_str.split(' ') {
                patterns.push(Pattern::from_str(p)?);
            }
            let mut display: Vec<Pattern> = Vec::new();
            for p in display_str.split(' ') {
                display.push(Pattern::from_str(p)?);
            }

            input.push(Entry { patterns, display });
        }
    }

    Ok(input)
}

fn part1(entries: &Vec<Entry>) -> usize {
    entries
        .iter()
        .map(|entry| {
            entry
                .display
                .iter()
                .filter(|pattern| pattern.has_unique_digit_count())
                .count()
        })
        .sum()
}

struct SegmentStore<'a> {
    available_choices: Vec<&'a HashSet<Segment>>,
}

impl<'a> SegmentStore<'a> {
    fn new() -> Self {
        SegmentStore {
            available_choices: Vec::new(),
        }
    }

    fn add_clears(&mut self, choices: &'a HashSet<Segment>) {
        self.available_choices.push(choices);
    }

    fn aquire_clear(&mut self, segment: &Segment) -> bool {
        let mut index: Option<usize> = None;
        for (i, segments) in self.available_choices.iter().enumerate() {
            if segments.contains(segment) {
                index = Some(i);
            }
        }

        if let Some(i) = index {
            self.available_choices.remove(i);
            true
        } else {
            false
        }
    }

    fn remaining(&self) -> usize {
        self.available_choices.len()
    }
}

enum Rule {
    None,
    One(u8),
    Many(Vec<u8>),
}

#[derive(Debug)]
struct Rules {
    rules: HashMap<Segment, HashSet<Segment>>,
}

impl Rules {
    fn new() -> Rules {
        let rules = HashMap::new();
        Rules { rules }
    }

    fn fill(&mut self) {
        for segment in Segment::members() {
            let segment_rules;
            match self.rules.get_mut(&segment) {
                Some(r) => segment_rules = r,
                None => {
                    let r = HashSet::new();
                    self.rules.insert(segment.clone(), r);
                    segment_rules = self.rules.get_mut(&segment).unwrap();
                }
            }
            for segment2 in Segment::members() {
                segment_rules.insert(segment2);
            }
        }
    }

    fn full() -> Rules {
        let mut rules = Rules::new();
        rules.fill();
        rules
    }

    fn is_resolved(&self) -> bool {
        self.rules
            .iter()
            .all(|(_, segment_rules)| segment_rules.len() == 1)
    }

    fn resolve(&self, pattern: &Pattern) -> Option<u8> {
        if !self.is_resolved() {
            return None;
        }

        match self.test_pattern(pattern) {
            Rule::One(number) => Some(number),
            _ => None,
        }
    }

    fn generate_rules(patterns: &Vec<&Pattern>) -> Option<Rules> {
        let unique_len_patterns = patterns.iter().filter(|p| p.has_unique_digit_count());

        let mut rules = Rules::full();
        // Add rules for known numbers
        for pattern in unique_len_patterns {
            let number = pattern.get_unique_number().unwrap();
            rules.update_with_known_pattern(pattern, number);
        }

        // Iterate through all patterns and try to figure out additional rules from the allowed patterns.
        let mut changed = true;
        let mut iter = patterns.iter();
        while !rules.is_resolved() {
            // Loop through the patterns multiple times if there was any
            // change since the last complete loop through the patterns.
            let pattern = match iter.next() {
                Some(e) => e,
                None => {
                    if !changed {
                        // Not finished and the last complete loop though the
                        // patterns did not further resolve the mapping. There
                        // is no point in trying again.
                        return None;
                    }
                    iter = patterns.iter();
                    changed = false;
                    continue;
                }
            };

            // Test to which numbers a pattern can match. If it matches to any
            // number, we might extract more rules from it.
            match rules.test_pattern(pattern) {
                Rule::One(number) => {
                    changed |= rules.update_with_known_pattern(pattern, number);
                }
                Rule::Many(numbers) => {
                    for number in numbers {
                        changed |= rules.update_with_known_pattern(pattern, number);
                    }
                }
                Rule::None => {
                    return None;
                }
            }
        }

        Some(rules)
    }

    fn test_pattern(&self, pattern: &Pattern) -> Rule {
        let mut number_count: u8 = 0;
        let mut numbers: Vec<u8> = Vec::new();

        // Test if pattern matches to any number.
        for number in 0u8..=9 {
            let mut store = SegmentStore::new();
            // Store all available clear segments.
            for code_segment in &pattern.pattern {
                let clear_segments = self.rules.get(code_segment).unwrap();
                store.add_clears(clear_segments);
            }

            // Remove clear segments required for number from the store
            // one by one. If all clear segments are available and none
            // remain, number can match.
            if Segment::get_segments(number)
                .iter()
                .all(|s| store.aquire_clear(s))
                && store.remaining() == 0
            {
                number_count += 1;
                numbers.push(number);
            }
        }

        match number_count {
            0 => Rule::None,
            1 => Rule::One(numbers[0]),
            _ => Rule::Many(numbers),
        }
    }

    fn update_with_known_pattern(&mut self, pattern: &Pattern, number: u8) -> bool {
        let mut changed = false;

        let pattern = &pattern.pattern;
        // println!("\nProcessing a {}: {:?}", number, pattern);

        let segments = Segment::get_segments(number);
        // println!("A {} must contain clear {:?}", number, segments);
        for code_segment in Segment::members() {
            let possibilites = self.rules.get_mut(&code_segment).unwrap();
            // println!("possibilities for {:?}: {:?}", code_segment, possibilites);

            if pattern.contains(&code_segment) {
                for clear_segment in Segment::members() {
                    if !segments.contains(&clear_segment) {
                        // println!(
                        //     "remove {:?} from {:?}: {:?}",
                        //     clear_segment, code_segment, possibilites
                        // );
                        changed |= possibilites.remove(&clear_segment);
                    }
                }
            } else {
                for clear_segment in Segment::members() {
                    if segments.contains(&clear_segment) {
                        // println!(
                        //     "remove {:?} from {:?}: {:?}",
                        //     clear_segment, code_segment, possibilites
                        // );
                        changed |= possibilites.remove(&clear_segment);
                    }
                }
            }
            // println!(
            //     "final possibilities for {:?}: {:?}",
            //     code_segment, possibilites
            // );
        }

        changed
    }
}

fn part2(entries: &Vec<Entry>) -> u32 {
    let mut total: u32 = 0;
    for entry in entries {
        let patterns: Vec<&Pattern> = entry.patterns.iter().chain(entry.display.iter()).collect();
        let rule = Rules::generate_rules(&patterns).expect("Unable to resolve the mapping.");
        let mut current: u32 = 0;

        // print!("{:?} -> ", entry.display);
        for pattern in &entry.display {
            let number = rule.resolve(pattern).unwrap();
            // print!("{}", number);

            current = current * 10 + number as u32;
        }
        // println!("");

        total += current;
    }

    total
}

fn main() {
    let input = read_input("inputs/08.txt").unwrap();
    // let input = read_input("test_inputs/08_01.txt").unwrap();

    // for entry in &input {
    //     println!("{:?}", entry);
    // }

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests08 {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = read_input("test_inputs/08_02.txt").unwrap();
        let solution = part1(&input);

        assert_eq!(solution, 26);
    }

    #[test]
    fn test_solution_part1() {
        let input = read_input("inputs/08.txt").unwrap();
        let solution = part1(&input);

        assert_eq!(solution, 387);
    }

    #[test]
    fn test_example1_part2() {
        let input = read_input("test_inputs/08_01.txt").unwrap();
        let solution = part2(&input);

        assert_eq!(solution, 5353);
    }

    #[test]
    fn test_example2_part2() {
        let input = read_input("test_inputs/08_02.txt").unwrap();
        let solution = part2(&input);

        assert_eq!(solution, 61229);
    }
}
