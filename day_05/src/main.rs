// Day 5: Hydrothermal Venture
// https://adventofcode.com/2021/day/5

#![warn(unused_crate_dependencies)]
#![warn(clippy::pedantic)]

use std::cmp::{self, Ordering};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::path::Path;
use std::process;
use std::str::FromStr;

use counter::Counter;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("One argument required - the input file path!");
        process::exit(1);
    });
    let vent_lines = read_vent_lines(&path);
    println!(
        "Part 1: Total points with intersections (excluding diagonals) = {}",
        total_points_with_intersections(&vent_lines, false)
    );
    println!(
        "Part 2: Total points with intersections (including diagonals) = {}",
        total_points_with_intersections(&vent_lines, true)
    );
}

fn read_vent_lines(path: impl AsRef<Path>) -> Vec<Line> {
    let f = File::open(path).expect("Error opening input file");
    BufReader::new(f)
        .lines()
        .map(|line| {
            line.expect("Error reading line")
                .parse::<Line>()
                .expect("Error parsing line")
        })
        .collect()
}

fn total_points_with_intersections(vent_lines: &[Line], include_diagonals: bool) -> usize {
    let all_line_points = vent_lines
        .iter()
        .filter(|line| include_diagonals || !line.is_diagonal())
        .flat_map(Line::points_on_line);
    let occurrences_of_each_point = all_line_points.collect::<Counter<_>>();
    // If a point was seen more than once, more than one line must pass through it, meaning a line intersection there.
    let points_with_intersections = occurrences_of_each_point
        .values()
        .filter(|&count| *count > 1);
    points_with_intersections.count()
}

#[derive(PartialEq, Eq, Hash)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_diagonal(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }

    fn points_on_line(&self) -> impl Iterator<Item = Point> + '_ {
        // Generate the list of points that exist on the line. For example, for a line with
        // start (0,0) and end (3,3), this returns a Vec with elements:
        // [(0,0), (1,1), (2,2), (3,3)]
        // Only supports lines that are vertical, horizontal or at a 45 degree angle.
        let total_points = 1 + cmp::max(
            (self.end.x - self.start.x).abs(),
            (self.end.y - self.start.y).abs(),
        );
        let x_step = match self.end.x.cmp(&self.start.x) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };
        let y_step = match self.end.y.cmp(&self.start.y) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };
        (0..total_points).map(move |num| Point {
            x: self.start.x + num * x_step,
            y: self.start.y + num * y_step,
        })
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Example input: "0,9 -> 5,9"
        let (start, end) = s.split_once(" -> ").unwrap_or_default();
        Ok(Self {
            start: start.parse::<Point>()?,
            end: end.parse::<Point>()?,
        })
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Example input: "0,9"
        let (x, y) = s.split_once(',').unwrap_or_default();
        Ok(Self {
            x: x.parse::<i64>()?,
            y: y.parse::<i64>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    fn example_file() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("example.txt")
    }

    #[test]
    fn part_one_example() {
        let vent_lines = read_vent_lines(&example_file());
        assert_eq!(total_points_with_intersections(&vent_lines, false), 5);
    }

    #[test]
    fn part_two_example() {
        let vent_lines = read_vent_lines(&example_file());
        assert_eq!(total_points_with_intersections(&vent_lines, true), 12);
    }
}
