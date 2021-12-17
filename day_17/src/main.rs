// use std::collections::HashMap;
// use std::str::FromStr;
use std::cmp::Ordering;
use std::ops::RangeInclusive;

fn main() {
    // target area: x=25..67, y=-260..-200
    let target = Target {
        x_range: 25..=67,
        y_range: -260..=-200,
    };
    let out = part_1(&target);
    println!("part_1: {}", out);

    let out = part_2(&target);
    println!("part_2: {}", out);
}

fn part_1(target: &Target) -> i32 {
    maximize_y(target, &(0, 0), &(0..=67), &(-260..=260))
}

fn part_2(target: &Target) -> u64 {
    guesses(target, &(0, 0), &(0..=67), &(-260..=260)).len() as u64
}

type Point = (i32, i32);
type Velocity = (i32, i32);

#[derive(Clone, Debug)]
struct Probe {
    location: Point,
    velocity: Velocity,
}

impl Probe {
    fn step(&mut self) {
        let (mut x, mut y) = self.location;
        let (mut x_vel, mut y_vel) = self.velocity;

        x += x_vel;
        y += y_vel;
        y_vel -= 1;
        match x_vel.cmp(&0) {
            Ordering::Less => x_vel += 1,
            Ordering::Equal => {}
            Ordering::Greater => x_vel -= 1,
        }

        self.location = (x, y);
        self.velocity = (x_vel, y_vel);
    }

    fn is_within(&self, target: &Target) -> bool {
        let (x, y) = self.location;
        target.x_range.contains(&x) && target.y_range.contains(&y)
    }

    fn is_beyond(&self, target: &Target) -> bool {
        let (_, y) = self.location;
        let (_, y_vel) = self.velocity;
        y < *target.min_y() && y_vel <= 1
    }
}

struct Target {
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

impl Target {
    fn min_y(&self) -> &i32 {
        let end_val = self.y_range.end();
        let start_val = self.y_range.start();
        if start_val < end_val {
            start_val
        } else {
            end_val
        }
    }
}

enum Shot {
    Hit(i32),
    Miss,
}

fn fire(probe: &mut Probe, target: &Target) -> Shot {
    let mut max_height = i32::MIN;
    while !probe.is_within(&target) {
        if probe.is_beyond(&target) {
            return Shot::Miss;
        }
        let height = probe.location.1;
        if height > max_height {
            max_height = height
        }
        probe.step();
    }

    Shot::Hit(max_height)
}

#[derive(Debug)]
struct Guess {
    // x_vel: i32,
    // y_vel: i32,
    height: i32,
}

fn guesses(
    target: &Target,
    start: &Point,
    x_range: &RangeInclusive<i32>,
    y_range: &RangeInclusive<i32>,
) -> Vec<Guess> {
    let mut guesses = Vec::new();
    let x_range = x_range.clone();
    for x_vel in x_range.into_iter() {
        let y_range = y_range.clone();
        for y_vel in y_range.into_iter() {
            let mut probe = Probe {
                location: *start,
                velocity: (x_vel, y_vel),
            };
            match fire(&mut probe, target) {
                Shot::Miss => {}
                Shot::Hit(height) => {
                    let guess = Guess {
                        // x_vel,
                        // y_vel,
                        height,
                    };
                    guesses.push(guess);
                }
            }
        }
    }
    guesses
}

fn maximize_y(
    target: &Target,
    start: &Point,
    x_range: &RangeInclusive<i32>,
    y_range: &RangeInclusive<i32>,
) -> i32 {
    let guesses = guesses(target, start, x_range, y_range);

    guesses
        .iter()
        .max_by(|a, b| a.height.cmp(&b.height))
        .unwrap()
        .height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blerg() {
        let target = Target {
            x_range: 20..=30,
            y_range: -10..=-5,
        };

        let x_range = 0..=30;
        let y_range = -10..=10;
        assert_eq!(maximize_y(&target, &(0, 0), &x_range, &y_range), 45);

        let guesses = guesses(&target, &(0, 0), &x_range, &y_range);
        assert_eq!(guesses.len(), 112);
    }

    #[test]
    fn test_physics() {
        let mut probe = Probe {
            location: (0, 0),
            velocity: (7, 2),
        };

        let target = Target {
            x_range: 20..=30,
            y_range: -10..=-5,
        };
        probe.step();
        assert_eq!(probe.location, (7, 2));
        assert!(!&probe.is_within(&target));
        assert!(!&probe.is_beyond(&target));

        probe.step();
        assert_eq!(probe.location, (13, 3));
        probe.step();
        assert_eq!(probe.location, (18, 3));

        probe.step();
        assert_eq!(probe.location, (22, 2));

        probe.step();
        assert_eq!(probe.location, (25, 0));

        probe.step();
        assert_eq!(probe.location, (27, -3));

        probe.step();
        assert_eq!(probe.location, (28, -7));
        assert!(&probe.is_within(&target));
        assert!(!&probe.is_beyond(&target));

        probe.step();
        assert!(&probe.is_beyond(&target));
    }

    #[test]
    fn test_check() {}
}
