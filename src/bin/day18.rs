use std::env;
use std::error;
use std::fmt;
use std::fs;
use std::ops::Add;

use aoc2021::error::Error;

type Pair = (Box<Number>, Box<Number>);

#[derive(Debug, Default, Clone)]
struct Parser {
    source: String,
    current: usize,
    depth: usize,
    stack: Vec<Number>,
}

impl Parser {
    fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            current: 0,
            depth: 0,
            stack: Vec::new(),
        }
    }

    fn parse(&mut self) -> Result<Number, Box<dyn error::Error>> {
        while let Some(c) = self.advance() {
            match c {
                '[' => self.depth += 1,
                ']' => {
                    if self.depth == 0 {
                        return Err(Box::new(Error("Mismatched bracket in input.".to_string())));
                    }
                    self.depth -= 1;
                    let second = self.pop_stack()?;
                    let first = self.pop_stack()?;
                    self.stack
                        .push(Number::Pair((Box::new(first), Box::new(second))));
                }
                c if c.is_ascii_digit() => {
                    let value = self.parse_regular_number(c)?;
                    self.stack.push(value);
                }
                ',' => {}
                _ => {
                    return Err(Box::new(Error(format!(
                        "Unexpected character in input '{}'.",
                        c
                    ))));
                }
            }
        }

        let result = self
            .stack
            .pop()
            .ok_or_else(|| Error("Stack is empty.".to_string()))?;
        Ok(result)
    }

    fn parse_regular_number(&mut self, begin: char) -> Result<Number, Box<dyn error::Error>> {
        let mut string = String::from(begin);
        while let Some(c) = self.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            self.advance();
            string.push(c);
        }

        let value = string.parse::<usize>()?;
        Ok(Number::Regular(value))
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(c) = self.source[self.current..].chars().next() {
            self.current += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<char> {
        self.source[self.current..].chars().next()
    }

    fn pop_stack(&mut self) -> Result<Number, Error> {
        self.stack
            .pop()
            .ok_or_else(|| Error("Stack is empty.".to_string()))
    }
}

fn parse_line(line: &str) -> Result<Number, Box<dyn error::Error>> {
    let mut parser = Parser::new(line);
    parser.parse()
}

fn parse_input(data: &str) -> Result<Vec<Number>, Box<dyn error::Error>> {
    data.lines()
        .map(|s| parse_line(s))
        .collect::<Result<Vec<_>, _>>()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Number {
    Regular(usize),
    Pair(Pair),
}

impl Number {
    fn add_left(&mut self, value: usize) {
        match self {
            Number::Regular(n) => *n += value,
            Number::Pair((left, _)) => left.add_left(value),
        }
    }

    fn add_right(&mut self, value: usize) {
        match self {
            Number::Regular(n) => *n += value,
            Number::Pair((_, right)) => right.add_right(value),
        }
    }
}

impl Add for Box<Number> {
    type Output = Box<Number>;

    fn add(self, rhs: Self) -> Self::Output {
        Box::new(Number::Pair((self, rhs)))
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        Number::Pair((Box::new(self), Box::new(rhs)))
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Regular(n) => {
                write!(f, "{}", n)?;
            }
            Number::Pair((l, r)) => {
                write!(f, "[{},{}]", *l, *r)?;
            }
        }
        Ok(())
    }
}

trait NumberVisitor {
    type Output;

    fn visit(&mut self, number: &mut Number) -> Self::Output;
}

#[derive(Debug, Default, Copy, Clone)]
struct ExploderVisitor {
    depth: usize,
    had_explosion: bool,
}

impl ExploderVisitor {
    fn new() -> Self {
        Default::default()
    }
}

impl NumberVisitor for ExploderVisitor {
    type Output = Option<(Option<usize>, Option<usize>, usize)>;

