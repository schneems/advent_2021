// use std::collections::HashMap;
// use std::str::FromStr;
// use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Copy, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Color {
    A,
    B,
    C,
    D,
    None,
}

#[derive(Copy, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum RoomState {
    Full,
    ReadyAt(usize),
    RemoveNext(usize),
}

#[derive(Copy, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum BoardSize {
    Small = 2,
    Big = 4,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
struct Board {
    size: BoardSize,
    hallway: [Color; 11],
    a: [Color; 4],
    b: [Color; 4],
    c: [Color; 4],
    d: [Color; 4],
}

impl Board {
    fn open_hallway_from(&self, color: &Color) -> Vec<usize> {
        let index = self.door_index(color);

        let mut out = Vec::new();
        for range in [
            (0..index).rev().collect::<Vec<usize>>(),
            (index + 1..11).collect::<Vec<usize>>(),
        ]
        .iter()
        {
            for i in range.into_iter() {
                if [2, 4, 6, 8].contains(i) {
                    continue;
                }

                if self.hallway[*i] == Color::None {
                    out.push(*i);
                } else {
                    break; // inner
                }
            }
        }
        out
    }

    fn room_state(&self, color: &Color) -> RoomState {
        let room = &self.room_for_color(color)[0..self.size as usize];

        if room.iter().all(|c| c == color) {
            return RoomState::Full;
        }

        if room.iter().all(|c| c == color || c == &Color::None) {
            return RoomState::ReadyAt(
                room.iter()
                    .enumerate()
                    .rev()
                    .find(|(_, c)| c == &&Color::None)
                    .unwrap()
                    .0,
            );
        }

        // Full,
        // ReadyAt(u8),
        // RemoveNext(u8),
        RoomState::RemoveNext(
            room.iter()
                .enumerate()
                .find(|(_, c)| c != &&Color::None)
                .unwrap()
                .0,
        )
    }

    fn door_index(&self, color: &Color) -> usize {
        match color {
            Color::A => 2,
            Color::B => 4,
            Color::C => 6,
            Color::D => 8,
            Color::None => panic!("nope"),
        }
    }

    fn steps_to_door_from(&self, index: usize) -> Option<u64> {
        let target = self.door_index(&self.hallway[index]);

        if target > index {
            for i in index + 1..target {
                if self.hallway[i] != Color::None {
                    return None;
                }
            }
            Some((target - index).try_into().unwrap())
        } else {
            for i in target + 1..index {
                if self.hallway[i] != Color::None {
                    return None;
                }
            }
            Some((index - target).try_into().unwrap())
        }
    }

    fn room_for_color(&self, color: &Color) -> [Color; 4] {
        match color {
            Color::A => self.a,
            Color::B => self.b,
            Color::C => self.c,
            Color::D => self.d,
            Color::None => panic!("nope"),
        }
    }

    fn color_is_happy(&self, color: &Color) -> bool {
        match self.room_state(color) {
            RoomState::Full => true,
            RoomState::ReadyAt(_) => !self.hallway.iter().any(|c| c == color),
            RoomState::RemoveNext(_) => false,
        }
    }

    fn room_is_ready(&self, color: &Color) -> Option<usize> {
        match self.room_state(color) {
            RoomState::ReadyAt(x) => Some(x),
            _ => None,
        }
    }
}

