#!/usr/bin/env ruby

require 'pathname'

def run!(cmd)
  out = `#{cmd}`
  raise "Failed: '#{cmd}' out: #{out}" unless $?.success?
  out
end

def cargo_new(dir)
  run!("cargo new #{dir}")
  dir.join("src/main.rs").write(<<~'EOM')
// use std::collections::HashMap;
// use std::str::FromStr;

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

fn parse(input: &str) {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        let input = r#""#;
        assert_eq!(part_1(input), 99);
        // assert_eq!(part_2(input), 99);
    }
}

  EOM
end

def download_input(dir, day_int: )
  run!(%Q{curl -b session="#{dir.join("../.aocrc").read.strip}" https://adventofcode.com/2021/day/#{day_int}/input > #{dir.join('input.txt')}})
end

def calculate_next_day(aoc_dir)
  aoc_dir.children.sort.map do |f|
    next if !f.directory?
    f.basename.to_s.split("_").last.to_i
  end.compact.last + 1
end

def call
  aoc_dir = Pathname.new(__dir__)
  next_day_int = calculate_next_day(aoc_dir)
  puts "Next day is #{next_day_int}"

  day_dir = aoc_dir.join("day_#{next_day_int.to_s.rjust(2, "0")}")
  cargo_new(day_dir)
  download_input(day_dir, day_int: next_day_int)
end

call()
