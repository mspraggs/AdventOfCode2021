use std::collections::{HashSet, VecDeque};
use std::env;
use std::error;
use std::fs;
use std::hash::Hash;
use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};
use std::slice::Iter;
use std::str::Lines;

use aoc2021::error::Error;

const NDIMS: usize = 3;

// Comments below refer to the sequence of rotations anti-clockwise around the relevant axis.
const ROTATIONS: [Matrix; 24] = [
    Matrix([1, 0, 0, 0, 0, -1, 0, 1, 0]),   // x
    Matrix([0, 0, 1, 0, 1, 0, -1, 0, 0]),   // y
    Matrix([0, -1, 0, 1, 0, 0, 0, 0, 1]),   // z
    Matrix([1, 0, 0, 0, -1, 0, 0, 0, -1]),  // xx
    Matrix([0, 1, 0, 0, 0, -1, -1, 0, 0]),  // xy
    Matrix([0, 0, 1, 1, 0, 0, 0, 1, 0]),    // xz
    Matrix([-1, 0, 0, 0, 1, 0, 0, 0, -1]),  // yy
    Matrix([0, -1, 0, 0, 0, 1, -1, 0, 0]),  // yz
    Matrix([0, -1, 0, 0, 0, -1, 1, 0, 0]),  // zx
    Matrix([-1, 0, 0, 0, -1, 0, 0, 0, 1]),  // zz
    Matrix([1, 0, 0, 0, 0, 1, 0, -1, 0]),   // xxx
    Matrix([0, 0, -1, 0, -1, 0, -1, 0, 0]), // xxy
    Matrix([0, 1, 0, 1, 0, 0, 0, 0, -1]),   // xxz
    Matrix([-1, 0, 0, 0, 0, -1, 0, -1, 0]), // xyy
    Matrix([0, 0, 1, 0, -1, 0, 1, 0, 0]),   // xzx
    Matrix([-1, 0, 0, 0, 0, 1, 0, 1, 0]),   // xzz
    Matrix([0, 0, -1, 0, 1, 0, 1, 0, 0]),   // yyy
    Matrix([0, -1, 0, -1, 0, 0, 0, 0, -1]), // yyz
    Matrix([0, 1, 0, -1, 0, 0, 0, 0, 1]),   // zzz
    Matrix([1, 0, 0, 0, 1, 0, 0, 0, 1]),    // xxxx
    Matrix([0, 0, -1, 1, 0, 0, 0, -1, 0]),  // xxxz
    Matrix([0, 1, 0, 0, 0, 1, 1, 0, 0]),    // xxzx
    Matrix([0, 0, 1, -1, 0, 0, 0, -1, 0]),  // xyyz
    Matrix([0, 0, -1, -1, 0, 0, 0, 1, 0]),  // xzzz
];

fn parse_point(line: &str) -> Result<Point, Box<dyn error::Error>> {
    let mut point = Point([0; 3]);

    for (i, s) in line.split(',').enumerate() {
        if i >= 3 {
            return Err(Box::new(Error(
                "Extra input found when parsing point.".to_string(),
            )));
        }
        point[i] = s.parse::<i32>()?;
    }

    Ok(point)
}

fn parse_scanner(line_iter: &mut Lines) -> Result<Scanner, Box<dyn error::Error>> {
    let mut points = Vec::new();

    while let Some(line) = line_iter.next() {
        if line.is_empty() {
            break;
        }
        points.push(parse_point(line)?);
    }

    Ok(Scanner::new(points))
}

fn parse_input(data: &str) -> Result<Vec<Scanner>, Box<dyn error::Error>> {
    let mut line_iter = data.lines();
    let mut scanners = Vec::new();

    while let Some(line) = line_iter.next() {
        if line.starts_with("---") {
            scanners.push(parse_scanner(&mut line_iter)?);
        }
    }

    Ok(scanners)
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Point([i32; NDIMS]);

impl Point {
    fn iter(&self) -> Iter<'_, i32> {
        self.0.iter()
    }
}

