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
    magnitude(&mut add_reduce(input))
}

fn part_2(input: &str) -> u64 {
    let mut largest = u64::MIN;
    let lines = input.trim().lines().map(parse).collect::<Vec<HyperGraph>>();
    for i in 0..lines.len() {
        for j in 0..lines.len() {
            for (x, y) in [(i, j), (j, i)] {
                let mut out = add(lines[x].clone(), lines[y].clone());
                reduce(&mut out);

                let val = magnitude(&mut out);
                if val > largest {
                    largest = val;
                }
            }
        }
    }
    largest
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
fn explode(input: &mut HyperGraph) -> bool {
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
        true
    } else {
        false
    }
}
fn reduce(input: &mut HyperGraph) {
    while explode(input) || split(input) {
        // iterate
    }
}

fn add_reduce(input: &str) -> HyperGraph {
    let mut lines = input.trim().lines().into_iter();
    let mut last = parse(lines.next().unwrap());
    reduce(&mut last);
    for line in lines {
        last = add(last, parse(line));
        reduce(&mut last);
    }
    last
}

fn split(input: &mut HyperGraph) -> bool {
    if let Some(a_index) = input.iter().position(|x| x.val >= 10) {
        let a = input.remove(a_index);
        let left = num::integer::div_floor(a.val, 2);
        let right = num::integer::div_ceil(a.val, 2);

        input.insert(
            a_index,
            HyperNum {
                depth: a.depth + 1,
                val: left,
            },
        );
        input.insert(
            a_index + 1,
            HyperNum {
                depth: a.depth + 1,
                val: right,
            },
        );

        true
    } else {
        false
    }
}

// The magnitude of a pair is 3 times the magnitude of its left
// plus 2 times the magnitude of its right element.
fn magnitude(input: &mut HyperGraph) -> u64 {
    while input.len() > 1 {
        for i in 0..input.len() {
            let a_val = input[i].val;
            let a_depth = input[i].depth;
            if let Some(b) = input.get_mut(i + 1) {
                if a_depth == b.depth {
                    let left = 3 * a_val;
                    let right = 2 * b.val;
                    b.val = left + right;
                    b.depth -= 1;
                    input.remove(i);
                    break; // start over again
                }
            }
        }
    }
    input[0].val.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mag() {
        let out = magnitude(&mut parse("[9,1]"));
        assert_eq!(out, 29);

        let out = magnitude(&mut parse(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        ));
        assert_eq!(out, 3488);

        let out = magnitude(&mut add_reduce(
            r#"
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
"#,
        ));
        assert_eq!(out, 4140);
    }

    #[test]
    fn test_moar_add() {
        let out = add_reduce(
            r#"
[1,1]
[2,2]
[3,3]
[4,4]
        "#,
        );
        assert_eq!(out, parse("[[[[1,1],[2,2]],[3,3]],[4,4]]"));

        let out = add_reduce(
            r#"
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
        "#,
        );
        assert_eq!(
            out,
            parse("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
        );
    }

    #[test]
    fn test_reduce() {
        let mut out = parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        reduce(&mut out);
        assert_eq!(out, parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }

    #[test]
    fn test_manual_reduce() {
        let mut out = parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        explode(&mut out);
        explode(&mut out);
        split(&mut out);
        split(&mut out);
        explode(&mut out);
        assert_eq!(out, parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }

    #[test]
    fn test_split() {
        let mut out = parse("[3,10]");
        split(&mut out);
        assert_eq!(out, parse("[3,[5,5]]"));
    }

    #[test]
    fn test_explode() {
        let mut out = parse("[[[[[9,8],1],2],3],4]");
        explode(&mut out);
        assert_eq!(out, parse("[[[[0,9],2],3],4]"));

        let mut out = parse("[7,[6,[5,[4,[3,2]]]]]");
        explode(&mut out);
        assert_eq!(out, parse("[7,[6,[5,[7,0]]]]"));

        let mut out = parse("[[6,[5,[4,[3,2]]]],1]");
        explode(&mut out);
        assert_eq!(out, parse("[[6,[5,[7,0]]],3]"));

        let mut out = parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        explode(&mut out);
        assert_eq!(out, parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));

        let mut out = parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        explode(&mut out);
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
        let input = r#"
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
"#;

        assert_eq!(part_2(input), 3993);
    }
}
