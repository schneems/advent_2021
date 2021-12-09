use std::collections::HashMap;
type Hyperhash = HashMap<Point, usize>;

fn main() {
    let out = part_1(include_str!("../input.txt"));
    println!("part_1: {}", out);

    let out = part_2(include_str!("../input.txt"));
    println!("part_2: {}", out);
}

fn part_1(input: &str) -> u64 {
    let hyper = build_hyperhash(input);
    count_danger(hyper)
}

fn part_2(input: &str) -> u64 {
    let hyper = build_hyperhash(input);
    let mut basins = lowest(&hyper)
        .iter()
        .map(|point| fill_basin_count(&hyper, point.clone()))
        .collect::<Vec<u64>>();

    basins.sort_unstable();
    basins
        .into_iter()
        .rev()
        .take(3)
        .reduce(|a, b| a * b)
        .unwrap()
}

fn build_hyperhash(input: &str) -> Hyperhash {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(move |(i, line)| {
            line.chars().enumerate().map(move |(j, char)| {
                (
                    Point {
                        i: i.try_into().unwrap(),
                        j: j.try_into().unwrap(),
                    },
                    char.to_string().parse::<usize>().unwrap(),
                )
            })
        })
        .collect::<Hyperhash>()
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    i: i8,
    j: i8,
}

impl Point {
    fn neighbors(&self) -> [Self; 4] {
        let up = Point {
            i: self.i - 1,
            j: self.j,
        };
        let down = Point {
            i: self.i + 1,
            j: self.j,
        };
        let left = Point {
            i: self.i,
            j: self.j - 1,
        };
        let right = Point {
            i: self.i,
            j: self.j + 1,
        };

        [up, down, left, right]
    }
}

fn fill_basin_count(hyper: &Hyperhash, point: Point) -> u64 {
    let mut hyper = hyper.clone();
    let mut frontier = vec![point];
    let mut basin = Vec::new();

    while let Some(point) = frontier.pop() {
        hyper.remove(&point);
        basin.push(point.clone());

        for p in point.neighbors().iter() {
            if let Some(x) = hyper.get(&p) {
                if *x < 9 {
                    hyper.remove(&p);
                    frontier.push(p.clone());
                }
            }
        }
    }

    basin.len().try_into().unwrap()
}

fn lowest(hyper: &Hyperhash) -> Vec<Point> {
    let mut out = Vec::new();
    for (point, value) in hyper.iter() {
        let lower_neighbors = point
            .neighbors()
            .into_iter()
            .filter_map(|direction| hyper.get(&direction))
            .filter(|x| x <= &value) // true == keep ; elements that are less than current element
            .collect::<Vec<&usize>>();

        if lower_neighbors.is_empty() {
            out.push(point.clone())
        }
    }
    out
}

fn count_danger(hyper: Hyperhash) -> u64 {
    let mut cost = 0;
    for point in lowest(&hyper) {
        if let Some(value) = hyper.get(&point) {
            cost += value + 1;
        }
    }
    cost.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let input = r#"
2199943210
3987894921
9856789892
8767896789
9899965678
      "#;

        let hyper = build_hyperhash(input);
        assert_eq!(fill_basin_count(&hyper, Point { i: 0, j: 1 }), 3);
        assert_eq!(fill_basin_count(&hyper, Point { i: 0, j: 9 }), 9);
        assert_eq!(fill_basin_count(&hyper, Point { i: 2, j: 2 }), 14);
        assert_eq!(fill_basin_count(&hyper, Point { i: 4, j: 6 }), 9);

        assert_eq!(part_2(input), 1134);
    }

    #[test]
    fn test_part_1() {
        let input = r#"
2199943210
3987894921
9856789892
8767896789
9899965678
      "#;

        assert_eq!(part_1(input), 15);
    }
}
