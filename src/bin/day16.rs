use std::env;
use std::error;
use std::fs;

use aoc2021::error::Error;

#[derive(Debug, Default, Clone)]
struct Parser {
    data: Vec<u8>,
    current: usize,
    version_sum: usize,
    stack: Vec<usize>,
}

impl Parser {
    fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            current: 0,
            version_sum: 0,
            stack: Vec::new(),
        }
    }

    fn parse(&mut self) -> usize {
        self.parse_packet();
        self.stack.last().copied().expect("Stack is empty.")
    }

    fn parse_packet(&mut self) {
        let version = self.parse_bits(3);
        self.version_sum += version;
        let packet_type = self.parse_bits(3);

        if packet_type == 4 {
            let literal = self.parse_literal();
            self.stack.push(literal);
        } else {
            let stack_size = self.stack.len();
            let length_type = self.advance().expect("Insufficient bits found for packet.");
            if length_type {
                let num_packets = self.parse_bits(11);
                for _ in 0..num_packets {
                    self.parse_packet();
                }
            } else {
                let length = self.parse_bits(15);
                let current_bits = self.current;
                let target_bits = current_bits + length;

                while self.current < target_bits {
                    self.parse_packet();
                }
            }

            macro_rules! cumulative_op {
                ($op:expr) => {{
                    let mut acc = self.stack.pop().expect("Stack is empty.");
                    while self.stack.len() > stack_size {
                        acc = $op(acc, self.stack.pop().expect("Stack is empty."));
                    }
                    self.stack.push(acc);
                }};
            }

            macro_rules! binary_op {
                ($op:expr) => {{
                    let second = self.stack.pop().expect("Stack is empty.");
                    let first = self.stack.pop().expect("Stack is empty.");
                    self.stack.push($op(first, second));
                }};
            }

            match packet_type {
                0 => cumulative_op!(|a, b| a + b),
                1 => cumulative_op!(|a, b| a * b),
                2 => cumulative_op!(|a, b| {
                    if a < b {
                        a
                    } else {
                        b
                    }
                }),
                3 => cumulative_op!(|a, b| {
                    if a > b {
                        a
                    } else {
                        b
                    }
                }),
                5 => binary_op!(|a, b| if a > b { 1 } else { 0 }),
                6 => binary_op!(|a, b| if a < b { 1 } else { 0 }),
                7 => binary_op!(|a, b| if a == b { 1 } else { 0 }),
                _ => panic!("Unknown packet type."),
            }
            assert_eq!(stack_size + 1, self.stack.len());
        }
    }

    fn parse_literal(&mut self) -> usize {
        let mut bit_groups = Vec::new();

        loop {
            let group = self.parse_bits(5);
            bit_groups.push(group);
            if (group & 0b10000) == 0 {
                break;
            }
        }

        let mut value = 0;
        for group in bit_groups.iter() {
            value <<= 4;
            value |= *group & 0b1111;
        }

        value
    }

    fn parse_bits(&mut self, num_bits: u32) -> usize {
        assert!(num_bits <= usize::BITS);
        let mut value = 0;

        for i in (0..num_bits).rev() {
            if self.advance().expect("Insufficient bits found for value.") {
                value |= 1 << i;
            }
        }

        value
    }

    fn current_bit(&self) -> bool {
        let byte = self.current / 8;
        let bit = self.current % 8;
        (self.data[byte] & (0b10000000 >> bit)) != 0
    }

    fn advance(&mut self) -> Option<bool> {
        if self.current >= self.data.len() * 8 {
            return None;
        }
        let current_bit = self.current_bit();
        self.current += 1;
        Some(current_bit)
    }
}

fn print_bits(data: &[u8]) {
    for d in data {
        let s = &format!("{:#010b}", d)[2..];
        print!("{} ", s)
    }
    println!()
}

