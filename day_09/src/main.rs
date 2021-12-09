use std::collections::HashMap;
type Hyperhash = HashMap<Point, usize>;

fn main() {
    let out = part_1(include_str!("../input.txt"));
    println!("part_1: {}", out);
}

fn part_1(input: &str) -> u64 {
    let hyper = build_hyperhash(input);
    count_danger(hyper)
}

fn build_hyperhash(input: &str) -> Hyperhash {
    let mut hyperhash = HashMap::new();

    for (i, line) in input.trim().lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            let val = char.to_string().parse::<usize>().unwrap();
            hyperhash.insert(Point {i: i.try_into().unwrap(), j: j.try_into().unwrap()}, val);
        }
    }
    hyperhash
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    i: i8,
    j: i8
}

fn count_danger(hyper: Hyperhash) -> u64 {
    let mut cost = 0;
    for (Point {i, j}, value) in hyper.iter() {
        let up = Point { i: *i - 1, j: *j};
        let down = Point { i: *i + 1, j: *j};
        let left = Point { i: *i , j: *j - 1};
        let right = Point { i: *i , j: *j + 1};
        let lower_neighbors = [up, down, left, right]
            .into_iter()
            .filter_map(|direction|
                hyper.get(&direction)
            )
            .filter(|x| x <= &value) // true == keep ; elements that are less than current element
            .collect::<Vec<&usize>>();

        if lower_neighbors.is_empty() {
            cost += value + 1;
        }
    }
    cost.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

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
