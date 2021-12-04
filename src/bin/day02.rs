use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
enum Instruction {
    Up(u32),
    Down(u32),
    Forward(u32),
}

fn read_input(filename: &str) -> Vec<Instruction> {
    let mut moves = Vec::new();

    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<_> = line.split_whitespace().collect();

            if parts.len() == 2 {
                let direction = parts.get(0).unwrap();
                let count: u32 = parts.get(1).unwrap().parse().unwrap();

                let instruction = match direction {
                    &"forward" => Instruction::Forward(count),
                    &"down" => Instruction::Down(count),
                    &"up" => Instruction::Up(count),
                    _ => {panic!("Invalid direction.")},
                };
                moves.push(instruction);
            }
        }
    }

    moves
}

fn interpret1(instructions: &Vec<Instruction>, depth: u32, position: u32) -> (u32, u32) {
    let mut depth = depth;
    let mut position = position;

    for instruction in instructions {
        match instruction {
            Instruction::Forward(x) => {
                position += x;
            },
            Instruction::Up(y) => {
                depth -= y;
            },
            Instruction::Down(y) => {
                depth += y;
            }
        }
    }

    (depth, position)
}

fn part1(instructions: &Vec<Instruction>) -> u32 {
    let (depth, position) = interpret1(instructions, 0, 0);
    depth * position
}

fn interpret2(instructions: &Vec<Instruction>, depth: u32, position: u32) -> (u32, u32) {
    let mut depth = depth;
    let mut position = position;
    let mut aim = 0;

    for instruction in instructions {
        // println!("{:?} {} {} {}", instruction, depth, position, aim);
        match instruction {
            Instruction::Forward(x) => {
                position += x;
                depth += aim * x;
            },
            Instruction::Up(y) => {
                aim -= y;
            },
            Instruction::Down(y) => {
                aim += y;
            }
        }
    }

    (depth, position)
}

fn part2(instructions: &Vec<Instruction>) -> u32 {
    let (depth, position) = interpret2(instructions, 0, 0);
    depth * position
}

fn main() {
    let instructions = read_input("inputs/02.txt");

    let n1 = part1(&instructions);
    println!("Part 1: Product of depth and position is {}", n1);

    let n2 = part2(&instructions);
    println!("Part 2: Product of depth and position is {}", n2);
}


#[cfg(test)]
mod tests02 {
    use super::*;

    #[test]
    fn test01() {
        let instructions = read_input("test_inputs/02_01.txt");
        let product = part1(&instructions);

        assert_eq!(product, 150);
    }

    #[test]
    fn test02() {
        let instructions = read_input("test_inputs/02_01.txt");
        let product = part2(&instructions);

        assert_eq!(product, 900);
    }
}