fn parse(input: &str) -> Board {
    let mut lines = input.trim().lines().into_iter();
    let size = match lines.clone().count() {
        7 => BoardSize::Big,
        5 => BoardSize::Small,
        _ => panic!("Unknown board size {}", lines.count()),
    };

    let mut hallway = [Color::None; 11];
    let mut a = [Color::None; 4];
    let mut b = [Color::None; 4];
    let mut c = [Color::None; 4];
    let mut d = [Color::None; 4];

    lines.next();
    let hall_chars = lines.next().unwrap().chars().collect::<Vec<char>>();
    for (i, c) in hall_chars[1..12].iter().enumerate() {
        hallway[i] = match c {
            'A' => Color::A,
            'B' => Color::B,
            'C' => Color::C,
            'D' => Color::D,
            '#' => continue,
            '.' => Color::None,
            ' ' => continue,
            _ => panic!("Highly unexpected {}", c),
        };
    }
    let mut index = 0;
    while let Some(line) = lines.next() {
        let chars = line.chars().collect::<Vec<char>>();

        for i in [3, 5, 7, 9] {
            let color = match chars[i] {
                'A' => Color::A,
                'B' => Color::B,
                'C' => Color::C,
                'D' => Color::D,
                '#' => continue,
                '.' => Color::None,
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
        size,
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
    let board = parse(input);
    play(board)
}

fn expand_board(input: &str) -> String {
    let mut lines = input.trim().lines().into_iter();
    [
        lines.next().unwrap().to_string(),
        lines.next().unwrap().to_string(),
        lines.next().unwrap().to_string(),
        "  #D#C#B#A#".to_string(),
        "  #D#B#A#C#".to_string(),
        lines.next().unwrap().to_string(),
    ]
    .join("\n")
}

fn part_2(input: &str) -> u64 {
    let board = parse(&expand_board(input));
    play(board)
}

fn color_to_str(color: &Color) -> String {
    match color {
        Color::A => "A".to_string(),
        Color::B => "B".to_string(),
        Color::C => "C".to_string(),
        Color::D => "D".to_string(),
        Color::None => ".".to_string(),
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
        .filter(|(_, c)| c != &&Color::None)
        .filter_map(|(i, c)| board.room_is_ready(c).map(|c_index| (i, c_index, c)))
        .find_map(|(index, c_index, c)| {
            board
                .steps_to_door_from(index)
                .map(|steps| (index, c_index, steps, c))
        });

    if let Some((index, c_index, steps, color)) = can_move {
        *cost += (steps + c_index as u64 + 1) * cost_color(color);
        match *color {
            Color::A => board.a[c_index] = color.clone(),
            Color::B => board.b[c_index] = color.clone(),
            Color::C => board.c[c_index] = color.clone(),
            Color::D => board.d[c_index] = color.clone(),
            Color::None => panic!("Nope"),
        };
        board.hallway[index] = Color::None;

        move_hallway_to_room(_heuristic, cost, board);
    }
}

fn move_room_to_hallway(
    _heuristic: u64,
    cost: u64,
    board: &Board,
    frontier: &mut BinaryHeap<Reverse<(u64, u64, Board)>>,
) {
    let mut full_count = 0;
    let mut maybe_move = Vec::new();
    for (room_color, state) in [
        (Color::A, board.room_state(&Color::A)),
        (Color::B, board.room_state(&Color::B)),
        (Color::C, board.room_state(&Color::C)),
        (Color::D, board.room_state(&Color::D)),
    ] {
        match state {
            RoomState::Full => full_count += 1,
            RoomState::ReadyAt(_) => continue,
            RoomState::RemoveNext(i) => maybe_move.push((i, room_color)),
        }
    }

    if full_count == 4 {
        frontier.push(Reverse((_heuristic, cost, board.clone())));
    }
    if maybe_move.is_empty() {
        return;
    }

    for (color_index, room_color) in maybe_move {
        for hall_index in board.open_hallway_from(&room_color).iter() {
            let mut next = board.clone();
            let steps = (next.door_index(&room_color) as i8 - *hall_index as i8).abs() as u64;

            let color = match room_color {
                Color::A => next.a[color_index],
                Color::B => next.b[color_index],
                Color::C => next.c[color_index],
                Color::D => next.d[color_index],
                Color::None => panic!("Nope"),
            };

            match room_color {
                Color::A => next.a[color_index] = Color::None,
                Color::B => next.b[color_index] = Color::None,
                Color::C => next.c[color_index] = Color::None,
                Color::D => next.d[color_index] = Color::None,
                Color::None => panic!("Nope"),
            };
            let next_cost = cost + (steps + color_index as u64 + 1) * cost_color(&color);
            next.hallway[*hall_index] = color.clone();

            frontier.push(Reverse((_heuristic, next_cost, next.clone())));
        }
    }
}

fn play(board: Board) -> u64 {
    let mut frontier = BinaryHeap::new();
    frontier.push(Reverse((0, 0, board.clone())));

    while let Some(Reverse((mut hueristic, mut cost, board))) = frontier.pop() {
        if board.color_is_happy(&Color::A)
            && board.color_is_happy(&Color::B)
            && board.color_is_happy(&Color::C)
            && board.color_is_happy(&Color::D)
        {
            return cost;
        }

        let mut next = board.clone();
        move_hallway_to_room(&mut hueristic, &mut cost, &mut next);
        move_room_to_hallway(hueristic, cost, &next, &mut frontier);

        // if board != next {
        //     frontier.push(Reverse((cost, cost, next)));
        // }
    }
    99
}

fn cost_color(color: &Color) -> u64 {
    match color {
        Color::A => 1,
        Color::B => 10,
        Color::C => 100,
        Color::D => 1000,
        Color::None => panic!("Cannot move nothing"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand() {
        let expanded = expand_board(
            r#"
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
"#,
        );

        println!("{}", expanded);

        assert_eq!(
            expanded.trim(),
            r#"
#############
#...........#
###B#C#B#D###
  #D#C#B#A#
  #D#B#A#C#
  #A#D#C#A#
"#
            .trim()
        )
    }

    #[test]
    fn test_bigger_endgame() {
        let board = parse(
            r#"
#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #A#B#C#D#
  #A#B#C#D#
  #########
"#,
        );

        assert_eq!(play(board), 0);

        let board = parse(
            r#"
#############
#..........D#
###A#B#C#.###
  #A#B#C#D#
  #A#B#C#D#
  #A#B#C#D#
  #########
"#,
        );

        assert_eq!(play(board), 3000);

        let board = parse(
            r#"
#############
#.........AD#
###.#B#C#.###
  #A#B#C#D#
  #A#B#C#D#
  #A#B#C#D#
  #########

"#,
        );

        assert_eq!(play(board), 8 + 3000);

        let board = parse(
            r#"
#############
#...D.....AD#
###.#B#C#.###
  #A#B#C#.#
  #A#B#C#D#
  #A#B#C#D#
  #########
"#,
        );

        assert_eq!(play(board), 7000 + 8 + 3000);

        let board = parse(
            r#"
#############
#AA.......AD#
###B#.#C#.###
  #D#B#C#.#
  #D#B#C#.#
  #A#B#C#D#
  #########
"#,
        );

        assert_eq!(play(board), 40 + 11000 + 4000 + 4 + 4 + 7000 + 8 + 3000);
    }

    #[test]
    fn test_part_1() {
        let board = parse(
            r#"
#############
#.....D.....#
###.#B#C#D###
  #A#B#C#A#
  #########
"#,
        );

        assert_eq!(board.room_state(&Color::D), RoomState::RemoveNext(0));

        assert_eq!(play(board), 2003 + 7000 + 8);

        let board = parse(
            r#"
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########

"#,
        );

        // assert_eq!(play(board), 40 + 400 + 3000 + 30 + 40 + 2003 + 7000 + 8);
    }

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

        assert_eq!(board.open_hallway_from(&Color::B), vec![5, 7, 9, 10]);

        assert_eq!(board.room_state(&Color::A), RoomState::Full);
        assert_eq!(board.room_state(&Color::B), RoomState::RemoveNext(0));
        assert_eq!(board.room_state(&Color::C), RoomState::ReadyAt(0));
        assert_eq!(board.room_state(&Color::D), RoomState::Full);
        assert_eq!(play(board), 400 + 20);

        let board = parse(
            r#"
#############
#...C.......#
###A#C#.#D###
  #A#B#B#D#
  #########
"#,
        );

        assert_eq!(board.room_state(&Color::B), RoomState::RemoveNext(0));
        assert_eq!(board.room_state(&Color::C), RoomState::RemoveNext(1));
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

        assert_eq!(board.room_is_ready(&Color::A).unwrap(), 0);
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
        assert_eq!(board.room_is_ready(&Color::A).unwrap(), 1);
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
        assert_eq!(&board.hallway, &[Color::None; 11]);
        assert_eq!(&board.a, &[Color::B, Color::A, Color::None, Color::None]);
        assert_eq!(&board.b, &[Color::C, Color::D, Color::None, Color::None]);
        assert_eq!(&board.c, &[Color::B, Color::C, Color::None, Color::None]);
        assert_eq!(&board.d, &[Color::D, Color::A, Color::None, Color::None]);
        // panic!("lol");
    }
}
