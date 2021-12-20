// use itertools::Itertools;

// use std::collections::HashMap;
// use std::str::FromStr;
#[derive(Clone, Eq, PartialEq, Debug)]
struct HyperNum {
    depth: i32,
    val: i32,
}

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

fn parse(input: &str) -> Vec<HyperNum> {
    let mut depth = 0;
    let mut out = Vec::new();
    let mut chars = Vec::new();
    for c in input.trim().split("") {
        match c {
            "[" => depth += 1,
            "]" => {
                if chars.len() > 0 {
                    out.push(HyperNum {
                        depth: depth,
                        val: chars.join("").parse().unwrap(),
                    });
                    chars.clear();
                }

                depth -= 1;
            }
            "," => {
                if chars.len() > 0 {
                    out.push(HyperNum {
                        depth: depth,
                        val: chars.join("").parse().unwrap(),
                    });
                    chars.clear();
                }
            }
            _ => {
                chars.push(c);
            }
        }
    }

    out
}

type HyperGraph = Vec<HyperNum>;

fn add(left: HyperGraph, right: HyperGraph) -> HyperGraph {
    let mut out = left.clone();
    out.extend(right);

    for mut x in out.iter_mut() {
        x.depth += 1;
    }

    out
}

// let out = explode(parse("[[[[[9,8],1],2],3],4]"));
// assert_eq!(out, parse("[[[[0,9],2],3],4]"));
fn explode(input: HyperGraph) -> HyperGraph {
    let mut input = input.clone();
    if let Some(a_index) = input.iter().position(|x| x.depth > 4) {
        if a_index > 0 {
            let a_val = input[a_index].val;
            let mut left = input.get_mut(a_index - 1).unwrap();
            left.val += a_val;
        }

        let mut a = input.get_mut(a_index).unwrap();
        a.depth -= 1;
        a.val = 0;

        let last = input.remove(a_index + 1);
        if let Some(right) = input.get_mut(a_index + 1) {
            right.val += last.val;
        }
    }

    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explode() {
        let out = explode(parse("[[[[[9,8],1],2],3],4]"));
        assert_eq!(out, parse("[[[[0,9],2],3],4]"));

        let out = explode(parse("[7,[6,[5,[4,[3,2]]]]]"));
        assert_eq!(out, parse("[7,[6,[5,[7,0]]]]"));

        let out = explode(parse("[[6,[5,[4,[3,2]]]],1]"));
        assert_eq!(out, parse("[[6,[5,[7,0]]],3]"));

        let out = explode(parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"));
        assert_eq!(out, parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));

        let out = explode(parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));
        assert_eq!(out, parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
    }

    #[test]
    fn test_add() {
        let out = add(parse("[1,2]"), parse("[[3,4],5]"));
        assert_eq!(out, parse("[[1,2],[[3,4],5]]"));
    }

    #[test]
    fn test_blerg() {
        let out = parse("[1,2]");
        assert_eq!(
            out,
            vec![HyperNum { depth: 1, val: 1 }, HyperNum { depth: 1, val: 2 }]
        );

        let out = parse("[[3,4],5]");

        assert_eq!(
            out,
            vec![
                HyperNum { depth: 2, val: 3 },
                HyperNum { depth: 2, val: 4 },
                HyperNum { depth: 1, val: 5 }
            ]
        )
    }

    #[test]
    fn test_parts() {
        let input = r#""#;
        assert_eq!(part_1(input), 99);
        // assert_eq!(part_2(input), 99);
    }
}
