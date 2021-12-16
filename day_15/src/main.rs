use std::collections::HashMap;
// use std::collections::HashSet;

// use std::str::FromStr;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

type NumGrid = HashMap<Point, i64>;
// type StrGrid = HashMap<Point, String>;

#[derive(PartialEq, Eq, Hash, Ord, PartialOrd, Debug, Clone)]
struct Point {
    i: i16,
    j: i16,
}

impl Point {
    fn _neighbors_diag(&self) -> Vec<Point> {
        let mut neighbors = Vec::with_capacity(8);
        for diff_i in [-1, 0, 1].iter() {
            for diff_j in [-1, 0, 1].iter() {
                if (*diff_i, *diff_j) != (0, 0) {
                    neighbors.push(Point {
                        i: self.i + diff_i,
                        j: self.j + diff_j,
                    })
                }
            }
        }
        neighbors
    }

    fn _neighbors(&self) -> Vec<Point> {
        let mut neighbors = Vec::with_capacity(8);
        for diff in [-1, 1].iter() {
            neighbors.push(Point {
                i: self.i + diff,
                j: self.j,
            });

            neighbors.push(Point {
                i: self.i,
                j: self.j + diff,
            })
        }
        neighbors
    }

    fn man_dist(&self, other: &Self) -> i64 {
        (self.i - other.i + self.j - other.j).abs().into()
    }
}

#[derive(PartialOrd, Ord, Eq, PartialEq, Debug)]
struct Route {
    heuristic: i64,
    last: Point,
}

impl Route {
    fn new(last: Point) -> Self {
        Route {
            heuristic: 0,
            last: last,
        }
    }
}

fn main() {
    let out = part_1(include_str!("../input.txt"));
    println!("part_1: {}", out);

    let out = part_2(include_str!("../input.txt"));
    println!("part_2: {}", out);
}

fn part_1(input: &str) -> i64 {
    let grid = parse(input);
    let points = grid.iter().map(|(point, _)| point);
    let max_i = *&points.max_by(|a, b| a.i.cmp(&b.i)).unwrap().i;
    let points = grid.iter().map(|(point, _)| point);
    let max_j = *&points.max_by(|a, b| a.j.cmp(&b.j)).unwrap().j;

    let target = Point { i: max_i, j: max_j };
    search(&grid, target)
}

fn part_2(input: &str) -> i64 {
    let grid = parse(input);
    let grid = expand(&grid);

    let points = grid.iter().map(|(point, _)| point);
    let max_i = *&points.max_by(|a, b| a.i.cmp(&b.i)).unwrap().i;
    let points = grid.iter().map(|(point, _)| point);
    let max_j = *&points.max_by(|a, b| a.j.cmp(&b.j)).unwrap().j;

    let target = Point { i: max_i, j: max_j };
    search(&grid, target)
}

fn expand(grid: &NumGrid) -> NumGrid {
    let points = grid.iter().map(|(point, _)| point);
    let max_i = *&points.max_by(|a, b| a.i.cmp(&b.i)).unwrap().i;
    let points = grid.iter().map(|(point, _)| point);
    let max_j = *&points.max_by(|a, b| a.j.cmp(&b.j)).unwrap().j;

    let max_i = max_i + 1;
    let max_j = max_j + 1;

    let mut new_grid = grid.clone();
    let max_max_i = (max_i) * 5;
    let max_max_j = (max_j) * 5;

    for i in 0..max_max_i {
        for j in 0..max_max_j {
            let value = [Point { i: i - max_i, j: j }, Point { i: i, j: j - max_j }]
                .into_iter()
                .filter_map(|x| new_grid.get(&x))
                .max();

            if let Some(value) = value {
                let value = (9 + *value) % 9 + 1;
                new_grid.insert(Point { i, j }, value);
            }
        }
    }
    new_grid
}

// use sorted_vec::ReverseSortedVec;

fn search(grid: &NumGrid, target: Point) -> i64 {
    let start = Point { i: 0, j: 0 };
    let mut frontier = BinaryHeap::new();
    let mut visited: HashMap<Point, i64> = HashMap::new();
    let mut _picture = grid.clone();

    frontier.push(Reverse(Route::new(start.clone())));
    visited.insert(start, 0);
    while let Some(Reverse(route)) = frontier.pop() {
        if route.last == target {
            return *visited.get(&route.last).unwrap();
        }
        let cost = *visited.get(&route.last).unwrap();

        let mut neighbors = Vec::with_capacity(4);
        for diff in [-1, 1].iter() {
            neighbors.push(Point {
                i: route.last.i + diff,
                j: route.last.j,
            });

            neighbors.push(Point {
                i: route.last.i,
                j: route.last.j + diff,
            })
        }

        for neighbor in neighbors {
            if let Some(value) = grid.get(&neighbor) {
                let new_cost = cost + value;
                if let Some(last_cost) = visited.get(&neighbor) {
                    if new_cost < *last_cost {
                        visited.insert(neighbor.clone(), new_cost);
                        frontier.push(Reverse(Route {
                            heuristic: neighbor.man_dist(&target) + new_cost,
                            last: neighbor,
                        }));
                    }
                } else {
                    visited.insert(neighbor.clone(), new_cost);
                    frontier.push(Reverse(Route {
                        heuristic: neighbor.man_dist(&target) + new_cost,
                        last: neighbor,
                    }));
                }
            }
        }

        // picture.remove(&route.last);
        // debug(&picture);
        // visited.insert(route.last);
    }

    panic!("No such luck")
}

fn debug(grid: &NumGrid) {
    let points = grid.iter().map(|(point, _)| point);
    let max_i = *&points.max_by(|a, b| a.i.cmp(&b.i)).unwrap().i;
    let points = grid.iter().map(|(point, _)| point);
    let max_j = *&points.max_by(|a, b| a.j.cmp(&b.j)).unwrap().j;
    println!("");
    for i in 0..=max_i {
        let mut line = Vec::new();
        for j in 0..=max_j {
            let out = match grid.get(&Point { i, j }) {
                Some(x) => x.to_string(),
                None => ".".to_string(),
            };
            line.push(out);
        }
        println!("{}", line.join(""));
    }
}

fn parse(input: &str) -> NumGrid {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().map(move |(j, c)| {
                (
                    Point {
                        i: i as i16,
                        j: j as i16,
                    },
                    c.to_string().parse::<i64>().unwrap(),
                )
            })
        })
        .collect::<NumGrid>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand() {
        let input = r#"8"#;

        let expected = r#"
89123
91234
12345
23456
34567"#;

        assert_eq!(expand(&parse(input)), parse(expected))
    }

    #[test]
    fn test_part_1() {
        let input = r#"
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;
        // let grid = parse(input);
        // let route = search(&grid, Point { i: 9, j: 9 });
        // assert_eq!(route.cost, 40);
        // assert_eq!(part_1(input), 40);

        assert_eq!(part_2(input), 315);
    }
}
