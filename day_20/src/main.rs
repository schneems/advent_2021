use std::collections::HashMap;
//         Ok(// use std::str::FromStr;

fn main() {
    let out = part_1(include_str!("../input.txt"));
    println!("part_1: {}", out);

    let out = part_2(include_str!("../input.txt"));
    println!("part_2: {}", out);
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Point {
    i: i32,
    j: i32,
}

type Grid = HashMap<Point, String>;

fn part_1(input: &str) -> u64 {
    let Data {
        mut grid,
        algorithm,
    } = parse(input);
    enhance(&mut grid, &algorithm);
    enhance(&mut grid, &algorithm);

    grid.iter()
        .filter_map(|(_, v)| if v.as_str() == "#" { Some(1) } else { None })
        .count()
        .try_into()
        .unwrap()
}

fn part_2(input: &str) -> u64 {
    let _thing = parse(input);
    unimplemented!()
}

fn parse(input: &str) -> Data {
    let mut lines = input.trim().lines().into_iter();
    let algorithm = parse_enhance(lines.next().unwrap());
    lines.next();
    let grid = parse_grid(&lines.map(String::from).collect::<Vec<String>>().join("\n"));
    Data { algorithm, grid }
}

fn str_to_usize(input: &str) -> usize {
    usize::from_str_radix(input, 2).unwrap()
}

type Enhance = Vec<String>;

fn parse_enhance(string: &str) -> Enhance {
    string
        .trim()
        .split("")
        .into_iter()
        .filter(|c| c != &"")
        .map(|c| String::from(c))
        .collect::<Vec<String>>()
}

impl Point {
    fn neighbors(&self) -> Vec<Point> {
        (-1..=1)
            .into_iter()
            .flat_map(|i| {
                (-1..=1)
                    .into_iter()
                    .map(|j| Point {
                        i: self.i + i,
                        j: self.j + j,
                    })
                    .collect::<Vec<Point>>()
            })
            .collect::<Vec<Point>>()
    }
}

fn enhance<'a, 'b>(grid: &'a mut Grid, enhance: &'b Enhance) -> &'a mut Grid {
    let lookup = grid.clone();
    let min_i = min_i(&grid) - 3;
    let max_i = max_i(&grid) + 3;
    let min_j = min_j(&grid) - 3;
    let max_j = max_j(&grid) + 3;

    for i in min_i..=max_i {
        for j in min_j..=max_j {
            let point = Point { i, j };

            let substring = point
                .neighbors()
                .into_iter()
                .map(|p| {
                    //
                    lookup.get(&p).unwrap_or(&String::from(".")).clone()
                })
                .collect::<Vec<String>>()
                .clone()
                .join("");

            let index = enhance_index(&substring);
            // *v = enhance[index].clone();
            let value = grid.entry(point).or_insert(String::from("."));
            *value = enhance[index].clone();
        }
    }

    for (point, v) in grid.iter_mut() {
        let substring = point
            .neighbors()
            .into_iter()
            .map(|p| {
                //
                lookup.get(&p).unwrap_or(&String::from(".")).clone()
            })
            .collect::<Vec<String>>()
            .clone()
            .join("");

        let index = enhance_index(&substring);
        *v = enhance[index].clone();
    }

    grid
}

fn enhance_index(input: &str) -> usize {
    str_to_usize(
        input
            .trim()
            .split("")
            .into_iter()
            .filter(|c| c != &"")
            .map(|c| match c {
                "#" => "1",
                "." => "0",
                _ => panic!("Lol"),
            })
            .collect::<Vec<&str>>()
            .join("")
            .as_str(),
    )
}

struct Data {
    algorithm: Enhance,
    grid: Grid,
}

fn parse_grid(input: &str) -> Grid {
    input
        .trim()
        .lines()
        .into_iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .into_iter()
                .enumerate()
                .map(|(j, c)| {
                    (
                        Point {
                            i: i.try_into().unwrap(),
                            j: j.try_into().unwrap(),
                        },
                        c.to_string(),
                    )
                })
                .collect::<Vec<(Point, String)>>()
        })
        .collect::<HashMap<Point, String>>()
}

fn min_i(grid: &Grid) -> i32 {
    grid.iter()
        .min_by(|(a, _), (b, _)| a.i.cmp(&b.i))
        .unwrap()
        .0
        .i
}

fn max_i(grid: &Grid) -> i32 {
    grid.iter()
        .max_by(|(a, _), (b, _)| a.i.cmp(&b.i))
        .unwrap()
        .0
        .i
}

fn min_j(grid: &Grid) -> i32 {
    grid.iter()
        .min_by(|(a, _), (b, _)| a.j.cmp(&b.j))
        .unwrap()
        .0
        .j
}

fn max_j(grid: &Grid) -> i32 {
    grid.iter()
        .max_by(|(a, _), (b, _)| a.j.cmp(&b.j))
        .unwrap()
        .0
        .j
}

fn print(grid: &Grid) {
    let min_i = min_i(&grid);
    let max_i = max_i(&grid);
    let min_j = min_j(&grid);
    let max_j = max_j(&grid);

    println!("");
    for i in min_i..=max_i {
        let mut line = Vec::new();
        for j in min_j..=max_j {
            line.push(
                grid.get(&Point { i, j })
                    .unwrap_or(&String::from(".").clone())
                    .clone(),
            )
        }
        println!("{}", line.join(""));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm() {
        let algorithm = parse_enhance("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#");
        let mut grid = parse_grid(
            r#"
#..#.
#....
##..#
..#..
..###
"#,
        );

        print(&grid);
        enhance(&mut grid, &algorithm);
        print(&grid);
        enhance(&mut grid, &algorithm);
        print(&grid);
    }

    #[test]
    fn test_parse_enhance() {
        let input = r#"..#.#..#####.#.#.#.###.##.....###.##."#;
        let out = parse_enhance(input);
        assert_eq!(out[34], String::from("#"));
        assert_eq!(out[0], String::from("."));
        assert_eq!(out[2], String::from("#"));
    }

    #[test]
    fn test_enhance_index() {
        assert_eq!(enhance_index("...#...#."), 34);
    }

    #[test]
    fn test_parts() {
        let input = r#"
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
"#;
        assert_eq!(part_1(input), 35);
        // assert_eq!(part_2(input), 99);
    }
}
