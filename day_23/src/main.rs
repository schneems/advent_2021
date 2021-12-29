// use std::collections::HashMap;
// use std::str::FromStr;
// use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Copy, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum AmphipodColor {
    A,
    B,
    C,
    D,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
struct Board {
    hallway: [AmphipodColor; 10],
    a: [AmphipodColor; 2],
    b: [AmphipodColor; 2],
    c: [AmphipodColor; 2],
    d: [AmphipodColor; 2],
}

impl Board {
    fn door_index(&self, color: &AmphipodColor) -> usize {
        match color {
            AmphipodColor::A => 2,
            AmphipodColor::B => 4,
            AmphipodColor::C => 6,
            AmphipodColor::D => 8,
            AmphipodColor::None => panic!("nope"),
        }
    }

    fn steps_to_door_from(&self, index: usize) -> Option<u64> {
        let target = self.door_index(&self.hallway[index]);

        if target > index {
            for i in index + 1..target {
                if self.hallway[i] != AmphipodColor::None {
                    return None;
                }
            }
            Some((target - index).try_into().unwrap())
        } else {
            for i in target + 1..index {
                if self.hallway[i] != AmphipodColor::None {
                    return None;
                }
            }
            Some((index - target).try_into().unwrap())
        }
    }

    fn room_for_color(&self, color: &AmphipodColor) -> [AmphipodColor; 2] {
        match color {
            AmphipodColor::A => self.a,
            AmphipodColor::B => self.b,
            AmphipodColor::C => self.c,
            AmphipodColor::D => self.d,
            AmphipodColor::None => panic!("nope"),
        }
    }

    fn color_is_happy(&self, color: &AmphipodColor) -> bool {
        self.room_for_color(&color).iter().all(|c| c == color)
    }

    fn room_is_ready(&self, color: &AmphipodColor) -> Option<usize> {
        let room = self.room_for_color(&color);
        if room.iter().all(|c| c == &AmphipodColor::None || c == color) {
            let (i, _) = room
                .iter()
                .enumerate()
                .filter(|(_, c)| c == &&AmphipodColor::None)
                .last()
                .unwrap();

            Some(i)
        } else {
            None
        }
    }
}

