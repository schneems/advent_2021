// use std::collections::HashMap;
// use std::str::FromStr;
use std::cmp::Ordering;
use std::ops::RangeInclusive;

fn main() {
    let out = part_1(include_str!("../input.txt"));
    println!("part_1: {}", out);

    let out = part_2(include_str!("../input.txt"));
    println!("part_2: {}", out);
}

fn part_1(input: &str) -> u64 {
    let _thing = parse(input);
    unimplemented!()
}

fn part_2(input: &str) -> u64 {
    let _thing = parse(input);
    unimplemented!()
}

fn parse(input: &str) {
    unimplemented!()
}

type Point = (i32, i32);
type Velocity = (i32, i32);

struct Probe {
    location: Point,
    velocity: Velocity,
}

struct Target {
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

impl Target {
    fn is_within(&self, point: Point) -> bool {
        let (x, y) = point;
        self.x_range.contains(&x) && self.y_range.contains(&y)
    }

    fn is_beyond(&self, point: Point) -> bool {
        let (_, y) = point;
        y < *self.y_range.start()
    }
}

fn step(probe: &Probe, count: u32) -> Point {
    let Probe {
        location: (mut x, mut y),
        velocity: (mut x_vel, mut y_vel),
    } = probe;

    for _ in 0..count {
        x += x_vel;
        y += y_vel;
        y_vel -= 1;
        match x_vel.cmp(&0) {
            Ordering::Less => x_vel += 1,
            Ordering::Equal => {}
            Ordering::Greater => x_vel -= 1,
        };
    }
    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics() {
        let probe = Probe {
            location: (0, 0),
            velocity: (7, 2),
        };

        let target = Target {
            x_range: 20..=30,
            y_range: -10..=-5,
        };
        assert_eq!(step(&probe, 1), (7, 2));
        assert!(!&target.is_within((7, 2)));
        assert!(!&target.is_beyond((7, 2)));

        assert_eq!(step(&probe, 2), (13, 3));
        assert_eq!(step(&probe, 3), (18, 3));
        assert_eq!(step(&probe, 4), (22, 2));
        assert_eq!(step(&probe, 5), (25, 0));
        assert_eq!(step(&probe, 6), (27, -3));
        assert_eq!(step(&probe, 7), (28, -7));
        assert!(&target.is_within((28, -7)));
        assert!(!&target.is_beyond((28, -7)));
        assert!(&target.is_beyond(step(&probe, 8)));

        let probe = Probe {
            location: (0, 0),
            velocity: (9, 0),
        };
        assert_eq!(step(&probe, 1), (9, 0));
        assert_eq!(step(&probe, 2), (17, -1));
        assert_eq!(step(&probe, 3), (24, -3));
        assert_eq!(step(&probe, 4), (30, -6));
    }

    #[test]
    fn test_check() {}

    #[test]
    fn test_parts() {
        let _input = r#""#;
        // assert_eq!(part_1(input), 99);
        // assert_eq!(part_2(input), 99);
    }
}