fn parse_input(data: &str) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let mut parsed_hex = Vec::new();
    for i in (0..data.len()).step_by(2) {
        if !data.is_char_boundary(i + 2) {
            return Err(Box::new(Error("Unable to parse input.".to_string())));
        }

        let hex_string = &data[i..i + 2];
        let value = u8::from_str_radix(hex_string, 16)?;
        parsed_hex.push(value);
    }

    Ok(parsed_hex)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Box::new(Error(format!(
            "Usage: {} <input data path>",
            args[0]
        ))));
    }

    let path = &args[1];
    let file_contents =
        fs::read_to_string(path).map_err(|e| format!("Error reading input data: {}.", e))?;

    let bytes = parse_input(&file_contents)?;
    if cfg!(debug_assertions) {
        print_bits(&bytes);
    }

    let mut parser = Parser::new(bytes);
    let result = parser.parse();

    println!("Part one: {}", parser.version_sum);
    println!("Part two: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let src = "38F6";
        let bytes = parse_input(src).unwrap();
        assert_eq!(vec![0x38, 0xf6], bytes);
    }

    #[test]
    fn test_parse_header() {
        let bytes = vec![56];
        let mut parser = Parser::new(bytes);
        let version = parser.parse_bits(3);
        let type_ = parser.parse_bits(3);

        assert_eq!(1, version);
        assert_eq!(6, type_);
    }

    #[test]
    fn test_parse_packet_literal() {
        let bytes = vec![0xd2, 0xfe, 0x28];
        let mut parser = Parser::new(bytes);
        let literal = parser.parse();
        assert_eq!(2021, literal);
        assert_eq!(1, parser.stack.len());
    }

    #[test]
    fn test_parse_packet_operator_sum() {
        let bytes = vec![0xc2, 0x00, 0xb4, 0x0a, 0x82];
        let mut parser = Parser::new(bytes);
        let result = parser.parse();
        assert_eq!(3, result);
        assert_eq!(1, parser.stack.len());
    }

    #[test]
    fn test_parse_packet_operator_product() {
        let bytes = vec![0x4, 0x0, 0x5a, 0xc3, 0x38, 0x90];
        let mut parser = Parser::new(bytes);
        let result = parser.parse();
        assert_eq!(54, result);
        assert_eq!(1, parser.stack.len());
    }

    #[test]
    fn test_parse_packet_operator_min() {
        let bytes = vec![0x88, 0x0, 0x86, 0xc3, 0xe8, 0x81, 0x12];
        let mut parser = Parser::new(bytes);
        let result = parser.parse();
        assert_eq!(7, result);
        assert_eq!(1, parser.stack.len());
    }

    #[test]
    fn test_parse_packet_operator_max() {
        let bytes = vec![0xce, 0x0, 0xc4, 0x3d, 0x88, 0x11, 0x20];
        let mut parser = Parser::new(bytes);
        let result = parser.parse();
        assert_eq!(9, result);
        assert_eq!(1, parser.stack.len());
    }

    #[test]
    fn test_parse_packet_operator_less() {
        let bytes = vec![0xd8, 0x0, 0x5a, 0xc2, 0xa8, 0xf0];
        let mut parser = Parser::new(bytes);
        let result = parser.parse();
        assert_eq!(1, result);
        assert_eq!(1, parser.stack.len());
    }

    #[test]
    fn test_parse_packet_operator_greater() {
        let bytes = vec![0xf6, 0x0, 0xbc, 0x2d, 0x8f];
        let mut parser = Parser::new(bytes);
        let result = parser.parse();
        assert_eq!(0, result);
        assert_eq!(1, parser.stack.len());
    }

    #[test]
    fn test_parse_packet_operator_equal() {
        let bytes = vec![0x9c, 0x0, 0x5a, 0xc2, 0xf8, 0xf0];
        let mut parser = Parser::new(bytes);
        let result = parser.parse();
        assert_eq!(0, result);
        assert_eq!(1, parser.stack.len());
    }

    #[test]
    fn test_parse_packet_operator_composite() {
        let bytes = vec![
            0x9c, 0x1, 0x41, 0x8, 0x2, 0x50, 0x32, 0xf, 0x18, 0x2, 0x10, 0x4a, 0x8,
        ];
        let mut parser = Parser::new(bytes);
        let result = parser.parse();
        assert_eq!(1, result);
        assert_eq!(1, parser.stack.len());
    }
}
