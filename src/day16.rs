mod utils;
use utils::BinaryStreamIterator;

#[derive(Debug)]
enum PacketType {
    Literal,
    Sum,
    Product,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

#[derive(Debug)]
struct Packet {
    version: u64,
    packet_type: PacketType,
    sub_packets: Vec<Packet>,
    value: u64,
}

fn main() {
    // let input = _get_test_input();
    let input = _get_input();

    // ----------- Parse Input -----------

    let parsed: Vec<u8> = input
        .trim()
        .chars()
        .flat_map(|c| match c {
            '0' => [0, 0, 0, 0],
            '1' => [0, 0, 0, 1],
            '2' => [0, 0, 1, 0],
            '3' => [0, 0, 1, 1],
            '4' => [0, 1, 0, 0],
            '5' => [0, 1, 0, 1],
            '6' => [0, 1, 1, 0],
            '7' => [0, 1, 1, 1],
            '8' => [1, 0, 0, 0],
            '9' => [1, 0, 0, 1],
            'A' => [1, 0, 1, 0],
            'B' => [1, 0, 1, 1],
            'C' => [1, 1, 0, 0],
            'D' => [1, 1, 0, 1],
            'E' => [1, 1, 1, 0],
            'F' => [1, 1, 1, 1],
            _ => [0, 0, 0, 0],
        })
        .collect();

    // ----------- Solve -----------

    fn bit_vec_to_num<'a>(bitvec: &'a [u8]) -> u64 {
        bitvec
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, n)| acc + ((*n as u64) << i))
    }
    fn parse_packets<'a>(mut iter: &mut BinaryStreamIterator<'a>) -> Packet {
        let version = bit_vec_to_num(&iter.take(3));
        let packet_type = match bit_vec_to_num(&iter.take(3)) {
            0 => PacketType::Sum,
            1 => PacketType::Product,
            2 => PacketType::Min,
            3 => PacketType::Max,
            4 => PacketType::Literal,
            5 => PacketType::Gt,
            6 => PacketType::Lt,
            7 => PacketType::Eq,
            _ => panic!(),
        };

        let value;
        let mut sub_packets = vec![];
        match packet_type {
            PacketType::Literal => {
                let mut bytes: Vec<u8> = vec![];
                while iter.next() == 1 {
                    bytes.extend(iter.take(4).iter().map(|n| *n));
                }
                bytes.extend(iter.take(4).iter().map(|n| *n));
                value = bit_vec_to_num(&bytes);
            }
            _ => {
                let length_type_id = iter.next();
                match length_type_id {
                    0 => {
                        let num_subpacket_digits = bit_vec_to_num(iter.take(15)) as usize;
                        let starting_index = iter.index;
                        while iter.index < starting_index + num_subpacket_digits {
                            sub_packets.push(parse_packets(&mut iter));
                        }
                    }
                    1 => {
                        let num_subpackets = bit_vec_to_num(iter.take(11));
                        for _ in 0..num_subpackets {
                            sub_packets.push(parse_packets(&mut iter));
                        }
                    }
                    _ => {}
                }

                match packet_type {
                    PacketType::Sum => {
                        value = sub_packets.iter().fold(0, |acc, p| acc + p.value);
                    }
                    PacketType::Product => {
                        value = sub_packets.iter().fold(1, |acc, p| acc * p.value);
                    }
                    PacketType::Min => {
                        value = sub_packets.iter().map(|p| p.value).min().unwrap();
                    }
                    PacketType::Max => {
                        value = sub_packets.iter().map(|p| p.value).max().unwrap();
                    }
                    PacketType::Gt => {
                        value = if sub_packets[0].value > sub_packets[1].value {
                            1
                        } else {
                            0
                        }
                    }
                    PacketType::Lt => {
                        value = if sub_packets[0].value < sub_packets[1].value {
                            1
                        } else {
                            0
                        }
                    }
                    PacketType::Eq => {
                        value = if sub_packets[0].value == sub_packets[1].value {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!(),
                }
            }
        };

        Packet {
            version,
            packet_type,
            sub_packets,
            value,
        }
    }

    fn sum_packet_versions(packet: &Packet) -> u64 {
        packet.version
            + packet
                .sub_packets
                .iter()
                .fold(0, |acc, packet| acc + sum_packet_versions(packet))
    }

    let mut iter = BinaryStreamIterator::new(&parsed);
    let root = parse_packets(&mut iter);
    let p1 = sum_packet_versions(&root);
    let p2 = root.value;

    // ----------- Print -----------

    println!("Part 1: {:?}", p1);
    println!("Part 2: {:?}", p2);
}

fn _get_test_input() -> String {
    return "

04005AC33890

"
    .to_string();
}

fn _get_input() -> String {
    return "

E0529D18025800ABCA6996534CB22E4C00FB48E233BAEC947A8AA010CE1249DB51A02CC7DB67EF33D4002AE6ACDC40101CF0449AE4D9E4C071802D400F84BD21CAF3C8F2C35295EF3E0A600848F77893360066C200F476841040401C88908A19B001FD35CCF0B40012992AC81E3B980553659366736653A931018027C87332011E2771FFC3CEEC0630A80126007B0152E2005280186004101060C03C0200DA66006B8018200538012C01F3300660401433801A6007380132DD993100A4DC01AB0803B1FE2343500042E24C338B33F5852C3E002749803B0422EC782004221A41A8CE600EC2F8F11FD0037196CF19A67AA926892D2C643675A0C013C00CC0401F82F1BA168803510E3942E969C389C40193CFD27C32E005F271CE4B95906C151003A7BD229300362D1802727056C00556769101921F200AC74015960E97EC3F2D03C2430046C0119A3E9A3F95FD3AFE40132CEC52F4017995D9993A90060729EFCA52D3168021223F2236600ECC874E10CC1F9802F3A71C00964EC46E6580402291FE59E0FCF2B4EC31C9C7A6860094B2C4D2E880592F1AD7782992D204A82C954EA5A52E8030064D02A6C1E4EA852FE83D49CB4AE4020CD80272D3B4AA552D3B4AA5B356F77BF1630056C0119FF16C5192901CEDFB77A200E9E65EAC01693C0BCA76FEBE73487CC64DEC804659274A00CDC401F8B51CE3F8803B05217C2E40041A72E2516A663F119AC72250A00F44A98893C453005E57415A00BCD5F1DD66F3448D2600AC66F005246500C9194039C01986B317CDB10890C94BF68E6DF950C0802B09496E8A3600BCB15CA44425279539B089EB7774DDA33642012DA6B1E15B005C0010C8C917A2B880391160944D30074401D845172180803D1AA3045F00042630C5B866200CC2A9A5091C43BBD964D7F5D8914B46F040





"
    .to_string();
}