fn parse(input: &str) -> Board {
    let mut lines = input.trim().lines().into_iter();
    let mut hallway = [AmphipodColor::None; 10];
    let mut a = [AmphipodColor::None; 2];
    let mut b = [AmphipodColor::None; 2];
    let mut c = [AmphipodColor::None; 2];
    let mut d = [AmphipodColor::None; 2];

    lines.next();
    let hall_chars = lines.next().unwrap().chars().collect::<Vec<char>>();
    for (i, c) in hall_chars[1..11].iter().enumerate() {
        hallway[i] = match c {
            'A' => AmphipodColor::A,
            'B' => AmphipodColor::B,
            'C' => AmphipodColor::C,
            'D' => AmphipodColor::D,
            '#' => continue,
            '.' => AmphipodColor::None,
            ' ' => continue,
            _ => panic!("Highly unexpected {}", c),
        };
    }
    let mut index = 0;
    while let Some(line) = lines.next() {
        let chars = line.chars().collect::<Vec<char>>();

        for i in [3, 5, 7, 9] {
            let color = match chars[i] {
                'A' => AmphipodColor::A,
                'B' => AmphipodColor::B,
                'C' => AmphipodColor::C,
                'D' => AmphipodColor::D,
                '#' => continue,
                '.' => AmphipodColor::None,
                ' ' => continue,
                _ => panic!("Highly unexpected {}", chars[i]),
            };
            match i {
                3 => a[index] = color,
                5 => b[index] = color,
                7 => c[index] = color,
                9 => d[index] = color,
                _ => panic!("Oops {}", i),
            }
        }

        index += 1;
    }

    Board {
        hallway,
        a,
        b,
        c,
        d,
    }
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

fn color_to_str(color: &AmphipodColor) -> String {
    match color {
        AmphipodColor::A => "A".to_string(),
        AmphipodColor::B => "B".to_string(),
        AmphipodColor::C => "C".to_string(),
        AmphipodColor::D => "D".to_string(),
        AmphipodColor::None => ".".to_string(),
    }
}

fn print(board: &Board) {
    println!();
    println!("###########");
    println!(
        "{}",
        board
            .hallway
            .iter()
            .map(color_to_str)
            .collect::<Vec<_>>()
            .join("")
    );
    for i in 0..2 {
        print!("##{}", color_to_str(&board.a[i]));
        print!("#{}", color_to_str(&board.b[i]));
        print!("#{}", color_to_str(&board.c[i]));
        print!("#{}", color_to_str(&board.d[i]));
        println!("##");
    }
    println!("###########");
}

fn move_hallway_to_room(_heuristic: &mut u64, cost: &mut u64, board: &mut Board) {
    let can_move = board
        .hallway
        .iter()
        .enumerate()
        .filter(|(_, c)| c != &&AmphipodColor::None)
        .filter_map(|(i, c)| board.room_is_ready(c).map(|c_index| (i, c_index, c)))
        .find_map(|(index, c_index, c)| {
            board
                .steps_to_door_from(index)
                .map(|steps| (index, c_index, steps, c))
        });

    if let Some((index, c_index, steps, color)) = can_move {
        *cost += (steps + c_index as u64 + 1) * cost_color(color);
        match *color {
            AmphipodColor::A => board.a[c_index] = color.clone(),
            AmphipodColor::B => board.b[c_index] = color.clone(),
            AmphipodColor::C => board.c[c_index] = color.clone(),
            AmphipodColor::D => board.d[c_index] = color.clone(),
            AmphipodColor::None => panic!("Nope"),
        };
        board.hallway[index] = AmphipodColor::None;

        println!("steps {}", steps);
        println!("room cost {}", c_index + 1);
        move_hallway_to_room(_heuristic, cost, board);
    }
}

fn play(board: Board) -> u64 {
    let mut frontier = BinaryHeap::new();
    frontier.push(Reverse((0, 0, board.clone())));

    while let Some(Reverse((mut hueristic, mut cost, board))) = frontier.pop() {
        print(&board);
        if board.color_is_happy(&AmphipodColor::A)
            && board.color_is_happy(&AmphipodColor::B)
            && board.color_is_happy(&AmphipodColor::C)
            && board.color_is_happy(&AmphipodColor::D)
        {
            return cost;
        }

        let mut next = board.clone();
        move_hallway_to_room(&mut hueristic, &mut cost, &mut next);

        if board != next {
            frontier.push(Reverse((cost, cost, next)));
        }
    }
    99
}

fn cost_color(color: &AmphipodColor) -> u64 {
    match color {
        AmphipodColor::A => 1,
        AmphipodColor::B => 10,
        AmphipodColor::C => 100,
        AmphipodColor::D => 1000,
        AmphipodColor::None => panic!("Cannot move nothing"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_to_hallway() {
        let board = parse(
            r#"
#############
#...B.......#
###A#C#.#D###
  #A#B#C#D#
  #########
"#,
        );

        assert_eq!(play(board), 400);
    }

    #[test]
    fn test_endgame() {
        let board = parse(
            r#"
#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########
"#,
        );

        assert_eq!(play(board), 0);
    }

    #[test]
    fn test_clear_hallway_one() {
        let board = parse(
            r#"
#############
#.........A.#
###.#B#C#D###
  #A#B#C#D#
  #########
"#,
        );

        assert_eq!(board.room_is_ready(&AmphipodColor::A).unwrap(), 0);
        assert_eq!(play(board), 8);
    }

    #[test]
    fn test_clear_hallway_two() {
        let board = parse(
            r#"
#############
#A........A.#
###.#B#C#D###
  #.#B#C#D#
  #########
"#,
        );

        assert_eq!(board.steps_to_door_from(9).unwrap(), 7);
        assert_eq!(board.steps_to_door_from(0).unwrap(), 2);
        assert_eq!(board.room_is_ready(&AmphipodColor::A).unwrap(), 1);
        assert_eq!(play(board), 12);

        let board = parse(
            r#"
#############
#A......D.A.#
###.#B#C#.###
  #.#B#C#D#
  #########
"#,
        );

        assert_eq!(play(board), 2012);
    }

    #[test]
    fn test_parsing() {
        let board = parse(
            r#"
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
"#,
        );

        print(&board);
        assert_eq!(&board.hallway, &[AmphipodColor::None; 10]);
        assert_eq!(&board.a, &[AmphipodColor::B, AmphipodColor::A]);
        assert_eq!(&board.b, &[AmphipodColor::C, AmphipodColor::D]);
        assert_eq!(&board.c, &[AmphipodColor::B, AmphipodColor::C]);
        assert_eq!(&board.d, &[AmphipodColor::D, AmphipodColor::A]);
        // panic!("lol");
    }

    #[test]
    fn test_parts() {
        // let input = r#""#;
        // assert_eq!(part_1(input), 99);
        // assert_eq!(part_2(input), 99);
    }
}
