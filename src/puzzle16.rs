use crate::puzzle::AbstractPuzzle;

pub struct Puzzle16 {
    input: Vec<bool>,
}

impl AbstractPuzzle for Puzzle16 {
    fn get_day(&self) -> u8 {
        16
    }

    fn solve_part_1(&self) -> String {
        let mut input = self.input.clone();
        let packet = parse_packet(&mut input);
        version_sum(&packet).to_string()
    }

    fn solve_part_2(&self) -> String {
        let mut input = self.input.clone();
        let packet = parse_packet(&mut input);
        evaluate(&packet).to_string()
    }
}

impl Puzzle16 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        Box::new(Puzzle16 {
            input: parse_input(input),
        })
    }
}

fn version_sum(packet: &Packet) -> u64 {
    let mut sum: u64 = packet.version as u64;
    for sub_packet in packet.sub_packets.iter() {
        sum += version_sum(sub_packet);
    }
    sum
}

fn evaluate(packet: &Packet) -> u64 {
    return match packet.type_id {
        0 => packet.sub_packets.iter().map(evaluate).sum(),
        1 => packet.sub_packets.iter().map(evaluate).product(),
        2 => packet.sub_packets.iter().map(evaluate).min().unwrap(),
        3 => packet.sub_packets.iter().map(evaluate).max().unwrap(),
        4 => packet.payload,
        5 => {
            if evaluate(&packet.sub_packets[0]) > evaluate(&packet.sub_packets[1]) {
                1
            } else {
                0
            }
        }
        6 => {
            if evaluate(&packet.sub_packets[0]) < evaluate(&packet.sub_packets[1]) {
                1
            } else {
                0
            }
        }
        7 => {
            if evaluate(&packet.sub_packets[0]) == evaluate(&packet.sub_packets[1]) {
                1
            } else {
                0
            }
        }
        _ => panic!("Unknown packet type"),
    };
}

fn parse_input(input: &str) -> Vec<bool> {
    let mut binary_input: Vec<bool> = Vec::new();
    for c1 in input.trim().chars() {
        match c1 {
            '0' => binary_input.extend(vec![false, false, false, false]),
            '1' => binary_input.extend(vec![false, false, false, true]),
            '2' => binary_input.extend(vec![false, false, true, false]),
            '3' => binary_input.extend(vec![false, false, true, true]),
            '4' => binary_input.extend(vec![false, true, false, false]),
            '5' => binary_input.extend(vec![false, true, false, true]),
            '6' => binary_input.extend(vec![false, true, true, false]),
            '7' => binary_input.extend(vec![false, true, true, true]),
            '8' => binary_input.extend(vec![true, false, false, false]),
            '9' => binary_input.extend(vec![true, false, false, true]),
            'A' => binary_input.extend(vec![true, false, true, false]),
            'B' => binary_input.extend(vec![true, false, true, true]),
            'C' => binary_input.extend(vec![true, true, false, false]),
            'D' => binary_input.extend(vec![true, true, false, true]),
            'E' => binary_input.extend(vec![true, true, true, false]),
            'F' => binary_input.extend(vec![true, true, true, true]),
            _ => panic!("Invalid input"),
        }
    }
    binary_input
}

struct Packet {
    version: u8,
    type_id: u8,
    payload: u64,
    sub_packets: Vec<Packet>,
}

fn parse_packet(input: &mut Vec<bool>) -> Packet {
    let version = read_bits(input, 3) as u8;
    let type_id = read_bits(input, 3) as u8;
    if type_id == 4 {
        let mut payload: u64 = 0;
        loop {
            let x = read_bits(input, 5) as u8;
            payload <<= 4;
            payload |= (x & 15) as u64;
            if (x & 16) == 0 {
                break;
            }
        }
        Packet {
            version,
            type_id,
            payload,
            sub_packets: Vec::new(),
        }
    } else {
        let length_type_id = read_bits(input, 1) == 1;
        if length_type_id {
            let payload = read_bits(input, 11);
            let mut sub_packets: Vec<Packet> = Vec::new();
            for _ in 0..payload {
                sub_packets.push(parse_packet(input));
            }
            Packet {
                version,
                type_id,
                payload,
                sub_packets,
            }
        } else {
            let payload = read_bits(input, 15);
            let mut payload_bits: Vec<bool> = Vec::new();
            for _ in 0..payload {
                payload_bits.push(input.remove(0));
            }
            let mut sub_packets: Vec<Packet> = Vec::new();
            while !payload_bits.is_empty() {
                sub_packets.push(parse_packet(&mut payload_bits));
            }
            Packet {
                version,
                type_id,
                payload,
                sub_packets,
            }
        }
    }
}

