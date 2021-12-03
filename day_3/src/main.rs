
fn main() {
    let out = diagnostic(include_str!("../input_1.txt"));
    println!("Answer: {}", out);
}

fn diagnostic(input: &str) -> u64 {
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

    println!("one counts: {:?}", one_count);
    println!("zero counts: {:?}", zero_count);

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

    println!("gamma: {:?}", gamma);
    println!("epsilon: {:?}", epsilon);
    (gamma * epsilon).try_into().unwrap()
}

fn convert_string_binary(string: &str) -> isize {
    isize::from_str_radix(string, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_into_string(){
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
}
