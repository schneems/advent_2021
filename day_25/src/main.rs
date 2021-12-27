use std::collections::HashMap;
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

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Cucumber {
    Down,
    Right,
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
                        '>' => Cucumber::Right,
                        'v' => Cucumber::Down,
                        '.' => Cucumber::None,
                        _ => panic!("at the disco {}", char),
                    };
                    ((i as i16, j as i16), cucumber)
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<_, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;

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
