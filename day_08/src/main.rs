// use std::collections::HashMap;

fn main() {
    let out = part_1(include_str!("../input.txt"));
    println!("part_1: {}", out);
}

fn part_1(input: &str) -> u64 {
    let mut count = 0;
    for line in input.trim().lines() {
        let (paterns, digits) = line.split_once(" | ").unwrap();
        for value in digits.split(" ") {
            match value.len() {
                2 | 3 | 4 | 3 | 7 => count += 1,
                _ => {}
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r#"
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        "#;
        assert_eq!(part_1(input), 26);
    }
}

// Unique(value, count)
// 1 => 2
// 4 => 4
// 7 => 3
// 8 => 7

// Not unique(value, count)
// 2 => 5
// 3 => 5
// 5 => 5

// 0 => 6
// 6 => 6
// 9 => 6
