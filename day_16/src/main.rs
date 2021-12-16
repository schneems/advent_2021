fn main() {
    let out = part_1(include_str!("../input.txt"));
    println!("part_1: {}", out);
}

fn part_1(input: &str) -> u64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
      let input = r#""#;
      assert_eq!(part_1(input), 99);
    }
}
