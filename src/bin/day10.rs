use std::env;
use std::fs;

fn compute_line_score(line: &str) -> Result<(bool, usize, usize), String> {
    let mut stack = Vec::new();

    macro_rules! closing_case {
        ($open:expr, $value:expr) => {
            if let Some(p) = stack.pop() {
                if p != $open {
                    return Ok((false, $value, 0));
                }
            } else {
                return Ok((false, $value, 0));
            }
        }
    }

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => closing_case!('(', 3),
            ']' => closing_case!('[', 57),
            '}' => closing_case!('{', 1197),
            '>' => closing_case!('<', 25137),
            _ => return Err(format!("Unexpected character '{}'.", c)),
        }
    }

    if stack.len() == 0 {
        return Ok((false, 0, 0));
    }

    let mut completion_score = 0;

    for c in stack.iter().rev() {
        completion_score *= 5;
        match c {
            '(' => completion_score += 1,
            '[' => completion_score += 2,
            '{' => completion_score += 3,
            '<' => completion_score += 4,
            _ => unreachable!(),
        }
    }

    Ok((true, 0, completion_score))
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(format!("Usage: {} <input data path>", args[0]));
    }

    let path = &args[1];
    let file_contents =
        fs::read_to_string(path).map_err(|e| format!("Error reading input data: {}.", e))?;

    let mut corruption_score = 0;
    let mut completion_scores = Vec::new();

    for l in file_contents.lines() {
        let (incomplete, line_corruption_score, line_completion_score) = compute_line_score(l)?;
        corruption_score += line_corruption_score;
        if incomplete {
            completion_scores.push(line_completion_score);
        }
    }
    completion_scores.sort();
    let midpoint = completion_scores.len() / 2;
    let median_completion_score = completion_scores[midpoint];

    println!("Part one: {}", corruption_score);
    println!("Part two: {}", median_completion_score);

    Ok(())
}