fn read_bits(input: &mut Vec<bool>, size: usize) -> u64 {
    let mut result: u64 = 0;
    for _ in 0..size {
        result <<= 1;
        if input.remove(0) {
            result |= 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::puzzle16::{parse_input, parse_packet, Puzzle16};
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_single_digit_input_parsing() {
        assert_eq!(parse_input("0"), vec![false, false, false, false]);
        assert_eq!(parse_input("1"), vec![false, false, false, true]);
        assert_eq!(parse_input("2"), vec![false, false, true, false]);
        assert_eq!(parse_input("3"), vec![false, false, true, true]);
        assert_eq!(parse_input("4"), vec![false, true, false, false]);
        assert_eq!(parse_input("5"), vec![false, true, false, true]);
        assert_eq!(parse_input("6"), vec![false, true, true, false]);
        assert_eq!(parse_input("7"), vec![false, true, true, true]);
        assert_eq!(parse_input("8"), vec![true, false, false, false]);
        assert_eq!(parse_input("9"), vec![true, false, false, true]);
        assert_eq!(parse_input("A"), vec![true, false, true, false]);
        assert_eq!(parse_input("B"), vec![true, false, true, true]);
        assert_eq!(parse_input("C"), vec![true, true, false, false]);
        assert_eq!(parse_input("D"), vec![true, true, false, true]);
        assert_eq!(parse_input("E"), vec![true, true, true, false]);
        assert_eq!(parse_input("F"), vec![true, true, true, true]);
    }

    #[test]
    fn test_double_digit_input_parsing() {
        assert_eq!(
            parse_input("00"),
            vec![false, false, false, false, false, false, false, false]
        );
        assert_eq!(
            parse_input("22"),
            vec![false, false, true, false, false, false, true, false]
        );
    }

    #[test]
    fn test_example_packet_1() {
        let mut input = parse_input("D2FE28");
        let packet = parse_packet(&mut input);
        assert_eq!(packet.version, 6);
        assert_eq!(packet.type_id, 4);
        assert_eq!(packet.payload, 2021);
    }

    #[test]
    fn test_example_packet_2() {
        let mut input = parse_input("38006F45291200");
        let packet = parse_packet(&mut input);
        assert_eq!(packet.version, 1);
        assert_eq!(packet.type_id, 6);
        assert_eq!(packet.payload, 27);
        assert_eq!(packet.sub_packets.len(), 2);
        assert_eq!(packet.sub_packets[0].type_id, 4);
        assert_eq!(packet.sub_packets[0].payload, 10);
        assert_eq!(packet.sub_packets[1].type_id, 4);
        assert_eq!(packet.sub_packets[1].payload, 20);
    }

    #[test]
    fn test_example_packet_3() {
        let mut input = parse_input("EE00D40C823060");
        let packet = parse_packet(&mut input);
        assert_eq!(packet.version, 7);
        assert_eq!(packet.type_id, 3);
        assert_eq!(packet.payload, 3);
        assert_eq!(packet.sub_packets.len(), 3);
        assert_eq!(packet.sub_packets[0].type_id, 4);
        assert_eq!(packet.sub_packets[0].payload, 1);
        assert_eq!(packet.sub_packets[1].type_id, 4);
        assert_eq!(packet.sub_packets[1].payload, 2);
        assert_eq!(packet.sub_packets[2].type_id, 4);
        assert_eq!(packet.sub_packets[2].payload, 3);
    }

    #[test]
    fn test_example_packet_4() {
        let mut input = parse_input("8A004A801A8002F478");
        let packet = parse_packet(&mut input);
        assert_eq!(packet.version, 4);
        assert_ne!(packet.type_id, 4);
        assert_eq!(packet.sub_packets.len(), 1);
        assert_eq!(packet.sub_packets[0].version, 1);
        assert_ne!(packet.sub_packets[0].type_id, 4);
        assert_eq!(packet.sub_packets[0].sub_packets.len(), 1);
        assert_eq!(packet.sub_packets[0].sub_packets[0].version, 5);
        assert_ne!(packet.sub_packets[0].sub_packets[0].type_id, 4);
        assert_eq!(packet.sub_packets[0].sub_packets[0].sub_packets.len(), 1);
        assert_eq!(
            packet.sub_packets[0].sub_packets[0].sub_packets[0].version,
            6
        );
        assert_eq!(
            packet.sub_packets[0].sub_packets[0].sub_packets[0].type_id,
            4
        );
        assert_eq!(
            packet.sub_packets[0].sub_packets[0].sub_packets[0]
                .sub_packets
                .len(),
            0
        );
    }

    #[test]
    fn test_example_packet_5() {
        let mut input = parse_input("620080001611562C8802118E34");
        let packet = parse_packet(&mut input);
        assert_eq!(packet.version, 3);
        assert_ne!(packet.type_id, 4);
        assert_eq!(packet.sub_packets.len(), 2);
        assert_ne!(packet.sub_packets[0].type_id, 4);
        assert_eq!(packet.sub_packets[0].sub_packets.len(), 2);
        assert!(packet.sub_packets[0]
            .sub_packets
            .iter()
            .all(|p| p.type_id == 4));
        assert!(packet.sub_packets[0]
            .sub_packets
            .iter()
            .all(|p| p.sub_packets.len() == 0));
        assert_ne!(packet.sub_packets[1].type_id, 4);
        assert_eq!(packet.sub_packets[1].sub_packets.len(), 2);
        assert!(packet.sub_packets[1]
            .sub_packets
            .iter()
            .all(|p| p.type_id == 4));
        assert!(packet.sub_packets[1]
            .sub_packets
            .iter()
            .all(|p| p.sub_packets.len() == 0));
    }

    #[test]
    fn test_example_packet_6() {
        let mut input = parse_input("C0015000016115A2E0802F182340");
        let packet = parse_packet(&mut input);
        assert_ne!(packet.type_id, 4);
        assert_eq!(packet.sub_packets.len(), 2);
        assert_ne!(packet.sub_packets[0].type_id, 4);
        assert_eq!(packet.sub_packets[0].sub_packets.len(), 2);
        assert!(packet.sub_packets[0]
            .sub_packets
            .iter()
            .all(|p| p.type_id == 4));
        assert!(packet.sub_packets[0]
            .sub_packets
            .iter()
            .all(|p| p.sub_packets.len() == 0));
        assert_ne!(packet.sub_packets[1].type_id, 4);
        assert_eq!(packet.sub_packets[1].sub_packets.len(), 2);
        assert!(packet.sub_packets[1]
            .sub_packets
            .iter()
            .all(|p| p.type_id == 4));
        assert!(packet.sub_packets[1]
            .sub_packets
            .iter()
            .all(|p| p.sub_packets.len() == 0));
    }

    #[test]
    fn test_example_packet_7() {
        let mut input = parse_input("A0016C880162017C3686B18A3D4780");
        let packet = parse_packet(&mut input);
        assert_ne!(packet.type_id, 4);
        assert_eq!(packet.sub_packets.len(), 1);
        assert_ne!(packet.sub_packets[0].type_id, 4);
        assert_eq!(packet.sub_packets[0].sub_packets.len(), 1);
        assert_ne!(packet.sub_packets[0].sub_packets[0].type_id, 4);
        assert_eq!(packet.sub_packets[0].sub_packets[0].sub_packets.len(), 5);
        assert!(packet.sub_packets[0].sub_packets[0]
            .sub_packets
            .iter()
            .all(|p| p.type_id == 4));
        assert!(packet.sub_packets[0].sub_packets[0]
            .sub_packets
            .iter()
            .all(|p| p.sub_packets.len() == 0));
    }

    #[test]
    fn test_part_1_example_1() {
        let input = "8A004A801A8002F478";
        let puzzle = Puzzle16::create(&input);
        assert_eq!(puzzle.solve_part_1(), "16");
    }

    #[test]
    fn test_part_1_example_2() {
        let input = "620080001611562C8802118E34";
        let puzzle = Puzzle16::create(&input);
        assert_eq!(puzzle.solve_part_1(), "12");
    }

    #[test]
    fn test_part_1_example_3() {
        let input = "C0015000016115A2E0802F182340";
        let puzzle = Puzzle16::create(&input);
        assert_eq!(puzzle.solve_part_1(), "23");
    }

    #[test]
    fn test_part_1_example_4() {
        let input = "A0016C880162017C3686B18A3D4780";
        let puzzle = Puzzle16::create(&input);
        assert_eq!(puzzle.solve_part_1(), "31");
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/16")).unwrap();
        let puzzle = Puzzle16::create(&input);
        assert_eq!(puzzle.solve_part_1(), "883");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "C200B40A82";
        let puzzle = Puzzle16::create(&input);
        assert_eq!(puzzle.solve_part_2(), "3");
    }

    #[test]
    fn test_part_2_example_2() {
        let input = "04005AC33890";
        let puzzle = Puzzle16::create(&input);
        assert_eq!(puzzle.solve_part_2(), "54");
    }

    #[test]
    fn test_part_2_example_3() {
        let input = "880086C3E88112";
        let puzzle = Puzzle16::create(&input);
        assert_eq!(puzzle.solve_part_2(), "7");
    }

    #[test]
    fn test_part_2_example_4() {
        let input = "CE00C43D881120";
        let puzzle = Puzzle16::create(&input);
        assert_eq!(puzzle.solve_part_2(), "9");
    }

    #[test]
    fn test_part_2_example_5() {
        let input = "D8005AC2A8F0";
        let puzzle = Puzzle16::create(&input);
        assert_eq!(puzzle.solve_part_2(), "1");
    }

    #[test]
    fn test_part_2_example_6() {
        let input = "F600BC2D8F";
        let puzzle = Puzzle16::create(&input);
        assert_eq!(puzzle.solve_part_2(), "0");
    }

    #[test]
    fn test_part_2_example_7() {
        let input = "9C005AC2F8F0";
        let puzzle = Puzzle16::create(&input);
        assert_eq!(puzzle.solve_part_2(), "0");
    }

    #[test]
    fn test_part_2_example_8() {
        let input = "9C0141080250320F1802104A08";
        let puzzle = Puzzle16::create(&input);
        assert_eq!(puzzle.solve_part_2(), "1");
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/16")).unwrap();
        let puzzle = Puzzle16::create(&input);
        assert_eq!(puzzle.solve_part_2(), "1675198555015");
    }
}
