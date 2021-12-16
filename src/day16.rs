use itertools::Itertools;

#[derive(Debug)]
struct Packet {
    version: usize,
    type_id: usize,
    v: usize,
    operators: Vec<Packet>,
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn convert_to_binary_from_hex(hex: &str) -> Vec<char> {
    hex.chars()
        .map(to_binary)
        .collect::<String>()
        .chars()
        .collect_vec()
}

fn convert(bytes: &[char]) -> usize {
    let s: String = bytes.into_iter().collect();
    usize::from_str_radix(s.as_str(), 2).unwrap()
}

fn parse_number(bytes: &[char]) -> (usize, usize) {
    let mut pos = 0;
    let mut res = Vec::new();
    loop {
        if bytes[pos] == '0' {
            res.extend_from_slice(&bytes[pos + 1..pos + 1 + 4]);
            break;
        }
        res.extend_from_slice(&bytes[pos + 1..pos + 1 + 4]);
        pos += 5;
    }
    let len = pos + 5;
    (convert(res.as_slice()), len)
}

fn parse_packet(packet: &[char]) -> (Packet, usize) {
    let version = convert(&packet[0..3]);
    let type_id = convert(&packet[3..6]);
    if type_id == 4 {
        let (v, l) = parse_number(&packet[6..]);
        let p = Packet {
            version,
            type_id,
            operators: Vec::new(),
            v,
        };
        (p, 6 + l)
    } else {
        // operator
        let length_type_id = convert(&packet[6..7]);
        if length_type_id == 0 {
            let length_subpackets = convert(&packet[7..22]);
            let mut ops_length = 0;
            let mut operators = Vec::new();
            while ops_length != length_subpackets {
                let (packet, len) = parse_packet(&packet[22 + ops_length..]);
                operators.push(packet);
                ops_length += len;
            }
            let p = Packet {
                version,
                type_id,
                operators,
                v: 0,
            };
            (p, 22 + ops_length)
        } else {
            let number_subpackets = convert(&packet[7..18]);
            let mut ops_length = 0;
            let mut subpackets = 0;
            let mut operators = Vec::new();
            while subpackets != number_subpackets {
                let (packet, len) = parse_packet(&packet[18 + ops_length..]);
                ops_length += len;
                operators.push(packet);
                subpackets += 1;
            }
            let p = Packet {
                version,
                type_id,
                operators,
                v: 0,
            };
            (p, 18 + ops_length)
        }
    }
}

fn sum_version(packet: &Packet) -> usize {
    packet.version
        + packet
            .operators
            .iter()
            .fold(0, |acc, p| acc + sum_version(&p))
}

pub fn part1(input: String) {
    let pkts = convert_to_binary_from_hex(input.as_str());
    let res = parse_packet(pkts.as_slice());
    println!("Solution part 1: {:?} ", sum_version(&res.0));
}

fn calc(p: &Packet) -> usize {
    match p.type_id {
        0 => p.operators.iter().fold(0, |acc, c| acc + calc(c)),
        1 => p.operators.iter().fold(1, |acc, c| acc * calc(c)),
        2 => p.operators.iter().map(|c| calc(c)).min().unwrap(),
        3 => p.operators.iter().map(|c| calc(c)).max().unwrap(),
        4 => p.v,
        5 => {
            if calc(&p.operators[0]) > calc(&p.operators[1]) {
                1
            } else {
                0
            }
        }
        6 => {
            if calc(&p.operators[0]) < calc(&p.operators[1]) {
                1
            } else {
                0
            }
        }
        7 => {
            if calc(&p.operators[0]) == calc(&p.operators[1]) {
                1
            } else {
                0
            }
        }
        _ => panic!(),
    }
}

pub fn part2(input: String) {
    let pkts = convert_to_binary_from_hex(input.as_str());
    let res = parse_packet(pkts.as_slice());
    println!("Solution part 2: {:?} ", calc(&res.0));
}

#[cfg(test)]
mod tests {

    use super::*;
    use itertools::Itertools;

    #[test]
    fn day16_part1_value() {
        let packetstr = "110100101111111000101000".chars().collect_vec();
        let (packet, len) = parse_packet(&packetstr);
        assert_eq!(6, packet.version);
        assert_eq!(4, packet.type_id);
        assert_eq!(2021, packet.v);
        assert_eq!(len, 21);
    }

    #[test]
    fn day16_part1_operator_1() {
        let packetstr = "00111000000000000110111101000101001010010001001000000000"
            .chars()
            .collect_vec();
        let (packet, len) = parse_packet(&packetstr);
        assert_eq!(1, packet.version);
        assert_eq!(6, packet.type_id);
        assert_eq!(0, packet.v);
        let operators = packet.operators;
        assert_eq!(6, operators[0].version);
        assert_eq!(4, operators[0].type_id);
        assert_eq!(10, operators[0].v);
        assert_eq!(2, operators[1].version);
        assert_eq!(4, operators[1].type_id);
        assert_eq!(20, operators[1].v);
        assert_eq!(len, 49);
    }

    #[test]
    fn day16_part1_operator_2() {
        let packetstr = "11101110000000001101010000001100100000100011000001100000"
            .chars()
            .collect_vec();
        let (packet, len) = parse_packet(&packetstr);
        assert_eq!(7, packet.version);
        assert_eq!(3, packet.type_id);
        assert_eq!(0, packet.v);
        let operators = packet.operators;
        assert_eq!(3, operators.len());
        assert_eq!(4, operators[0].type_id);
        assert_eq!(1, operators[0].v);
        assert_eq!(4, operators[1].type_id);
        assert_eq!(2, operators[1].v);
        assert_eq!(4, operators[1].type_id);
        assert_eq!(3, operators[2].v);
        assert_eq!(4, operators[2].type_id);
        assert_eq!(len, 51);
    }

    fn day16_part1_operator_3() {
        let packetstr =
            "00111000000000000110111101000101001010010001001000000000000000000000000000000"
                .chars()
                .collect_vec();
        let (packet, len) = parse_packet(&packetstr);
        assert_eq!(1, packet.version);
        assert_eq!(6, packet.type_id);
        assert_eq!(0, packet.v);
        let operators = packet.operators;
        assert_eq!(6, operators[0].version);
        assert_eq!(4, operators[0].type_id);
        assert_eq!(10, operators[0].v);
        assert_eq!(2, operators[1].version);
        assert_eq!(4, operators[1].type_id);
        assert_eq!(20, operators[1].v);
        assert_eq!(len, 49);
    }
}
