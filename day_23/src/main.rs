use std::collections::HashMap;
// use std::str::FromStr;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum AmphipodColor {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Amphipod {
    point: (i8, i8),
    color: AmphipodColor,
}

struct Square {
    room: Option<AmphipodColor>,
}

type Grid = HashMap<(i8, i8), Square>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Position {
    cost: u64,
    ready: HashMap<(i8, i8), Amphipod>,
    waiting: HashMap<(i8, i8), Amphipod>,
    happy: HashMap<(i8, i8), Amphipod>,
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn build_world_example() -> Grid {
    let mut grid = Grid::new();
    for i in 0..=10 {
        grid.insert((0, i), Square { room: None });
    }
    for (i, j) in [(-1, 2), (-2, 2)] {
        grid.insert(
            (i, j),
            Square {
                room: Some(AmphipodColor::A),
            },
        );
    }

    for (i, j) in [(-1, 4), (-2, 4)] {
        grid.insert(
            (i, j),
            Square {
                room: Some(AmphipodColor::B),
            },
        );
    }

    for (i, j) in [(-1, 6), (-2, 6)] {
        grid.insert(
            (i, j),
            Square {
                room: Some(AmphipodColor::C),
            },
        );
    }

    for (i, j) in [(-1, 8), (-2, 8)] {
        grid.insert(
            (0, i),
            Square {
                room: Some(AmphipodColor::C),
            },
        );
    }
    grid
}

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

fn parse(input: &str) -> Position {
    let mut lines = input.trim().lines().into_iter();

    lines.next();
    lines.next();

    let mut ready = HashMap::new();
    for (i, line) in lines.enumerate() {
        let i = i as i8 * -1 + -1;
        for (j, c) in line.chars().enumerate() {
            let j = j as i8 + -1;
            let amphi = match c {
                'A' => Amphipod {
                    color: AmphipodColor::A,
                    point: (i, j),
                },
                'B' => Amphipod {
                    color: AmphipodColor::B,
                    point: (i, j),
                },
                'C' => Amphipod {
                    color: AmphipodColor::C,
                    point: (i, j),
                },
                'D' => Amphipod {
                    color: AmphipodColor::D,
                    point: (i, j),
                },
                '#' | ' ' => continue,
                _ => panic!("Nope {:?}", c),
            };
            ready.insert((i, j), amphi);
        }
    }
    Position {
        cost: 0,
        ready: ready,
        waiting: HashMap::new(),
        happy: HashMap::new(),
    }
}

fn color_to_str(color: &AmphipodColor) -> String {
    match color {
        &AmphipodColor::A => "A".to_string(),
        &AmphipodColor::B => "B".to_string(),
        &AmphipodColor::C => "C".to_string(),
        &AmphipodColor::D => "D".to_string(),
    }
}

fn print(grid: Grid, position: Position) {
    println!("");
    for i in (-2..=0).rev() {
        let mut line = Vec::new();
        for j in 0..10 {
            if let Some(a) = position.ready.get(&(i, j)) {
                line.push(color_to_str(&a.color));
            } else if let Some(a) = position.waiting.get(&(i, j)) {
                line.push(color_to_str(&a.color));
            } else {
                if let Some(s) = grid.get(&(i, j)) {
                    line.push(".".to_string());
                } else {
                    line.push("#".to_string())
                }
            }
        }
        println!("{:?}", line.join(""));
    }
}

fn play(grid: Grid, position: Position) -> u64 {
    let mut frontier = BinaryHeap::new();
    frontier.push(Reverse(position.clone()));

    if let Some(Reverse(position)) = frontier.pop() {
        if position.ready.is_empty() && position.waiting.is_empty() {
            return position.cost;
        }

        for (point, amphi) in &position.ready {
            let mut p = position.clone();
            let amphi = p.ready.remove(point).unwrap();

            if let Some(square) = grid.get(point) {
                if square.room.as_ref().unwrap() != &amphi.color {
                } else {
                    let below = (point.0 - 1, point.1);
                    match grid.get(&below) {
                        None => {
                            p.happy.insert(*point, amphi);
                            frontier.push(Reverse(p));
                        }
                        Some(s) => {
                            if let Some(a) = position.happy.get(&below) {
                                p.happy.insert(*point, amphi);
                                frontier.push(Reverse(p));
                            } else if let Some(a) = position.ready.get(&below) {
                                // Amphipod blocking, move into the hallway
                                for j in [-1, 1] {
                                    let mut next = p.clone();
                                    next.ready.insert((0, point.0 + j), amphi.clone());
                                    next.cost +=
                                        dist(*point, (0, point.0 + j)) * cost_color(&amphi.color);
                                    frontier.push(Reverse(next));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    0
}

fn cost_color(color: &AmphipodColor) -> u64 {
    match color {
        &AmphipodColor::A => 1,
        &AmphipodColor::B => 10,
        &AmphipodColor::C => 100,
        &AmphipodColor::D => 1000,
    }
}

fn dist(a: (i8, i8), b: (i8, i8)) -> u64 {
    ((a.0 - b.0).abs() + (a.0 - b.0).abs()).try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lol() {
        let grid = build_world_example();
        let position = parse(
            r#"
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
"#,
        );

        let cost = play(grid, position);
        assert_eq!(cost, 12521);
    }

    #[test]
    fn test_parts() {
        let input = r#""#;
        // assert_eq!(part_1(input), 99);
        // assert_eq!(part_2(input), 99);
    }
}
