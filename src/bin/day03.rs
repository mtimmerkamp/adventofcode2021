use std::fs::File;
use std::io::{BufReader, BufRead};


fn read_input(filename: &str) -> (Vec<u32>, usize) {
    let mut input = Vec::new();

    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);
    let mut bits = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            if bits == 0 {
                bits = line.len();
            }
            let v = u32::from_str_radix(&line, 2).unwrap();
            input.push(v);
        }
    }

    (input, bits)
}

fn count_set_bit(values: &Vec<u32>, bit: usize) -> usize {
    values.iter().filter(|v| (*v >> bit) & 1 == 1).count()
}

fn part1(input: &Vec<u32>, bits: usize) -> usize {
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    for i in 0..bits {
        if count_set_bit(input, i) > input.len() / 2 {
            gamma_rate |= 1 << i;
        }
        else {
            epsilon_rate |= 1 << i;
        }
    }

    gamma_rate * epsilon_rate
}

fn part2(input: &Vec<u32>, bits: usize) -> u32 {
    let mut values = input.clone();
    let mut bit = bits - 1;
    while values.len() > 1 {
        let bc = count_set_bit(&values, bit);
        let most_common_bit = if bc * 2 >= values.len() {1} else {0};
        // println!("{}: {} {}, {:?}", bit, bc, most_common_bit, values);
        values = values.iter().filter(|v| (*v >> bit) & 1 == most_common_bit).cloned().collect();
        if bit > 0 {
            bit -= 1;
        }
    }
    // println!("{:?}", values);
    if values.len() > 1 {
        panic!("More than one value left!");
    }
    let oxygen = values.get(0).unwrap();
    // println!("{}", oxygen);


    let mut values = input.clone();
    let mut bit = bits - 1;
    while values.len() > 1 {
        let least_common_bit = if count_set_bit(&values, bit) * 2 >= values.len() {0} else {1};
        // println!("{}: {}, {:?}", bit, least_common_bit, values);
        values = values.iter().filter(|v| (*v >> bit) & 1 == least_common_bit).cloned().collect();
        if bit > 0 {
            bit -= 1;
        }
        else {
            break;
        }
    }
    // println!("{:?}", values);
    if values.len() > 1 {
        panic!("More than one value left!");
    }
    let co2 = values.get(0).unwrap();
    
    oxygen * co2
}

fn main() {
    // let input = read_input("inputs/02.txt");
    let (input, bits) = read_input("inputs/03.txt");

    let solution1 = part1(&input, bits);
    println!("Part 1: Product of gamma and epsilon rate: {}", solution1);

    let solution2 = part2(&input, bits);
    println!("Part 2: Product of oxygem generator and CO2 scrubber rating: {}", solution2);
}


#[cfg(test)]
mod tests03 {
    use super::*;

    #[test]
    fn test01() {
        let (input, bits) = read_input("test_inputs/03_01.txt");
        let solution = part1(&input, bits);

        assert_eq!(solution, 198);
    }

    #[test]
    fn test02() {
        let (input, bits) = read_input("inputs/03.txt");
        let solution = part1(&input, bits);

        assert_eq!(solution, 845186);
    }

    #[test]
    fn test03() {
        let (input, bits) = read_input("test_inputs/03_01.txt");
        let solution = part2(&input, bits);

        assert_eq!(solution, 230);
    }
}
