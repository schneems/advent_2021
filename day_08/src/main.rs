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

fn part_2(input: &str) -> u64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
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
        assert_eq!(part_2(input), 61229);
    }

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
// one => 2
// four => 4
// seven => 3
// eight => 7

// A in 7,8 (not 1,4)
// seven & eight - one - four

// B in 4,8 (not 1,7)
// D in 4,8 (not 1,7)
// C in ALL
// F in ALL
// E in 8 (not 1,4,7)
// G in 8 (not 1,4,7)

// 1 =>     c,    f
// 4 =>   b,c,d,  f
// 7 => a,  c,    f
// 8 => a,b,c,d,e,f,g

// 2 => a,  c,d,e,  g
// 3 => a,  c,d,  f,g
// 5 => a,b,  d,  f,g

// 0 => a,b,c,  e,f,g
// 6 => a,b,  d,e,f,g
// 9 => a,b,c,d,  f,g


// one => 2
// four => 4
// seven => 3
// eight => 7

// one   (2) =>     c,    f
// four  (4) =>   b,c,d,  f
// seven (3) => a,  c,    f
// eight (7) => a,b,c,d,e,f,g

// count_to_sym = {}
// count_to_sym[2] => :one
// count_to_sym[4] => :four
// count_to_sym[3] => :seven
// count_to_sym[7] => :eight




// ## Unique
//
