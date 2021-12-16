use itertools::Itertools;
use std::str::FromStr;

fn main() {
    let out = part_1(include_str!("../input.txt"));
    println!("part_1: {}", out);

    // let out = part_2(include_str!("../input.txt"));
    // println!("part_2: {}", out);
}

fn part_1(input: &str) -> u64 {
    let packet = parse(input);
    sum_versions(&packet)
}

fn part_2(input: &str) -> u64 {
    let _thing = parse(input);
    unimplemented!()
}

fn parse(input: &str) -> Packet {
    Packet::from_str(&data_to_binary_string(input)).unwrap()
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

fn sum_versions(packet: &Packet) -> u64 {
    let mut val = 0;
    match packet {
        Packet::Data(x) => val += x.header.version as u64,
        Packet::Operator(x) => {
            val += x.header.version as u64;
            for y in &x.packets {
                val += sum_versions(&y) as u64;
            }
        }
    }
    val as u64
}

fn len(packet: &Packet) -> u32 {
    match packet {
        Packet::Data(x) => x.len,
        Packet::Operator(x) => x.len,
    }
}

struct DataPacket {
    header: Header,
    literal: u64,
    len: u32,
}

struct OperatorPacket {
    header: Header,
    packet_len: u32,
    packets: Vec<Packet>,
    len: u32,
}

#[derive(Debug)]
struct Header {
    version: u8,
    type_id: u8,
}

impl FromStr for Header {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let characters = string.trim().chars().map(String::from);
        let version_str = characters.clone().take(3).collect::<Vec<String>>().join("");
        let version = u8::from_str_radix(&version_str, 2).expect("Not a binary number!");

        let type_str = characters
            .clone()
            .skip(3)
            .take(3)
            .collect::<Vec<String>>()
            .join("");
        let type_id = u8::from_str_radix(&type_str, 2).expect("Not a binary number!");

        Ok(Header { version, type_id })
    }
}

enum Packet {
    Operator(OperatorPacket),
    Data(DataPacket),
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let header = Header::from_str(string).unwrap();
        match header.type_id {
            4 => Ok(Packet::Data(DataPacket::from_str(string).unwrap())),
            _ => Ok(Packet::Operator(OperatorPacket::from_str(string).unwrap())),
        }
    }
}

impl FromStr for DataPacket {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        // println!("input str: {:?}", string);
        let string = string.to_string();
        let characters = string.trim().chars().map(String::from);

        let header = Header::from_str(&string).unwrap();
        println!("Data version {} type id {}", header.version, header.type_id);

        let characters = characters.skip(6);

        let mut literal_vec = Vec::new();
        for mut chunk in &characters.chunks(5) {
            if let Some(c) = chunk.next() {
                literal_vec.push(chunk.join(""));
                if c.as_str() == "0" {
                    break;
                }
            }
        }
        let literal = u64::from_str_radix(&literal_vec.join(""), 2).expect("Not a binary number!");

        println!("vec {:?}", literal_vec);
        println!("string: {:?} literal: {:?}", string, literal);
        println!("header {:?}", header);

        Ok(DataPacket {
            header: header,
            literal: literal,
            len: 3 + 3 + literal_vec.len() as u32 * 5,
        })
    }
}

impl FromStr for OperatorPacket {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        // println!("input str: {:?}", string);
        let string = string.to_string();
        let header = Header::from_str(&string).unwrap();

        println!(
            "Operator version {} type id {}",
            header.version, header.type_id
        );
        let characters = string.trim().chars().map(String::from);
        let bit_length = match characters
            .clone()
            .nth(6)
            .unwrap_or_else(|| panic!("string too short {}", &string))
            .as_str()
        {
            "0" => 15,
            "1" => 11,
            _ => panic!("Not binary {}", &string),
        } as u32;
        let mut packets = Vec::new();

        let packet_len_str = characters
            .clone()
            .skip(7)
            .take(bit_length as usize)
            .join("");
        let packet_len = u32::from_str_radix(&packet_len_str, 2).expect("Not bin");

