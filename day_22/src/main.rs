use std::ops::RangeInclusive;

// use std::collections::HashMap;
// use std::str::FromStr;

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

fn count_one(x: RangeInclusive<i64>, y: RangeInclusive<i64>, z: RangeInclusive<i64>) -> i64 {
    (x.clone().count() * y.clone().count() * z.clone().count())
        .try_into()
        .unwrap()
}

use std::collections::HashSet;
type HyperGrid = HashSet<(i64, i64, i64)>;

fn naieve_on(
    grid: &mut HyperGrid,
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
    z: RangeInclusive<i64>,
) {
    for i in 0..=50 {
        for j in 0..=50 {
            for k in 0..=50 {
                if x.contains(&i) && y.contains(&j) && z.contains(&k) {
                    grid.insert((i, j, k));
                }
            }
        }
    }
}

fn naieve_off(
    grid: &mut HyperGrid,
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
    z: RangeInclusive<i64>,
) {
    for i in 0..=50 {
        for j in 0..=50 {
            for k in 0..=50 {
                if x.contains(&i) && y.contains(&j) && z.contains(&k) {
                    grid.remove(&(i, j, k));
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Command {
    tuple: (
        RangeInclusive<i64>,
        RangeInclusive<i64>,
        RangeInclusive<i64>,
    ),
    on: bool,
}

fn parse(input: &str) -> Vec<Command> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split(" ");
            let on = match parts.next() {
                Some("on") => true,
                Some("off") => false,
                _ => panic!("lol"),
            };

            let mut parts = parts.next().unwrap().split(",");
            let (x_str, y_str, z_str) = (
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            );

            Command {
                tuple: (
                    str_into_range(x_str),
                    str_into_range(y_str),
                    str_into_range(z_str),
                ),
                on: on,
            }
        })
        .collect::<Vec<Command>>()
}

fn str_into_range(input: &str) -> RangeInclusive<i64> {
    let mut parts = input.split("..");
    parts.next().unwrap().parse().unwrap()..=parts.next().unwrap().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lol() {
        let out = parse("on -12..41,-1..48,-27..19");
        assert_eq!(
            out,
            vec![Command {
                on: true,
                tuple: (-12..=41, -1..=48, -27..=19)
            }]
        );

        let out = parse("off 0..18,-9..2,-23..-8");
        assert_eq!(
            out,
            vec![Command {
                on: false,
                tuple: (0..=18, -9..=2, -23..=-8)
            }]
        );
    }

    #[test]
    fn test_count() {
        assert_eq!(count_one(10..=12, 10..=12, 10..=12), 27);
    }

    #[test]
    fn test_parts() {
        let input = r#""#;
        assert_eq!(part_1(input), 99);
        // assert_eq!(part_2(input), 99);
    }
}
