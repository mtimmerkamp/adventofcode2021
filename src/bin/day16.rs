use std::collections::LinkedList;
use std::fmt::{Debug, Write};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Tape {
    data: LinkedList<bool>,
}

impl Tape {
    // fn new(data: &Vec<bool>) -> Tape {
    //     Tape {
    //         data: LinkedList::from_iter(data.iter().cloned()),
    //     }
    // }

    fn from_u8s(numbers: Vec<u8>, bits: u8) -> Tape {
        let mut data = LinkedList::new();
        for value in numbers {
            for i in 0..bits {
                let bit = bits - 1 - i;
                data.push_back((value >> bit) & 1 == 1)
            }
        }

        Tape { data }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn read_bit(&mut self) -> bool {
        if self.data.is_empty() {
            panic!("Trying to read from empty tape.")
        }
        self.data.pop_front().unwrap()
    }

    fn read_number(&mut self, bits: u8) -> u64 {
        if bits > 64 {
            panic!("Attempting to read value with more than 64 bits.");
        }

        let mut value = 0;
        for _bit in 0..bits {
            value <<= 1;
            if self.read_bit() {
                value += 1;
            }
        }

        value
    }

    fn read_u4(&mut self) -> u8 {
        self.read_number(4) as u8
    }

    fn read_literal(&mut self) -> Content {
        let mut last = false;
        let mut value = 0;
        while !last {
            last = !self.read_bit();
            value <<= 4;
            value += self.read_u4() as u64;
        }

        Content::Literal(value)
    }

    fn read_subpackets(&mut self) -> Vec<Packet> {
        let mut subpackets: Vec<Packet> = Vec::new();
        let type_length_id = self.read_bit();

        if !type_length_id {
            // i.e. type_length_id == 0
            let total_subpacket_length = self.read_number(15);
            let mut subpackets_tape = self.consume(total_subpacket_length as usize);

            while !subpackets_tape.is_empty() {
                subpackets.push(Packet::parse(&mut subpackets_tape));
            }
        } else {
            // i.e., type_length_id == 1
            let subpacket_count = self.read_number(11);
            for _i in 0..subpacket_count {
                subpackets.push(Packet::parse(self));
            }
        }

        subpackets
    }

    fn consume(&mut self, len: usize) -> Tape {
        if len > self.data.len() {
            panic!("Trying to read more values than available!");
        }
        let mut new = LinkedList::new();
        for _i in 0..len {
            new.push_back(self.data.pop_front().unwrap());
        }

        Tape { data: new }
    }
}

impl Debug for Tape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in self.data.iter() {
            f.write_char(if *v { '1' } else { '0' })?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Content {
    Literal(u64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Maximum(Vec<Packet>),
    Minimum(Vec<Packet>),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    content: Content,
}

impl Packet {
    fn parse_str(s: &str) -> Packet {
        let half_bits: Vec<u8> = s
            .chars()
            .map(|c| u8::from_str_radix(&String::from(c), 16).unwrap())
            .collect();
        let mut tape = Tape::from_u8s(half_bits, 4);

        Packet::parse(&mut tape)
    }

    fn parse(tape: &mut Tape) -> Packet {
        let version = tape.read_number(3) as u8;
        let type_id = tape.read_number(3) as u8;

        let content = match type_id {
            4 => tape.read_literal(),
            0 => Content::Sum(tape.read_subpackets()),
            1 => Content::Product(tape.read_subpackets()),
            2 => Content::Minimum(tape.read_subpackets()),
            3 => Content::Maximum(tape.read_subpackets()),
            5 => Content::GreaterThan(tape.read_subpackets()),
            6 => Content::LessThan(tape.read_subpackets()),
            7 => Content::EqualTo(tape.read_subpackets()),
            _ => panic!("Unknown type_id {}", type_id),
        };

        Packet {
            version,
            content,
        }
    }

    fn interpret(&self) -> u64 {
        fn evaluate(packet: &Packet) -> u64 {
            packet.interpret()
        }
        match &self.content {
            Content::Literal(v) => *v,
            Content::Sum(summands) => summands.iter().map(evaluate).sum(),
            Content::Product(factors) => factors.iter().map(evaluate).product(),
            Content::Minimum(values) => values.iter().map(evaluate).min().unwrap(),
            Content::Maximum(values) => values.iter().map(evaluate).max().unwrap(),
            // Specification states that >, <, and == always have two elements. The following is only valid in this exact case.
            Content::GreaterThan(values) => values.iter().map(evaluate).reduce(|prev, curr| if prev > curr {1} else {0}).unwrap(),
            Content::LessThan(values) => values.iter().map(evaluate).reduce(|prev, curr| if prev < curr {1} else {0}).unwrap(),
            Content::EqualTo(values) => values.iter().map(evaluate).reduce(|prev, curr| if prev == curr {1} else {0}).unwrap(),
        }
    }
}

fn read_input(filename: &str) -> Packet {
    let file = File::open(filename).expect(&format!("Cannot open file {}", filename));
    let reader = BufReader::new(file);

    if let Some(Ok(line)) = reader.lines().next() {
        Packet::parse_str(&line)
    } else {
        panic!("Cannot read packet.");
    }
}

fn part1(transmission: &Packet) -> u64 {
    fn sum_version_numbers(p: &Packet) -> u64 {
        p.version as u64
            + match &p.content {
                Content::Literal(_) => 0,
                Content::Sum(subpackets)
                | Content::Product(subpackets)
                | Content::Maximum(subpackets)
                | Content::Minimum(subpackets)
                | Content::GreaterThan(subpackets)
                | Content::LessThan(subpackets)
                | Content::EqualTo(subpackets) => subpackets.iter().map(sum_version_numbers).sum(),
            }
    }

    sum_version_numbers(transmission)
}

fn part2(transmission: &Packet) -> u64 {
    transmission.interpret()
}

fn main() {
    let packet = read_input("inputs/16.txt");

    println!("Part 1: {}", part1(&packet));
    println!("Part 2: {}", part2(&packet));
}

#[cfg(test)]
mod tests16 {
    use super::*;

    #[test]
    fn test01() {
        for (s, version_sum) in [
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ] {
            let packet = Packet::parse_str(s);
            assert_eq!(part1(&packet), version_sum);
        }
    }

    #[test]
    fn test02() {
        for (s, solution) in [
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ] {
            let packet = Packet::parse_str(s);
            assert_eq!(part2(&packet), solution);
        }
    }
}
