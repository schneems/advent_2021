use std::collections::HashMap;
type Hyperhash = HashMap<Point, Octopus>;

fn main() {
    let out = part_1(include_str!("../input.txt"), 100);
    println!("part_1: {}", out);

    let out = part_2(include_str!("../input.txt"), 1000);
    println!("part_2: {}", out);
}

fn part_2(input: &str, steps: u32) -> u64 {
    let mut hash = build_hyperhash(input);
    let stop_at = hash.iter().count();
    for i in 1..=steps {
        if step(&mut hash) == stop_at as u32 {
            return i.into();
        }
    }
    panic!("Not found")
}

fn part_1(input: &str, steps: u32) -> u64 {
    let mut hash = build_hyperhash(input);
    let mut count = 0;
    for _ in 0..steps {
        count += step(&mut hash);
    }
    count.into()
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    i: i16,
    j: i16,
}

impl Point {
    fn neighbors(&self) -> Vec<Point> {
        let mut neighbors = Vec::with_capacity(8);
        for diff_i in [-1, 0, 1].iter() {
            for diff_j in [-1, 0, 1].iter() {
                if (*diff_i, *diff_j) != (0, 0) {
                    neighbors.push(Point {
                        i: self.i + diff_i,
                        j: self.j + diff_j,
                    })
                }
            }
        }
        neighbors
    }
}

fn step(grid: &mut Hyperhash) -> u32 {
    let mut flashed = Vec::new();
    for (point, octopus) in grid.iter_mut() {
        octopus.energy += 1;
        if octopus.energy > 9 {
            flashed.push(point.clone());
        }
    }

    let mut count = 0;
    while let Some(point) = flashed.pop() {
        if let Some(octopus) = grid.get_mut(&point) {
            match octopus.energy {
                0 => {} // Already flashed
                1..=8 => octopus.energy += 1,
                9..=u32::MAX => {
                    // Flashing
                    count += 1;
                    octopus.energy = 0;
                    for neighbor in point.neighbors() {
                        flashed.push(neighbor);
                    }
                }
            }
        }
    }

    count
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Octopus {
    point: Point,
    energy: u32,
}

fn build_hyperhash(input: &str) -> Hyperhash {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(move |(i, line)| {
            line.chars().enumerate().map(move |(j, char)| {
                let point = Point {
                    i: i.try_into().unwrap(),
                    j: j.try_into().unwrap(),
                };
                let octopus = Octopus {
                    point: point.clone(),
                    energy: char.to_string().parse::<u32>().unwrap(),
                };
                (point, octopus)
            })
        })
        .collect::<Hyperhash>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let input = r#"
11111
19991
19191
19991
11111
        "#;
        let mut hyperhash = build_hyperhash(input);
        assert_eq!(step(&mut hyperhash), 9);
        assert_eq!(step(&mut hyperhash), 0);
    }

    #[test]
    fn test_point() {
        let point = Point { i: 1, j: 1 };
        let neighbors = point.neighbors();
        assert_eq!(neighbors.len(), 8);
        assert_eq!(neighbors[0], Point { i: 0, j: 0 });
        assert_eq!(neighbors[7], Point { i: 2, j: 2 });
    }

    #[test]
    fn test_parts() {
        let input = r#"
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
      "#;
        assert_eq!(part_1(input, 10), 204);
        assert_eq!(part_1(input, 100), 1656);
        assert_eq!(part_2(input, 200), 195);
    }
}
