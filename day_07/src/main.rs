fn main() {
    let out = part_2(include_str!("../input_1.txt"));
    println!("{}", out);
}

fn median(numbers: &Vec<u32>) -> u32 {
    let mut numbers = numbers.clone();
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn parse(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(",")
        .map(str::parse::<u32>)
        .collect::<Result<_, _>>()
        .unwrap()
}

fn part_1(input: &str) -> u32 {
    let positions = parse(input);
    let target = median(&positions);

    let fuel = positions
        .iter()
        .map(|x| {
            let dist = if x > &target {
                x - &target
            } else {
                &target - x
            };
            dist
        })
        .sum();
    fuel
}

fn fuel_for(positions: &Vec<u32>, target: u32) -> u32 {
    let fuel = positions
        .iter()
        .map(|x| {
            let dist = if x > &target {
                x - &target
            } else {
                &target - x
            };
            (0..=dist).sum::<u32>()
        })
        .sum();
    fuel
}

fn part_2(input: &str) -> u32 {
    let positions = parse(input);
    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();

    let mut plans = Vec::new();
    for i in *min..*max {
        let f = fuel_for(&positions, i);
        plans.push(f)
    }

    *plans.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "16,1,2,0,4,2,7,1,2,14";

        let numbers = parse(&input);
        assert_eq!(median(&numbers), 2);

        assert_eq!(part_1(input), 37);
        assert_eq!(part_2(input), 168);
    }
}
