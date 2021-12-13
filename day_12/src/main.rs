use std::collections::HashMap;
use std::str::FromStr;

type Hyperhash = HashMap<String, Node>;

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

#[derive(Clone, PartialEq, Eq, Debug)]
struct AnswerPath {
    path: Vec<String>,
    duplicate: Option<String>,
}

impl AnswerPath {
    fn new() -> Self {
        AnswerPath {
            path: vec![],
            duplicate: None,
        }
    }

    fn last(&self) -> &String {
        self.path.last().unwrap()
    }

    fn can_push(&self, value: &String) -> bool {
        if value.chars().last().unwrap().is_ascii_lowercase() {
            match self.path.iter().filter(|x| x == &value).count() {
                0 => true,
                1 => self.duplicate.is_none() && value != "start" && value != "end",
                _ => false,
            }
        } else {
            true
        }
    }

    fn push(&mut self, value: String) -> &mut Self {
        if value.chars().last().unwrap().is_ascii_lowercase() {
            match self.path.iter().filter(|x| x == &&value).count() {
                0 => self.path.push(value),
                1 => {
                    if self.duplicate.is_none() && value != "start" && value != "end" {
                        self.duplicate = Some(value.clone());
                        self.path.push(value);
                    }
                }
                _ => {}
            }
        } else {
            self.path.push(value);
        }
        self
    }
}

fn count_bfs(grid: &Hyperhash, start: String, objective: String) -> u64 {
    let mut answers = Vec::new();
    let mut frontier = Vec::new();

    for neighbor in &grid.get(&start).unwrap().connected {
        let mut history = AnswerPath::new();
        history.push(start.clone());
        history.push(neighbor.clone());
        frontier.push(history);
    }

    while let Some(path) = frontier.pop() {
        if path.last() == &objective {
            answers.push(path);
        } else {
            for neighbor in &grid.get(path.last()).unwrap().connected {
                if path.can_push(neighbor) {
                    let mut new_path = path.clone();
                    new_path.push(neighbor.clone());
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
        assert_eq!(part_2(input), 103);
    }
}