        match bit_length {
            15 => {
                let packet_chars = characters
                    .clone()
                    .skip(7 + bit_length as usize)
                    .take(packet_len as usize);
                let mut offset = 0;
                while offset < packet_len {
                    let packet =
                        Packet::from_str(&packet_chars.clone().skip(offset as usize).join(""))
                            .unwrap();
                    offset += len(&packet);
                    packets.push(packet);
                }

                Ok(OperatorPacket {
                    header: header,
                    packet_len: packet_len,
                    packets: packets,
                    len: 3 + 3 + 1 + bit_length + packet_len,
                })
            }
            11 => {
                let packet_chars = characters.clone().skip(7 + bit_length as usize);
                let mut offset = 0;
                for _ in 0..packet_len {
                    let s = packet_chars.clone().skip(offset as usize).join("");
                    let packet = Packet::from_str(&s).unwrap();
                    offset += len(&packet);
                    packets.push(packet);
                }
                Ok(OperatorPacket {
                    header: header,
                    packet_len: packet_len,
                    packets: packets,
                    len: 3 + 3 + 1 + bit_length + offset as u32,
                })
            }
            _ => panic!("Unknown bit length {}", bit_length),
        }
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
    fn test_sum_versions() {
        let packet = Packet::from_str(&data_to_binary_string("8A004A801A8002F478")).unwrap();
        assert_eq!(sum_versions(&packet), 16);

        let packet = Packet::from_str(&data_to_binary_string(
            //
            "620080001611562C8802118E34",
        ))
        .unwrap();
        assert_eq!(sum_versions(&packet), 12);

        let packet = Packet::from_str(&data_to_binary_string(
            //
            "C0015000016115A2E0802F182340",
        ))
        .unwrap();
        assert_eq!(sum_versions(&packet), 23);

        let packet = Packet::from_str(&data_to_binary_string(
            //
            "A0016C880162017C3686B18A3D4780",
        ))
        .unwrap();
        assert_eq!(sum_versions(&packet), 31);
    }

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
        assert_eq!(packet.header.version, 1);
        assert_eq!(packet.header.type_id, 6);
        assert_eq!(packet.packet_len, 27);
        assert_eq!(
            packet.len,
            "0011100000000000011011110100010100101001000100100"
                .chars()
                .count() as u32
        );
        if let Packet::Data(x) = &packet.packets[0] {
            assert_eq!(x.literal, 10)
        } else {
            panic!("nope");
        }

        if let Packet::Data(x) = &packet.packets[1] {
            assert_eq!(x.literal, 20)
        } else {
            panic!("nope");
        }

        assert!(&packet.packets.get(2).is_none());

        let packet =
            OperatorPacket::from_str("11101110000000001101010000001100100000100011000001100000")
                .unwrap();
        assert_eq!(packet.header.version, 7);
        assert_eq!(packet.header.type_id, 3);
        assert_eq!(packet.packet_len, 3);

        assert_eq!(
            packet.len,
            "111011100000000011010100000011001000001000110000011"
                .chars()
                .count() as u32
        );
    }

    #[test]
    fn test_blerg() {
        let input = r#"D2FE28"#;
        let out = data_to_binary_string(input);
        assert_eq!(out, String::from("110100101111111000101000"));

        let packet = DataPacket::from_str("110100101111111000101000").unwrap();
        assert_eq!(packet.header.version, 6);
        assert_eq!(packet.header.type_id, 4);
        assert_eq!(packet.literal, 2021);
        assert_eq!(packet.len, "VVVTTTAAAAABBBBBCCCCC".chars().count() as u32);
        // let packet = parse_data(input);
    }

    // #[test]
    // fn test_parts() {
    //     let input = r#""#;
    //     assert_eq!(part_1(input), 99);
    //     // assert_eq!(part_2(input), 99);
    // }
}
