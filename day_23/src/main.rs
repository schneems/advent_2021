// use std::collections::HashMap;
// use std::str::FromStr;
// use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

type Point = (i8, i8);

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
    last: AmphipodColor,
    hallway: [AmphipodColor; 10],
    a: [AmphipodColor; 2],
    b: [AmphipodColor; 2],
    c: [AmphipodColor; 2],
    d: [AmphipodColor; 2],
}

impl Board {
    fn color_is_happy(&self, color: &AmphipodColor) -> bool {
        let room = match color {
            &AmphipodColor::A => self.a,
            &AmphipodColor::B => self.b,
            &AmphipodColor::C => self.c,
            &AmphipodColor::D => self.d,
            &AmphipodColor::None => panic!("nope"),
        };
        room.iter().all(|c| c == color)
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

    let last = AmphipodColor::None;
    Board {
        last,
        hallway,
        a,
        b,
        c,
        d,
    }
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct Position {
//     cost: u64,
//     ready: HashMap<(i8, i8), Amphipod>,
//     waiting: HashMap<(i8, i8), Amphipod>,
//     happy: HashMap<(i8, i8), Amphipod>,
// }

// impl Ord for Position {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.cost.cmp(&other.cost)
//     }
// }

// impl PartialOrd for Position {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// fn build_world_example() -> Grid {
//     let mut grid = Grid::new();
//     for i in 0..=10 {
//         grid.insert((0, i), Square { room: None });
//     }
//     for (i, j) in [(-1, 2), (-2, 2)] {
//         grid.insert(
//             (i, j),
//             Square {
//                 room: Some(AmphipodColor::A),
//             },
//         );
//     }

//     for (i, j) in [(-1, 4), (-2, 4)] {
//         grid.insert(
//             (i, j),
//             Square {
//                 room: Some(AmphipodColor::B),
//             },
//         );
//     }

//     for (i, j) in [(-1, 6), (-2, 6)] {
//         grid.insert(
//             (i, j),
//             Square {
//                 room: Some(AmphipodColor::C),
//             },
//         );
//     }

//     for (i, j) in [(-1, 8), (-2, 8)] {
//         grid.insert(
//             (0, i),
//             Square {
//                 room: Some(AmphipodColor::C),
//             },
//         );
//     }
//     grid
// }

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
        &AmphipodColor::A => "A".to_string(),
        &AmphipodColor::B => "B".to_string(),
        &AmphipodColor::C => "C".to_string(),
        &AmphipodColor::D => "D".to_string(),
        &AmphipodColor::None => ".".to_string(),
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

fn play(board: Board) -> u64 {
    let mut frontier = BinaryHeap::new();
    frontier.push(Reverse((0, 0, board.clone())));

    if let Some(Reverse(board)) = frontier.pop() {}
    0
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
    fn test_endgame() {
        let board = parse(
            r#"
#############
#.........A.#
###.#B#C#D###
  #A#B#C#D#
  #########
"#,
        );

        assert_eq!(play(board), 8);
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
        let input = r#""#;
        // assert_eq!(part_1(input), 99);
        // assert_eq!(part_2(input), 99);
    }
}
