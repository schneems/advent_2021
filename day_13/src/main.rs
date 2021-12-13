fn main() {
    part_1(include_str!("../input.txt"));
}

fn part_1(input: &str) -> u64 {
    let mut paper = parse(input);
    apply(&mut paper);
    paper.grid.len().try_into().unwrap()
}

fn apply(paper: &mut Paper) {
    for instruction in &paper.instructions {
        let mut hash = HashMap::new();
        for (mut point, _) in paper.grid.drain() {
            match instruction {
                Instruction::FoldX(x_axis) => {
                    if point.x > *x_axis {
                        point.x = point.x - 2 * (point.x - *x_axis);
                    }
                }
                Instruction::FoldY(y_axis) => {
                    if point.y > *y_axis {
                        point.y = point.y - 2 * (point.y - *y_axis);
                    }
                }
            };
            hash.insert(point, true);
        }
        match instruction {
            Instruction::FoldX(x_axis) => {
                paper.max_x = *x_axis;
            }
            Instruction::FoldY(y_axis) => {
                paper.max_y = *y_axis;
            }
        };

        paper.grid = hash;
        debug(&paper);
        println!("Dots: {}", paper.grid.len())
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum Instruction {
    FoldX(i32),
    FoldY(i32),
}

struct Paper {
    grid: HashMap<Point, bool>,
    instructions: Vec<Instruction>,
    max_x: i32,
    max_y: i32,
}

fn parse(input: &str) -> Paper {
    let mut lines = input.trim().lines().into_iter();
    let mut grid = HashMap::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        grid.insert(Point::from_str(line).unwrap(), true);
    }

    let mut instructions = Vec::new();
    while let Some(line) = lines.next() {
        instructions.push(Instruction::from_str(line).unwrap())
    }

    let max_y = grid
        .iter()
        .max_by(|(a, _), (b, _)| a.y.cmp(&b.y))
        .unwrap()
        .0
        .y;
    let max_x = grid
        .iter()
        .max_by(|(a, _), (b, _)| a.x.cmp(&b.x))
        .unwrap()
        .0
        .x;
    Paper {
        grid,
        instructions,
        max_x,
        max_y,
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap_or_default();
        Ok(Self {
            x: x.parse::<i32>()?,
            y: y.parse::<i32>()?,
        })
    }
}
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // fold along x=655
        let (dir, value) = s.split(" ").last().unwrap().split_once("=").unwrap();
        let val = value.parse::<i32>().unwrap();
        match dir {
            "x" => Ok(Instruction::FoldX(val)),
            "y" => Ok(Instruction::FoldY(val)),
            _ => panic!("Nope {}", s),
        }
    }
}

fn debug(paper: &Paper) {
    println!("{:?}", paper.grid);
    println!("");
    for y in 0..=paper.max_y {
        let mut line = Vec::new();
        for x in 0..=paper.max_x {
            let out = match paper.grid.get(&Point { x, y }) {
                Some(_) => "#",
                None => ".",
            };
            line.push(out);
        }
        println!("{}", line.join(""));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r#"
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
      "#;
        // let mut paper = parse(input);
        // apply(&mut paper);
        // debug(&paper);
        assert_eq!(part_1(input), 17);
    }
}
