// use std::collections::HashMap;
use std::str::FromStr;

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
struct ALU {
    w: i16,
    x: i16,
    y: i16,
    z: i16,
}

fn inp(val: &mut i16, by: i16) -> bool {
    *val = by;
    true
}

fn add(val: &mut i16, by: i16) -> bool {
    *val += by;
    true
}

fn mul(val: &mut i16, by: i16) -> bool {
    *val *= by;
    true
}

fn div(val: &mut i16, by: i16) -> bool {
    if by == 0 {
        false
    } else {
        *val = num::integer::div_floor(*val, by);
        true
    }
}

fn modit(val: &mut i16, by: i16) -> bool {
    if val < &mut 0 || by <= 0 {
        false
    } else {
        *val %= by;
        true
    }
}

fn eql(val: &mut i16, by: i16) -> bool {
    if *val == by {
        *val = 1;
    } else {
        *val = 0;
    }
    true
}

#[derive(Clone, Debug)]
struct Command {
    function: FFS,
    val_char: Register,
    by_char: Register,
}

#[derive(Clone, Debug)]
enum FFS {
    INP,
    ADD,
    MUL,
    DIV,
    MOD,
    EQL,
}

fn ffs_from_str(string: &str) -> FFS {
    match string {
        "inp" => FFS::INP,
        "add" => FFS::ADD,
        "mul" => FFS::MUL,
        "div" => FFS::DIV,
        "mod" => FFS::MOD,
        "eql" => FFS::EQL,
        _ => panic!("nope {}", string),
    }
}

#[derive(Clone, Debug)]
enum Register {
    W,
    X,
    Y,
    Z,
    INPUT,
    VAL(i16),
}

fn register_from_str(string: &str) -> Register {
    match string {
        "w" => Register::W,
        "x" => Register::X,
        "y" => Register::Y,
        "z" => Register::Z,
        _ => Register::VAL(string.parse::<i16>().unwrap()),
    }
}

fn parse(input: &str) -> Vec<Vec<Command>> {
    let mut out = Vec::new();

    let mut lines = input.trim().lines().into_iter();

    let mut instructions: Vec<Command> = vec![];
    while let Some(line) = lines.next() {
        let mut parts = line.split(" ").into_iter();
        match parts.next() {
            Some(s) if s == "inp" => {
                if &instructions.len() > &0 {
                    out.push(instructions);
                }

                instructions = Vec::new();

                instructions.push(Command {
                    function: FFS::INP,
                    val_char: register_from_str(parts.next().unwrap()),
                    by_char: Register::INPUT,
                })
            }
            Some(s) => {
                //
                instructions.push(Command {
                    function: ffs_from_str(s),
                    val_char: register_from_str(parts.next().unwrap()),
                    by_char: register_from_str(parts.next().unwrap()),
                })
            }
            None => panic!("WAT"),
        }
    }

    if &instructions.len() > &0 {
        out.push(instructions);
    }
    out
}

fn main() {
    let out = part_1(include_str!("../input.txt"));
    println!("part_1: {}", out);

    let out = part_2(include_str!("../input.txt"));
    println!("part_2: {}", out);
}

fn part_1(input: &str) -> u64 {
    let instructions = parse(input);
    let mut max = 0;
    for x in (11111111111111..=39924989499970 as i64).rev() {
        let mut alu = ALU {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        };

        run_commands(&mut alu, &instructions, x);

        if alu.z == 0 {
            max = x;
            break;
        }
    }
    max.try_into().unwrap()
}

fn part_2(input: &str) -> u64 {
    let instructions = parse(input);
    let mut min = 0;
    let mut count = 0;
    for x in 16111111111111..=39924989499969 as i64 {
        count += 1;
        if count == 10_000_000 {
            count = 0;
            println!("{}", x);
        }
        let mut alu = ALU {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        };

        run_commands(&mut alu, &instructions, x);

        if alu.z == 0 {
            min = x;
            break;
        }
    }
    min.try_into().unwrap()
}

