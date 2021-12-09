use std::env;
use std::fs;
use std::iter::FromIterator;

//   | a b c d e f g | 
// 2 |     1     1   | 0010010
// 3 | 7   7     7   | 1010010
// 4 |   4 4 4   4   | 0111010
// 5 | 2   2 2 2   2 | 1011101
// 5 | 3   3 3   3 3 | 1011011
// 5 | 5 5   5   5 5 | 1101011
// 6 | 6 6   6 6 6 6 | 1101111
// 6 | 0 0 0   0 0 0 | 1110111
// 6 | 9 9 9 9   9 9 | 1111011
// 7 | 8 8 8 8 8 8 8 | 1111111

fn sort_string(string: &str) -> String {
    let mut chars = string.chars().collect::<Vec<_>>();
    chars.sort();
    String::from_iter(chars)
}

fn find_segment_pos(
    mapping: &str,
    segments: &[String],
    char_count: usize,
) -> Result<usize, String> {
    segments
        .iter()
        .enumerate()
        .filter(|&(_, s)| mapping.chars().filter(|&c| s.contains(c)).count() == char_count)
        .map(|(i, _)| i)
        .next()
        .ok_or("Missing segment in input.".to_string())
}

fn extract_digit_map(line: &str) -> Result<Vec<String>, String> {
    let mut digit_descriptors: Vec<_> = line.split_whitespace().map(sort_string).collect();
    digit_descriptors.sort_by(|s1, s2| s1.len().cmp(&s2.len()));

    let mut mappings = vec!["".to_string(); 10];

    mappings[1] = digit_descriptors.remove(0);
    mappings[7] = digit_descriptors.remove(0);
    mappings[4] = digit_descriptors.remove(0);
    mappings[8] = digit_descriptors
        .pop()
        .ok_or("Not enough digits in input.")?;

    let mut five_digit_segments = digit_descriptors
        .iter()
        .filter(|s| s.len() == 5)
        .cloned()
        .collect::<Vec<_>>();

    let pos = find_segment_pos(&mappings[1], &five_digit_segments, 2)?;
    mappings[3] = five_digit_segments.remove(pos);
    let pos = find_segment_pos(&mappings[4], &five_digit_segments, 2)?;
    mappings[2] = five_digit_segments.remove(pos);
    mappings[5] = five_digit_segments[0].clone();

    let mut six_digit_segments = digit_descriptors
        .iter()
        .filter(|s| s.len() == 6)
        .cloned()
        .collect::<Vec<_>>();

    let pos = find_segment_pos(&mappings[1], &six_digit_segments, 1)?;
    mappings[6] = six_digit_segments.remove(pos);
    let pos = find_segment_pos(&mappings[4], &six_digit_segments, 3)?;
    mappings[0] = six_digit_segments.remove(pos);
    mappings[9] = six_digit_segments[0].clone();

    Ok(mappings)
}

fn process_line(line: &str) -> Result<usize, String> {
    let mut split_iter = line.split(" | ");
    let digit_descriptors = split_iter
        .next()
        .ok_or("Unable to parse input line.".to_string())?;
    let mapping = extract_digit_map(digit_descriptors)?;

    let digits_to_determine = split_iter
        .next()
        .ok_or("Unable to parse input line.".to_string())?
        .split_whitespace()
        .map(sort_string)
        .collect::<Vec<String>>();

    let mut result = 0_usize;

    for digit in digits_to_determine.iter() {
        let (i, _) = mapping
            .iter()
            .enumerate()
            .filter(|&(_, s)| s == digit)
            .next()
            .ok_or("Unable to translate digit.".to_string())?;
        result *= 10;
        result += i;
    }

    Ok(result)
}

fn part1(input: &str) -> Result<usize, String> {
    input
        .lines()
        .map(|l| -> Result<_, _> {
            Ok(l.split(" | ")
                .skip(1)
                .next()
                .ok_or("Unable to parse input data.".to_string())?
                .split_whitespace()
                .filter(|&s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
                .count())
        })
        .sum()
}

fn part2(input: &str) -> Result<usize, String> {
    let ints = input
        .lines()
        .map(process_line)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ints.iter().sum())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(format!("Usage: {} <input data path>", args[0]));
    }

    let path = &args[1];
    let file_contents =
        fs::read_to_string(path).map_err(|e| format!("Error reading input data: {}.", e))?;

    println!("Part one: {}", part1(&file_contents)?);
    println!("Part two: {}", part2(&file_contents)?);

    Ok(())
}
