use crate::PacketKind::{Literal, Operator};
use itertools::Itertools;

fn main() {
    let packet = PacketBuilder::from_hex(include_str!("input.txt")).get_next_packet();

    println!("part1 result: {}", packet.total_version());
    println!("part1 result: {}", packet.perform_calculation());
}

struct Packet {
    version: usize,
    kind: PacketKind,
    value: usize,
    sub_packets: Vec<Packet>,
    total_bits: usize,
}

enum PacketKind {
    Literal,
    Operator,
}

impl Packet {
    pub fn total_version(&self) -> usize {
        self.version
            + self
                .sub_packets
                .iter()
                .map(|p| p.total_version())
                .sum::<usize>()
    }

    pub fn perform_calculation(&self) -> usize {
        match self.kind {
            Literal => self.value,
            Operator => {
                let mut sub_results = self.sub_packets.iter().map(|p| p.perform_calculation());
                match self.value {
                    0 => sub_results.sum(),
                    1 => sub_results.product(),
                    2 => sub_results.min().unwrap(),
                    3 => sub_results.max().unwrap(),
                    5 => {
                        if sub_results.next().unwrap() > sub_results.next().unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        if sub_results.next().unwrap() < sub_results.next().unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        if sub_results.next().unwrap() == sub_results.next().unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    x => panic!("Unexpected operator value: {}", x),
                }
            }
        }
    }
}

struct PacketBuilder {
    bits: Box<dyn Iterator<Item = char>>,
}

impl PacketBuilder {
    pub fn from_hex(hex: &'static str) -> Self {
        let raw_bits = hex.chars().filter(|c| c.is_ascii_hexdigit()).flat_map(|c| {
            format!("{:04b}", c.to_digit(16).unwrap())
                .chars()
                .collect_vec()
        });
        PacketBuilder {
            bits: Box::new(raw_bits),
        }
    }

    pub fn get_next_packet(&mut self) -> Packet {
        let version = self.parse_as_numeric(3);
        let kind_value = self.parse_as_numeric(3);

        let kind;
        let value;
        let sub_packets;
        let total_bits;
        if kind_value == 4 {
            kind = Literal;
            let parse_result = self.parse_literal_value();
            value = parse_result.0;
            let literal_bits = parse_result.1;
            sub_packets = vec![];
            total_bits = literal_bits + LITERAL_PREAMBLE_BITS;
        } else {
            kind = Operator;
            value = kind_value;
            let preamble_bits = match self.bits.next().unwrap() {
                '0' => {
                    sub_packets = self.get_sub_packets_z();
                    LT0_PREAMBLE_BITS
                }
                '1' => {
                    sub_packets = self.get_sub_packets_o();
                    LT1_PREAMBLE_BITS
                }
                _ => panic!("Unexpected bit!"),
            };
            total_bits = sub_packets.iter().map(|p| p.total_bits).sum::<usize>() + preamble_bits;
        }

        Packet {
            version,
            kind,
            value,
            sub_packets,
            total_bits,
        }
    }

    fn get_sub_packets_z(&mut self) -> Vec<Packet> {
        let packet_bits = self.parse_as_numeric(15);
        let mut sub_packets = vec![];
        let mut bit_count = 0;
        while bit_count < packet_bits {
            let packet = self.get_next_packet();
            bit_count += packet.total_bits;
            sub_packets.push(packet);
        }
        sub_packets
    }

    fn get_sub_packets_o(&mut self) -> Vec<Packet> {
        let packet_count = self.parse_as_numeric(11);
        (0..packet_count)
            .map(|_| self.get_next_packet())
            .collect_vec()
    }

    fn parse_as_numeric(&mut self, bit_count: usize) -> usize {
        usize::from_str_radix(
            &(0..bit_count)
                .filter_map(|_| self.bits.next())
                .collect::<String>(),
            2,
        )
        .unwrap()
    }

    fn parse_literal_value(&mut self) -> (usize, usize) {
        let mut result = vec![];
        let mut bit_count = 0;

        loop {
            bit_count += 5;
            let is_last_chunk = self.bits.next().unwrap() == '0';
            result.extend((0..4).map(|_| self.bits.next().unwrap()));
            if is_last_chunk {
                break;
            }
        }

        (
            usize::from_str_radix(&result.into_iter().collect::<String>(), 2).unwrap(),
            bit_count,
        )
    }
}

const LITERAL_PREAMBLE_BITS: usize = 6;
const LT0_PREAMBLE_BITS: usize = 6 + 1 + 15;
const LT1_PREAMBLE_BITS: usize = 6 + 1 + 11;
