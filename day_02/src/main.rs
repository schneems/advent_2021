fn main() {
    let out = depth_from_input(include_str!("../input_2.txt"));
    println!("{}", out);
}

fn depth_from_input(input: &str) -> u64 {
    let mut position = Position::new();
    for line in input.lines() {
        let motion = motion_from_line(line);
        position.move_it(motion);
    }
    position.depth * position.horizontal
}

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Forward,
    Up,
    Down,
}

impl From<&str> for Direction {
    fn from(value: &str) ->Self {
        match value {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            _ => panic!("Bad direction {}", value)
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Motion {
    direction: Direction,
    value: u64,
}

fn motion_from_line(line: &str) -> Motion {
    if let Some((dir, val)) = line.split_once(" ") {
        Motion {
            direction: dir.into(),
            value: val.parse().unwrap()
        }
    } else {
        panic!("No space found in line {}", line)
    }
}

struct Position {
    aim: u64,
    depth: u64,
    horizontal: u64,
}

impl Position {
    fn new() -> Self {
        Position {
            depth: 0,
            horizontal: 0,
            aim: 0,
        }
    }

    fn move_it(&mut self, motion: Motion) -> &mut Self {
        match motion.direction {
            Direction::Up => {
                self.aim -= motion.value;
            }
            Direction::Down => {
                self.aim += motion.value;
            }
            Direction::Forward => {
                self.horizontal += motion.value;
                self.depth += self.aim * motion.value
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //     #[test]
    //     fn part_one() {
    //         let out = depth_from_input(
    //             r#"forward 5
    // down 5
    // forward 8
    // up 3
    // down 8
    // forward 2"#,
    //         );
    //         assert_eq!(out, 150)
    //     }

    #[test]
    fn part_two() {
        let out = depth_from_input(
            r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#,
        );
        assert_eq!(out, 900)
    }

    #[test]
    fn test_position() {
        let mut pos = Position::new();
        assert_eq!(pos.depth, 0);

        pos.move_it(Motion {
            direction: Direction::Down,
            value: 5,
        });

        assert_eq!(pos.aim, 5);

        pos.move_it(Motion {
            direction: Direction::Up,
            value: 2,
        });

        assert_eq!(pos.aim, 3);
    }

    #[test]
    fn test_motion() {
        assert_eq!(
            motion_from_line("forward 5"),
            Motion {
                direction: Direction::Forward,
                value: 5
            }
        );
        assert_eq!(
            motion_from_line("up 5"),
            Motion {
                direction: Direction::Up,
                value: 5
            }
        );
        assert_eq!(
            motion_from_line("down 5"),
            Motion {
                direction: Direction::Down,
                value: 5
            }
        );

        assert_eq!(
            motion_from_line("down 10"),
            Motion {
                direction: Direction::Down,
                value: 10
            }
        );
    }
}
