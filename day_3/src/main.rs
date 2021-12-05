fn main() {
    let out = life_support(include_str!("../input_1.txt"));
    println!("Answer: {}", out);
}

fn life_support(input: &str) -> isize {
    let o2 = convert_string_binary(filter_oxygen(input));
    let co2 = convert_string_binary(filter_co2(input));
    o2 * co2
}

fn diagnostic(input: &str) -> u64 {
    let (zero_count, one_count) = count_digits(input);

    let mut gamma_array: Vec<&str> = Vec::new();
    let mut epsilon_array: Vec<&str> = Vec::new();
    if let Some(line) = input.lines().next() {
        for i in 0..line.len() {
            if one_count.get(i).unwrap() > zero_count.get(i).unwrap() {
                gamma_array.push("1");
                epsilon_array.push("0");
            } else {
                gamma_array.push("0");
                epsilon_array.push("1");
            }
        }
    }
    let gamma = convert_string_binary(&gamma_array.join(""));
    let epsilon = convert_string_binary(&epsilon_array.join(""));

    (gamma * epsilon).try_into().unwrap()
}

fn count_digits(input: &str) -> (Vec<u64>, Vec<u64>) {
    let some_one = "1".chars().nth(0).unwrap();

    let mut one_count: Vec<u64> = Vec::new();
    let mut zero_count: Vec<u64> = Vec::new();

    if let Some(line) = input.lines().next() {
        for _i in 0..line.len() {
            one_count.push(0);
            zero_count.push(0);
        }
    }

    for line in input.lines() {
        for i in 0..line.len() {
            let c = line.chars().nth(i).unwrap();
            if c == some_one {
                let value = one_count.get_mut(i).unwrap();
                *value += 1;
            } else {
                let value = zero_count.get_mut(i).unwrap();
                *value += 1;
            }
        }
    }

    (zero_count, one_count)
}

fn convert_string_binary(string: &str) -> isize {
    isize::from_str_radix(string, 2).unwrap()
}

fn filter_oxygen(input: &str) -> &str {
    let mut frontier: Vec<&str> = input.lines().collect::<Vec<&str>>();

    let mut index = 0;
    while frontier.len() != 1 {
        let (zero_counts, one_counts) = count_digits(&frontier.join("\n"));
        let zero_count = *zero_counts.get(index).unwrap();
        let one_count = *one_counts.get(index).unwrap();

        frontier.retain(|line| keep_oxygen_line(one_count, zero_count, line, index));
        index += 1;
    }

    frontier.last().unwrap()
}

fn filter_co2(input: &str) -> &str {
    let mut frontier: Vec<&str> = input.lines().collect::<Vec<&str>>();

    let mut index = 0;
    while frontier.len() != 1 {
        let (zero_counts, one_counts) = count_digits(&frontier.join("\n"));
        let zero_count = *zero_counts.get(index).unwrap();
        let one_count = *one_counts.get(index).unwrap();

        frontier.retain(|line| keep_co2_line(one_count, zero_count, line, index));
        index += 1;
    }

    frontier.last().unwrap()
}

fn keep_oxygen_line(one_count: u64, zero_count: u64, line: &str, i: usize) -> bool {
    let some_one = "1".chars().nth(0).unwrap();

    match one_count.cmp(&zero_count) {
        // There are more zeroes than ones
        std::cmp::Ordering::Less => {
            // If line is a one, then stop
            if line.chars().nth(i).unwrap() == some_one {
                false
            } else {
                true
            }
        }
        std::cmp::Ordering::Equal => {
            // choose 1 when oxygen
            // choose 0 when co2
            if line.chars().nth(i).unwrap() == some_one {
                true
            } else {
                false
            }
        }
        // There are more ones than zeroes
        std::cmp::Ordering::Greater => {
            // If line is a one, then continue
            if line.chars().nth(i).unwrap() == some_one {
                true
            } else {
                false
            }
        }
    }
}

fn keep_co2_line(one_count: u64, zero_count: u64, line: &str, i: usize) -> bool {
    !keep_oxygen_line(one_count, zero_count, line, i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_into_string() {
        let string = "foo";
        assert_eq!(string.chars().nth(0), "f".chars().nth(0))
    }

    #[test]
    fn test_convert_string_binary() {
        let out = convert_string_binary("01001");
        assert_eq!(out, 9);
    }

    #[test]
    fn part_1() {
        let out = diagnostic(
            r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#,
        );
        assert_eq!(out, 198)
    }

    #[test]
    fn part_2() {
        let out = life_support(
            r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#,
        );
        assert_eq!(out, 230)
    }

    #[test]
    fn part_2_oxygen() {
        let out = filter_oxygen(
            r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#,
        );
        assert_eq!(out, "10111")
    }

    #[test]
    fn part_2_co2() {
        let out = filter_co2(
            r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#,
        );
        assert_eq!(out, "01010")
    }

    #[test]
    fn test_is_oxygen_line() {
        let line = "10111";
        assert!(keep_oxygen_line(7, 5, line, 0));
        assert!(keep_oxygen_line(3, 4, line, 1));
        assert!(keep_oxygen_line(4, 1, line, 2));
        assert!(keep_oxygen_line(2, 1, line, 3));
        assert!(keep_oxygen_line(1, 1, line, 4));

        let line = "10110";
        assert!(keep_oxygen_line(7, 5, line, 0));
        assert!(keep_oxygen_line(3, 4, line, 1));
        assert!(keep_oxygen_line(4, 1, line, 2));
        assert!(keep_oxygen_line(2, 1, line, 3));
        assert!(!keep_oxygen_line(1, 1, line, 4));
    }

    #[test]
    fn test_is_co2_line() {
        let line = "01010";
        assert!(keep_co2_line(7, 5, line, 0));
        assert!(keep_co2_line(2, 3, line, 1));
        assert!(keep_co2_line(1, 1, line, 2));
    }
}