    fn visit(&mut self, number: &mut Number) -> Self::Output {
        self.depth += 1;

        let ret = match number {
            Number::Pair((l, r)) => match (&**l, &**r) {
                (Number::Regular(m), Number::Regular(n)) => {
                    if self.depth >= 5 && !self.had_explosion {
                        self.had_explosion = true;
                        let ret = Some((Some(*m), Some(*n), self.depth));
                        *number = Number::Regular(0);
                        ret
                    } else {
                        None
                    }
                }
                _ => {
                    if let Some((m, n, d)) = self.visit(l) {
                        if let Some(n) = n {
                            r.add_left(n);
                            Some((m, None, d))
                        } else {
                            Some((m, n, d))
                        }
                    } else if let Some((m, n, d)) = self.visit(r) {
                        if let Some(m) = m {
                            l.add_right(m);
                            Some((None, n, d))
                        } else {
                            Some((m, n, d))
                        }
                    } else {
                        None
                    }
                }
            },
            Number::Regular(_) => None,
        };

        self.depth -= 1;

        ret
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct SplitterVisitor {}

impl SplitterVisitor {
    fn new() -> Self {
        Default::default()
    }
}

impl NumberVisitor for SplitterVisitor {
    type Output = bool;

    fn visit(&mut self, number: &mut Number) -> Self::Output {
        match number {
            Number::Regular(n) => {
                if *n > 9 {
                    let (l, r) = if *n % 2 == 0 {
                        (*n / 2, *n / 2)
                    } else {
                        (*n / 2, *n / 2 + 1)
                    };
                    *number =
                        Number::Pair((Box::new(Number::Regular(l)), Box::new(Number::Regular(r))));
                    true
                } else {
                    false
                }
            }
            Number::Pair((l, r)) => self.visit(l) || self.visit(r),
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct MagnitudeVisitor {}

impl MagnitudeVisitor {
    fn new() -> Self {
        Default::default()
    }
}

impl NumberVisitor for MagnitudeVisitor {
    type Output = usize;

    fn visit(&mut self, number: &mut Number) -> Self::Output {
        match number {
            Number::Regular(n) => *n,
            Number::Pair((l, r)) => {
                let value = self.visit(l) * 3 + self.visit(r) * 2;
                value
            }
        }
    }
}

fn sum(mut numbers: Vec<Number>) -> Result<Number, Box<dyn error::Error>> {
    numbers.reverse();

    let mut sum = numbers
        .pop()
        .ok_or_else(|| Error("No numbers available.".to_string()))?;

    while let Some(number) = numbers.pop() {
        sum = sum + number;
        reduce(&mut sum);
    }

    Ok(sum)
}

fn reduce(value: &mut Number) {
    loop {
        let mut visitor = ExploderVisitor::new();
        if visitor.visit(value).is_some() {
            continue;
        }

        let mut visitor = SplitterVisitor::new();
        if !visitor.visit(value) {
            break;
        }
    }
}

fn magnitude(value: &mut Number) -> usize {
    let mut visitor = MagnitudeVisitor::new();
    visitor.visit(value)
}

fn max_magnitude(numbers: Vec<Number>) -> Result<usize, Box<dyn error::Error>> {
    let mut max_magnitude = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }
            let sum_inputs = vec![numbers[i].clone(), numbers[j].clone()];
            let mut sum_value = sum(sum_inputs)?;
            let magnitude = magnitude(&mut sum_value);
            if magnitude > max_magnitude {
                max_magnitude = magnitude;
            }
        }
    }

    Ok(max_magnitude)
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

    let data = parse_input(&file_contents)?;

    let mut sum_value = sum(data.clone())?;
    let sum_magnitude = magnitude(&mut sum_value);
    println!("Part one: {}", sum_magnitude);

    println!("Part two: {}", max_magnitude(data)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_pair() {
        let src = "[1,2]";
        let mut parser = Parser::new(src);
        let expected = Number::Pair((Box::new(Number::Regular(1)), Box::new(Number::Regular(2))));
        let result = parser.parse().expect("Unable to parse input.");
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_nested_pair() {
        let src = "[[4,[6,7]],[[8,9],3]]";
        let mut parser = Parser::new(src);
        let expected = Number::Pair((
            Box::new(Number::Pair((
                Box::new(Number::Regular(4)),
                Box::new(Number::Pair((
                    Box::new(Number::Regular(6)),
                    Box::new(Number::Regular(7)),
                ))),
            ))),
            Box::new(Number::Pair((
                Box::new(Number::Pair((
                    Box::new(Number::Regular(8)),
                    Box::new(Number::Regular(9)),
                ))),
                Box::new(Number::Regular(3)),
            ))),
        ));
        let result = parser.parse().expect("Unable to parse input.");
        assert_eq!(expected, result);
    }

    #[test]
    fn test_number_add_regular() {
        let first = Box::new(Number::Regular(1));
        let second = Box::new(Number::Regular(2));
        let expected = Box::new(Number::Pair((first.clone(), second.clone())));
        let result = first + second;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_number_add_pair() {
        let first = Box::new(Number::Pair((
            Box::new(Number::Regular(1)),
            Box::new(Number::Regular(1)),
        )));
        let second = Box::new(Number::Pair((
            Box::new(Number::Regular(2)),
            Box::new(Number::Regular(2)),
        )));
        let expected = Box::new(Number::Pair((
            Box::new(Number::Pair((
                Box::new(Number::Regular(1)),
                Box::new(Number::Regular(1)),
            ))),
            Box::new(Number::Pair((
                Box::new(Number::Regular(2)),
                Box::new(Number::Regular(2)),
            ))),
        )));
        let result = first + second;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_exploder_traverser() {
        struct Case {
            input: String,
            output: String,
        }

        let cases = [
            Case {
                input: "[[[[[9,8],1],2],3],4]".to_string(),
                output: "[[[[0,9],2],3],4]".to_string(),
            },
            Case {
                input: "[7,[6,[5,[4,[3,2]]]]]".to_string(),
                output: "[7,[6,[5,[7,0]]]]".to_string(),
            },
            Case {
                input: "[[6,[5,[4,[3,2]]]],1]".to_string(),
                output: "[[6,[5,[7,0]]],3]".to_string(),
            },
            Case {
                input: "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".to_string(),
                output: "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".to_string(),
            },
            Case {
                input: "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".to_string(),
                output: "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".to_string(),
            },
            Case {
                input: "[[[[4,0],[5,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]"
                    .to_string(),
                output: "[[[[4,0],[5,4]],[[0,[7,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]"
                    .to_string(),
            },
        ];

        for case in &cases {
            let mut tree = parse_line(&case.input).expect("Unable to parse input.");
            let mut visitor = ExploderVisitor::new();
            visitor.visit(&mut tree);
            assert_eq!(case.output, format!("{}", tree));
        }
    }

    #[test]
    fn test_splitter_visitor() {
        let src = "[3,[11,12]]";
        let mut tree = parse_line(src).expect("Unable to parse input.");
        let expected = "[3,[[5,6],12]]";

        let mut visitor = SplitterVisitor::new();
        visitor.visit(&mut tree);

        assert_eq!(expected, format!("{}", tree));
    }

    #[test]
    fn test_magnitude_visitor() {
        let src = "[[1,2],[[3,4],5]]";
        let mut tree = parse_line(src).expect("Unable to parse input.");

        let mut visitor = MagnitudeVisitor::new();
        let magnitude = visitor.visit(&mut tree);

        assert_eq!(143, magnitude);

        let src = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]";
        let mut tree = parse_line(src).expect("Unable to parse input.");

        let mut visitor = MagnitudeVisitor::new();
        let magnitude = visitor.visit(&mut tree);

        assert_eq!(4140, magnitude);
    }

    #[test]
    fn test_reduce() {
        let src = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]";
        let mut value = parse_line(src).expect("Unable to parse input.");
        let expected = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";

        reduce(&mut value);
        assert_eq!(expected, format!("{}", value));

        let src = "[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]";
        let mut value = parse_line(src).expect("Unable to parse input.");
        let expected = "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]";

        reduce(&mut value);
        assert_eq!(expected, format!("{}", value));
    }
}