fn run_commands(alu: &mut ALU, instructions: &Vec<Vec<Command>>, x: i64) {
    let num_str = x.to_string();
    let mut parts = num_str.chars();
    if parts.clone().any(|x| x == '0') {
        alu.z = 1;
        return;
    }

    for instruction in instructions {
        let in_str = parts.next().unwrap().to_string();
        if run_mut(alu, &instruction, in_str.parse::<i16>().unwrap()) == false {
            alu.z = 1;
            break;
        }
    }
}

fn run_mut(alu: &mut ALU, commands: &Vec<Command>, input: i16) -> bool {
    for command in commands {
        let function = match command.function {
            FFS::INP => inp,
            FFS::MUL => mul,
            FFS::ADD => add,
            FFS::DIV => div,
            FFS::MOD => modit,
            FFS::EQL => eql,
        };

        let by = match command.by_char {
            Register::W => alu.w.clone(),
            Register::X => alu.x.clone(),
            Register::Y => alu.y.clone(),
            Register::Z => alu.z.clone(),
            Register::INPUT => input,
            Register::VAL(x) => x.clone(),
        };

        let val = match command.val_char {
            Register::W => &mut alu.w,
            Register::X => &mut alu.x,
            Register::Y => &mut alu.y,
            Register::Z => &mut alu.z,
            _ => panic!("nope {:?}", command.val_char),
        };

        if function(val, by) == false {
            return false;
        }
    }
    true
}

fn run(commands: &Vec<Command>, input: i16) -> ALU {
    let mut alu = ALU {
        w: 0,
        x: 0,
        y: 0,
        z: 0,
    };
    run_mut(&mut alu, commands, input);
    alu
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuuuuuuuuuuu() {
        let commands = parse(include_str!("../input.txt"));

        let mut stack: Vec<(usize, i16)> = Vec::new();
        let mut diff = [0; 14];
        for (i, instructions) in commands.iter().enumerate() {
            match &instructions[5].by_char {
                Register::VAL(x) if x < &0 => {
                    if let Register::VAL(this) = &instructions[5].by_char {
                        let (j, that) = stack.pop().unwrap();
                        // stack.push((j, that + this));
                        diff[i] = that + this;
                        diff[j] = -this - that;
                    }
                }
                _ => {
                    //push
                    if let Register::VAL(x) = &instructions[15].by_char {
                        stack.push((i, *x));
                    }
                }
            }
        }

        let mut answer = Vec::new();
        for i in 0..14 {
            let val = 9 + diff[i];
            if val < 9 {
                answer.push(val);
            } else {
                answer.push(9);
            }
        }

        println!(
            "Part 1: {:?}",
            answer
                .iter()
                .map(i16::to_string)
                .collect::<Vec<String>>()
                .join("")
        );

        let mut answer = Vec::new();
        for i in 0..14 {
            let val = 1 + diff[i];
            if val > 1 {
                answer.push(val);
            } else {
                answer.push(1);
            }
        }

        println!(
            "Part 2: {:?}",
            answer
                .iter()
                .map(i16::to_string)
                .collect::<Vec<String>>()
                .join("")
        );

        panic!("=========");
    }

    #[test]
    fn test_parse() {
        let out = parse(
            r#"
inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2
"#,
        );
        assert_eq!(
            run(&out.get(0).unwrap().to_vec(), 9),
            ALU {
                w: 1,
                x: 0,
                y: 0,
                z: 1,
            }
        );

        let out = parse(
            r#"
inp w
inp x
inp y
inp z
"#,
        );
        assert_eq!(out.len(), 4);
    }

    #[test]
    fn test_parts() {
        let commands = parse(include_str!("../input.txt"));
        let mut alu = ALU {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        };
        run_commands(&mut alu, &commands, 39924989499969);
        assert_eq!(alu.z, 0);
    }
}
