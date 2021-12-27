use std::collections::HashMap;
// use std::str::FromStr;

fn main() {
    let out = part_1(include_str!("../input.txt"));
    println!("part_1: {}", out);

    let out = part_2(include_str!("../input.txt"));
    println!("part_2: {}", out);
}

fn part_1(input: &str) -> u64 {
    let mut grid = parse(input);
    step_until_stop(&mut grid)
}

fn part_2(input: &str) -> u64 {
    let _thing = parse(input);
    unimplemented!()
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Cucumber {
    South,
    East,
    None,
}

type Grid = HashMap<(i16, i16), Cucumber>;

fn parse(input: &str) -> Grid {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, char)| {
                    let cucumber = match char {
                        '>' => Cucumber::East,
                        'v' => Cucumber::South,
                        '.' => Cucumber::None,
                        _ => panic!("at the disco {}", char),
                    };
                    ((i as i16, j as i16), cucumber)
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<_, _>>()
}

fn moves_in_dir(grid: &Grid, dir: &Cucumber) -> Vec<((i16, i16), (i16, i16))> {
    let mut out = Vec::new();
    for ((i, j), cucumber) in grid {
        if cucumber == dir {
            match cucumber {
                Cucumber::East => {
                    let mut next = (*i, j + 1);
                    if grid.get(&next).is_none() {
                        next = (*i, 0);
                    }
                    if let Some(other) = grid.get(&next) {
                        if other == &Cucumber::None {
                            out.push(((*i, *j), next));
                        }
                    }
                }
                Cucumber::South => {
                    let mut next = (i + 1, *j);
                    if grid.get(&next).is_none() {
                        next = (0, *j);
                    }
                    if let Some(other) = grid.get(&next) {
                        if other == &Cucumber::None {
                            out.push(((*i, *j), next));
                        }
                    }
                }
                Cucumber::None => unreachable!(),
            }
        }
    }
    out
}

fn step_dir(grid: &mut Grid, cucumbers: Vec<((i16, i16), (i16, i16))>, dir: &Cucumber) {
    for (from, to) in cucumbers {
        grid.insert(from, Cucumber::None);
        grid.insert(to, dir.clone());
    }
}

fn step_both(grid: &mut Grid) -> usize {
    let mut count = 0;
    for dir in [Cucumber::East, Cucumber::South] {
        let to_move = moves_in_dir(&grid, &dir);
        count += to_move.len();
        step_dir(grid, to_move, &dir);
    }
    count
}

fn step_until_stop(grid: &mut Grid) -> u64 {
    let mut count = 0;
    while step_both(grid) != 0 {
        count += 1
    }
    count + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_until_stop() {
        let mut grid = parse(
            r#"
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"#,
        );
        assert_eq!(step_until_stop(&mut grid), 58);
    }

    #[test]
    fn move_step_currents() {
        let mut grid = parse(
            r#"
...>...
.......
......>
v.....>
......>
.......
..vvv..
"#,
        );
        step_both(&mut grid);
        assert_eq!(
            grid,
            parse(
                r#"
..vv>..
.......
>......
v.....>
>......
.......
....v..
"#
            )
        );

        step_both(&mut grid);
        assert_eq!(
            grid,
            parse(
                r#"
....v>.
..vv...
.>.....
......>
v>.....
.......
.......
"#
            )
        );

        step_both(&mut grid);
        assert_eq!(
            grid,
            parse(
                r#"
......>
..v.v..
..>v...
>......
..>....
v......
.......
"#
            )
        );

        step_both(&mut grid);
        assert_eq!(
            grid,
            parse(
                r#"
>......
..v....
..>.v..
.>.v...
...>...
.......
v......
"#
            )
        );
    }

    #[test]
    fn move_step_both() {
        let mut grid = parse(
            r#"
..........
.>v....v..
.......>..
..........
"#,
        );
        step_both(&mut grid);
        assert_eq!(
            grid,
            parse(
                r#"
..........
.>........
..v....v>.
..........
"#
            )
        );
    }

    #[test]
    fn move_step_dir() {
        let mut grid = parse("...>>>>>...");
        let dir = Cucumber::East;
        let to_move = moves_in_dir(&grid, &dir);
        step_dir(&mut grid, to_move, &dir);
        assert_eq!(grid, parse("...>>>>.>.."));

        let to_move = moves_in_dir(&grid, &dir);
        step_dir(&mut grid, to_move, &dir);
        assert_eq!(grid, parse("...>>>.>.>."));
    }

    #[test]
    fn test_equality() {
        assert_eq!(parse("...>>>>>..."), parse("...>>>>>..."));
    }

    #[test]
    fn test_parts() {
        let input = r#""#;
        // assert_eq!(part_1(input), 99);
        // assert_eq!(part_2(input), 99);
    }
}
