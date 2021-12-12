use std::collections::HashMap;
use std::str::FromStr;

use std::collections::HashSet;

type Hyperhash = HashMap<String, Node>;
// use std::rc::Rc;

// type Hyperhash = HashMap<Point, usize>;
fn main() {
    let out = part_2(include_str!("../input.txt"));
    println!("part_2: {}", out);
}

fn part_2(input: &str) -> u64 {
    let hash = parse(input);
    count_bfs(&hash, "start".to_string(), "end".to_string())
}

#[derive(Eq, PartialEq, Debug, Hash)]
enum NodeType {
    Big,
    Small,
    Start,
    End,
}

#[derive(Eq, PartialEq, Debug, Hash)]
struct Node {
    connected: Vec<String>,
    name: String,
    ntype: NodeType,
}

impl FromStr for Node {
    type Err = String;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let connected = Vec::new();
        let name = string.to_string();
        let ntype = match string {
            "start" => NodeType::Start,
            "end" => NodeType::End,
            s if s.to_uppercase() == name => NodeType::Big,
            _ => NodeType::Small,
        };

        Ok(Node {
            connected,
            name,
            ntype,
        })
    }
}

fn parse(input: &str) -> Hyperhash {
    let mut hash = HashMap::new();
    for line in input.trim().lines() {
        let (from_s, to_s) = line.split_once("-").unwrap();
        let from = hash
            .entry(from_s.to_string())
            .or_insert_with(|| Node::from_str(from_s).unwrap());
        from.connected.push(to_s.to_string());

        let to = hash
            .entry(to_s.to_string())
            .or_insert_with(|| Node::from_str(to_s).unwrap());
        to.connected.push(from_s.to_string());
    }

    hash
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct AnswerPath {
    path: Vec<String>,
}

impl AnswerPath {
    fn new() -> Self {
        AnswerPath { path: vec![] }
    }

    fn last(&self) -> &String {
        self.path.last().unwrap()
    }

    fn push(&mut self, value: String) -> &mut Self {
        if value.to_lowercase() == value {
            if self.path.iter().all(|x| x != &value) {
                self.path.push(value);
            }
        } else {
            self.path.push(value);
        }
        self
    }
}

fn count_bfs(grid: &Hyperhash, start: String, objective: String) -> u64 {
    let mut answers: HashSet<AnswerPath> = HashSet::new();
    let mut frontier = Vec::new();

    for neighbor in &grid.get(&start).unwrap().connected {
        let mut history = AnswerPath::new();
        history.push(start.clone());
        history.push(neighbor.clone());
        frontier.push(history);
    }

    while let Some(path) = frontier.pop() {
        if path.last() == &objective {
            answers.insert(path);
        } else {
            for neighbor in &grid.get(path.last()).unwrap().connected {
                let mut new_path = path.clone();
                new_path.push(neighbor.clone());
                if new_path != path {
                    frontier.push(new_path);
                }
            }
        }
    }

    answers.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r#"
start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;

        let hash = parse(input);

        assert_eq!(part_2(input), 36);

        let input = r#"
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
        "#;
        assert_eq!(part_2(input), 3509);
    }
}
