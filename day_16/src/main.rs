// use std::collections::HashMap;
use std::str::FromStr;
// use std::str::Split;

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

struct DataPacket {
    version: u8,
    type_id: u8,
    string: String,
    literal: u16,
}

struct OperatorPacket {
    version: u8,
    type_id: u8,
    string: String,
    packet_count: u32, // sub_packets: Vec<DataPacket>,
    len: u32,
}

// impl OperatorPacket {
//     fn len(&self) -> u64 {
//         self.sub_packets
//         .iter()
//         .map(|p| p.literal)

//     }
// }

// struct Operator {
//     packets: Vec<Packet>,
//     type_id: u8,
// }

fn parse(input: &str) {
    unimplemented!()
}

// fn parse_data(input: &str) -> DataPacket {
//     // let foo  = 0b001;
//     // let blerg = "0b110".parse().unwrap();
//     DataPacket {
//         version: 1,
//         type_id: 1,
//         string: String,
//     }
// }

use itertools::Itertools;

impl FromStr for OperatorPacket {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let string = string.to_string();
        let packet = DataPacket::from_str(&string).unwrap();
        let mut characters = string.trim().chars().map(String::from);
        let bit_length = match characters
            .nth(6)
            .unwrap_or_else(|| panic!("string too short {}", &string))
            .as_str()
        {
            "0" => 15,
            "1" => 11,
            _ => panic!("Not binary {}", &string),
        };
        let packet_count_str = characters.take(bit_length).join("");
        let packet_count = u32::from_str_radix(&packet_count_str, 2).expect("Not bin");
        Ok(OperatorPacket {
            version: packet.version,
            type_id: packet.type_id,
            string: string,
            packet_count: packet_count,
            len: 0,
        })
    }
}

enum Packet {
    OperatorPacket,
    DataPacket,
}

impl FromStr for DataPacket {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let string = string.to_string();
        let mut characters = string.trim().chars().map(String::from);

        let version_str = characters.clone().take(3).collect::<Vec<String>>().join("");
        let version = u8::from_str_radix(&version_str, 2).expect("Not a binary number!");

        let type_str = characters
            .clone()
            .skip(3)
            .take(3)
            .collect::<Vec<String>>()
            .join("");
        let type_id = u8::from_str_radix(&type_str, 2).expect("Not a binary number!");

        let mut characters = characters.skip(6);

        let mut literal_vec = Vec::new();
        for mut chunk in &characters.chunks(5) {
            if let Some(c) = chunk.next() {
                literal_vec.push(chunk.join(""));
                if c.as_str() == "0" {
                    break;
                }
            }
        }
        let literal = u16::from_str_radix(&literal_vec.join(""), 2).expect("Not a binary number!");

        Ok(DataPacket {
            version: version,
            type_id: type_id,
            string: string,
            literal: literal,
        })
    }
}

fn data_to_binary_string(input: &str) -> String {
    input
        .trim()
        .chars()
        .map(String::from)
        .map(char_to_string_binary)
        .collect::<Vec<&str>>()
        .join("")
        .split("")
        .map(|s| String::from(s))
        .collect::<Vec<String>>()
        .join("")
}

fn char_to_string_binary(c: impl AsRef<str>) -> &'static str {
    match c.as_ref() {
        "0" => "0000",
        "1" => "0001",
        "2" => "0010",
        "3" => "0011",
        "4" => "0100",
        "5" => "0101",
        "6" => "0110",
        "7" => "0111",
        "8" => "1000",
        "9" => "1001",
        "A" => "1010",
        "B" => "1011",
        "C" => "1100",
        "D" => "1101",
        "E" => "1110",
        "F" => "1111",
        _ => panic!("Not valid char '{:?}'", c.as_ref()),
    }
}

// Rust binary string to integer
// https://www.programming-idioms.org/idiom/251/parse-binary-digits
//
// let i = i32::from_str_radix(s, 2).expect("Not a binary number!");

// Integer to binary string
// https://www.programming-idioms.org/idiom/76/binary-digits-from-an-integer
//
// let s = format!("{:b}", 4);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lol() {
        let input = r#"38006F45291200"#;

        let out = data_to_binary_string(input);
        assert_eq!(
            out,
            String::from("00111000000000000110111101000101001010010001001000000000")
        );

        let packet =
            OperatorPacket::from_str("00111000000000000110111101000101001010010001001000000000")
                .unwrap();
        assert_eq!(packet.version, 1);
        assert_eq!(packet.type_id, 6);
        assert_eq!(packet.packet_count, 27);

        let packet =
            OperatorPacket::from_str("11101110000000001101010000001100100000100011000001100000")
                .unwrap();
        assert_eq!(packet.version, 7);
        assert_eq!(packet.type_id, 3);
        assert_eq!(packet.packet_count, 3);
    }

    #[test]
    fn test_blerg() {
        let input = r#"D2FE28"#;
        let out = data_to_binary_string(input);
        assert_eq!(out, String::from("110100101111111000101000"));

        let packet = DataPacket::from_str("110100101111111000101000").unwrap();
        assert_eq!(packet.version, 6);
        assert_eq!(packet.type_id, 4);
        assert_eq!(packet.literal, 2021);
        // let packet = parse_data(input);
    }

    // #[test]
    // fn test_parts() {
    //     let input = r#""#;
    //     assert_eq!(part_1(input), 99);
    //     // assert_eq!(part_2(input), 99);
    // }
}
