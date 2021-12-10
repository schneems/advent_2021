fn main() {
    let out = part_1(include_str!("../input.txt"));
    println!("part_1: {}", out);

    let out = part_2(include_str!("../input.txt"));
    println!("part_1: {}", out);
}

fn part_2(input: &str) -> u64 {
    let mut incomplete_costs = input
        .trim()
        .lines()
        .map(cost_incomplete)
        .filter(|cost| *cost > 1)
        .collect::<Vec<u64>>();

    incomplete_costs.sort_unstable();

    incomplete_costs[incomplete_costs.len() / 2]
}

fn part_1(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(cost_first_syntax_error)
        .sum()
}

fn cost_incomplete(line: &str) -> u64 {
    let chars = line.chars();
    let mut stack = Vec::new();
    for char in chars {
        let char = char.to_string();
        if is_opening(&char) {
            stack.push(char)
        } else {
            let last = stack.pop().unwrap();
            if char_inverse(&char) != last {
                return 0; // syntax error, not incomplete
            }
        }
    }

    let mut cost = 0;
    while let Some(c) = stack.pop() {
        cost *= 5;
        let fix = char_inverse(&c);
        cost += syntax_cost_part_2(fix);
    }
    cost
}

fn syntax_cost_part_2(input: &str) -> u64 {
    match input {
        ")" => 1,
        "]" => 2,
        "}" => 3,
        ">" => 4,
        _ => 0,
    }
}

fn cost_first_syntax_error(line: &str) -> u64 {
    let chars = line.chars();
    let mut stack = Vec::new();
    for char in chars {
        let char = char.to_string();
        if is_opening(&char) {
            stack.push(char)
        } else {
            let last = stack.pop().unwrap();
            if char_inverse(&char) != last {
                return match char.as_str() {
                    ")" => 3,
                    "]" => 57,
                    "}" => 1197,
                    ">" => 25137,
                    _ => panic!("nope"),
                }
            }
        }
    }
    0
}

fn is_opening(char: &str) -> bool {
    matches!(char, "(" | "[" | "{" | "<")
}

fn char_inverse(char: &str) -> &str {
    match char {
        "(" => ")",
        ")" => "(",
        "[" => "]",
        "]" => "[",
        "{" => "}",
        "}" => "{",
        "<" => ">",
        ">" => "<",
        _ => panic!("no inverse for {}", char),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cost_incomplete() {
        assert_eq!(cost_incomplete("[({(<(())[]>[[{[]{<()<>>"), 288957);
    }

    #[test]
    fn test_inverse() {
        assert_eq!(char_inverse("("), ")");
    }

    #[test]
    fn test_scan_line() {
        assert_eq!(cost_first_syntax_error("()"), 0);
        assert_eq!(cost_first_syntax_error("([])"), 0);
        assert_eq!(cost_first_syntax_error("<([{}])>"), 0);
        assert_eq!(cost_first_syntax_error("[<>({}){}[([])<>]]"), 0);
        assert_eq!(cost_first_syntax_error("(((((((((())))))))))"), 0);
        assert_eq!(cost_first_syntax_error("(]"), 57);
    }

    #[test]
    fn test_parts() {
        let input = r#"
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
      "#;
        assert_eq!(part_1(input), 26397);
        assert_eq!(part_2(input), 288957);
    }
}