impl Index<usize> for Point {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Point {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = Point::default();
        for i in 0..NDIMS {
            ret[i] = self[i] + rhs[i];
        }
        ret
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut ret = Point::default();
        for i in 0..NDIMS {
            ret[i] = self[i] - rhs[i];
        }
        ret
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        let mut ret = Point::default();
        for i in 0..NDIMS {
            ret[i] = -self[i];
        }
        ret
    }
}

impl Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Matrix([i32; NDIMS * NDIMS]);

impl Index<[usize; 2]> for Matrix {
    type Output = i32;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.0[index[0] * NDIMS + index[1]]
    }
}

impl IndexMut<[usize; 2]> for Matrix {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.0[index[0] * NDIMS + index[1]]
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut result = Matrix::default();

        for i in 0..NDIMS {
            for j in 0..NDIMS {
                for k in 0..NDIMS {
                    result[[i, k]] += self[[i, j]] * rhs[[j, k]];
                }
            }
        }

        result
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        let mut result = Point::default();

        for i in 0..NDIMS {
            for j in 0..NDIMS {
                result[i] += self[[i, j]] * rhs[j];
            }
        }

        result
    }
}

#[derive(Debug, Default, Clone)]
struct Scanner {
    points: Vec<Point>,
}

impl Scanner {
    fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    fn match_beacons(&self, other: &Scanner) -> Option<(Scanner, Point)> {
        for rotation in ROTATIONS.iter() {
            let rotated_scanner = other.rotate(rotation);

            for &point in self.points.iter() {
                for &transformed_point in rotated_scanner.points.iter() {
                    let offset = transformed_point - point;
                    let num_overlapping_beacons = self
                        .points
                        .iter()
                        .filter(|&&p| rotated_scanner.points.contains(&(p + offset)))
                        .copied()
                        .count();

                    if num_overlapping_beacons >= 12 {
                        return Some((rotated_scanner.translate(&-offset), offset));
                    }
                }
            }
        }

        None
    }

    fn rotate(&self, rotation: &Matrix) -> Self {
        Self::new(self.points.iter().map(|&p| *rotation * p).collect())
    }

    fn translate(&self, offset: &Point) -> Self {
        Self::new(self.points.iter().map(|&p| p + *offset).collect())
    }
}

fn determine_unique_beacons(scanners: &[Scanner]) -> (Vec<Point>, Vec<Point>) {
    let mut beacons = HashSet::new();
    let mut offsets = HashSet::new();

    let mut scanner_queue = VecDeque::new();
    scanner_queue.push_back(scanners[0].clone());

    let mut scanners = scanners[1..].to_owned();

    while let Some(scanner) = scanner_queue.pop_front() {
        scanner.points.iter().for_each(|&p| {
            beacons.insert(p);
        });

        let mut to_remove = Vec::new();
        for (i, candidate_scanner) in scanners.iter().enumerate() {
            if let Some((transformed_scanner, offset)) = scanner.match_beacons(candidate_scanner) {
                to_remove.push(i);
                scanner_queue.push_back(transformed_scanner);
                offsets.insert(offset);
            }
        }

        while let Some(idx) = to_remove.pop() {
            scanners.remove(idx);
        }
    }

    (
        beacons.iter().copied().collect(),
        offsets.iter().copied().collect(),
    )
}

fn compute_manhattan_distance(first: &Point, second: &Point) -> i32 {
    let diff = *first - *second;
    diff.iter().map(|n| n.abs()).sum()
}

fn compute_max_manhattan_distance(offsets: &[Point]) -> i32 {
    let mut max = 0;
    for (i, first) in offsets.iter().enumerate() {
        for second in offsets.iter().skip(i + 1) {
            let manhattan_distance = compute_manhattan_distance(first, second);
            if manhattan_distance > max {
                max = manhattan_distance;
            }
        }
    }
    max
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

    let scanners = parse_input(&file_contents)?;

    let (beacons, offsets) = determine_unique_beacons(&scanners);

    println!("Part one: {}", beacons.len());
    println!("Part two: {}", compute_max_manhattan_distance(&offsets));

    Ok(())
}
