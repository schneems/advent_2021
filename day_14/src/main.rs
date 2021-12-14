use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let out = part_1(include_str!("../input.txt"));
    println!("part_1: {}", out);

    let out = part_2(include_str!("../input.txt"));
    println!("part_2: {}", out);
}

fn part_1(input: &str) -> u64 {
    let mut poly = parse(input);
    for _ in 0..10 {
        step(&mut poly);
    }
    poly.score()
}

fn part_2(input: &str) -> u64 {
    let mut poly = parse(input);
    for _ in 0..40 {
        step_2(&mut poly);
    }
    poly.score_2()
}

type TupleCount = HashMap<(String, String), u64>;

fn step_2(poly: &mut Polymer) {
    let mut new_tuple_count = TupleCount::new();
    for (tuple, _) in &poly.rules {
        let count = poly.tuple_count.get(&tuple).or(Some(&0)).unwrap();

        let (a, b) = poly.next_tuple(tuple.clone());
        let entry_a = new_tuple_count.entry(a.clone()).or_insert(0);
        *entry_a += count;

        let entry_b = new_tuple_count.entry(b.clone()).or_insert(0);
        *entry_b += count;
    }
    poly.tuple_count = new_tuple_count;
}

type Hyperhash = HashMap<(String, String), String>;

struct Polymer {
    chain: Vec<String>,
    rules: Hyperhash,
    tuple_count: TupleCount,
}

impl Polymer {
    fn score(&self) -> u64 {
        let hash = self.chain.iter().counts_by(|x| x);
        let largest = hash.iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap().1;

        let smallest = hash.iter().min_by(|(_, a), (_, b)| a.cmp(b)).unwrap().1;

        (largest - smallest) as u64
    }

    fn score_2(&self) -> u64 {
        let mut count_map = HashMap::new();
        for ((a, _), count) in &self.tuple_count {
            *count_map.entry(a).or_insert(0) += count;
            // *count_map.entry(b).or_insert(0) += count;
        }
        let last_char = self.chain.iter().last().unwrap();
        *count_map.entry(last_char).or_insert(0) += 1;

        let largest = count_map
            .iter()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
            .1;

        let smallest = count_map
            .iter()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
            .1;

        (largest - smallest) as u64
    }

    fn next_tuple(&self, tuple: (String, String)) -> ((String, String), (String, String)) {
        (
            (tuple.0.clone(), self.rules.get(&tuple).unwrap().to_string()),
            (self.rules.get(&tuple).unwrap().to_string(), tuple.1.clone()),
        )
    }
}

fn parse(input: &str) -> Polymer {
    let mut lines = input.trim().lines().into_iter();

    let chain = lines
        .next()
        .unwrap()
        .chars()
        .map(String::from)
        .collect::<Vec<String>>();

    lines.next();

    let rules = lines
        .map(|line| {
            let (from, to) = line.split_once(" -> ").unwrap();

            (
                from.chars()
                    .take(2)
                    .map(String::from)
                    .collect_tuple::<(String, String)>()
                    .unwrap(),
                String::from(to),
            )
        })
        .collect::<Hyperhash>();

    let mut tuple_count = TupleCount::new();

    let mut chain_iter = chain.iter().peekable();
    while let Some(a) = chain_iter.next() {
        if let Some(b) = chain_iter.peek() {
            let val = tuple_count
                .entry((a.to_string(), b.to_string()))
                .or_insert(0);
            *val += 1;
        }
    }

    Polymer {
        chain,
        rules,
        tuple_count,
    }
}

fn step(polymer: &mut Polymer) {
    let mut chain = polymer.chain.iter().peekable();
    let mut new_chain: Vec<String> = Vec::new();
    while let Some(one) = chain.next() {
        let one = one.to_string();
        new_chain.push(one.clone());
        if let Some(two) = chain.peek() {
            let two = two.to_string();
            if let Some(val) = polymer.rules.get(&(one, two.clone())) {
                new_chain.push(val.to_string());
            }
        }
    }

    polymer.chain = new_chain;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r#"
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
      "#;

        let mut polymer = parse(input);
        step(&mut polymer);
        assert_eq!(&polymer.chain.clone().join(""), "NCNBCHB");
        step(&mut polymer);
        assert_eq!(&polymer.chain.clone().join(""), "NBCCNBBBCBHCB");
        step(&mut polymer);
        assert_eq!(&polymer.chain.clone().join(""), "NBBBCNCCNBBNBNBBCHBHHBCHB");

        step(&mut polymer);
        assert_eq!(
            &polymer.chain.clone().join(""),
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB",
        );

        let mut polymer = parse(input);
        for _ in 0..10 {
            step(&mut polymer);
        }
        assert_eq!(&polymer.score(), &1588);

        let mut polymer = parse(input);
        step_2(&mut polymer);
        assert_eq!(
            polymer
                .tuple_count
                .get(&(String::from("H"), String::from("B"))),
            Some(&1)
        );

        let mut polymer = parse(input);
        for _ in 0..40 {
            step_2(&mut polymer);
        }
        assert_eq!(&polymer.score_2(), &2188189693529);
    }
}
