use itertools::Itertools;

#[derive(Debug)]
struct Packet {
    version: usize,
    packet_type: PacketType
}

#[derive(Debug)]
enum PacketType {
    Operator(u8, Vec<Packet>),
    Literal(usize)
}

impl Packet {
    pub fn value(&self) -> usize {
        match self.packet_type {
            PacketType::Literal(v) => v,
            PacketType::Operator(id, ref sub_packets) => {
                match id {
                    0 => {
                        let mut val = 0;
                        self.run(|p, val| *val += p.value(), &mut val);
                        val
                    },
                    1 => {
                        let mut val = 1;
                        self.run(|p, val| *val *= p.value(), &mut val);
                        val
                    },
                    2 => {
                        let mut val = usize::MAX;
                        self.run(|p, val| *val = (*val).min(p.value()), &mut val);
                        val
                    },
                    3 => {
                        let mut val = 0;
                        self.run(|p, val| *val = (*val).max(p.value()), &mut val);
                        val
                    },
                    5 => if sub_packets[0].value() > sub_packets[1].value() { 1 } else { 0 },
                    6 => if sub_packets[0].value() < sub_packets[1].value() { 1 } else { 0 },
                    7 => if sub_packets[0].value() == sub_packets[1].value() { 1 } else { 0 },
                    _ => unreachable!()
                }
            }
        }
    }

    pub fn run(&self, f: fn(&Self, &mut usize), val: &mut usize)  {
        if let PacketType::Operator(_, ref v) = self.packet_type {
            for p in v {
                (f)(p, val);
            }
        }
    }
}

#[aoc_generator(day16)]
fn parse(input: &str) -> String {
    let mut formatted = String::with_capacity(input.len() * 4);
    for c in input.chars() {
        formatted += &format!("{:04b}", c.to_digit(16).unwrap());
    }
    formatted
}

#[aoc(day16, part1)]
fn part1(mut input: &str) -> usize {
    let packet = read_packet(&mut input);
    let mut count = 0;
    count_ver(&packet, &mut count);
    count
}

#[aoc(day16, part2)]
fn part2(mut input: &str) -> usize {
    let packet = read_packet(&mut input);
    packet.value()
}

fn read_packet(input: &mut &str) -> Packet {
    let ver = usize::from_str_radix(&input[0..3], 2).unwrap();
    let type_id = usize::from_str_radix(&input[3..6], 2).unwrap();
    *input = &input[6..];
    match type_id {
        4 => {
            let mut found = false;
            let num = input.chars()
                .tuples()
                .filter_map(|(a, b, c, d, e)| {
                    if found {
                        return None;
                    }
                    if a == '0' {
                        found = true;
                    }
                    Some([b, c, d, e])
                })
                .fuse()
                .flatten()
                .join("");
            *input = &input[(num.len() + (num.len() / 4) as usize)..];
            let num = usize::from_str_radix(&num, 2).unwrap();
            Packet {version: ver, packet_type: PacketType::Literal(num)}
        },
        id => {
            let len_type_id = input[0..1].parse().unwrap();
            let mut read = vec![];
            match len_type_id {
                0 => {
                    // size
                    let size = usize::from_str_radix(&input[1..16], 2).unwrap();
                    let mut buf = &input[16..(16 + size)];
                    *input = &input[(16 + size)..];
                    while !buf.is_empty() {
                        read.push(read_packet(&mut buf));
                    }
                    Packet {version: ver, packet_type: PacketType::Operator(id as u8, read)}
                },
                1 => {
                    // length
                    let len = usize::from_str_radix(&input[1..12], 2).unwrap();
                    *input = &input[12..];
                    for _ in 0..len {
                        let packet = read_packet(input);
                        read.push(packet);
                    }
                    Packet {version: ver, packet_type: PacketType::Operator(id as u8, read)}
                },
                _ => unreachable!()
            }

        }
    }
}

fn count_ver(packet: &Packet, count: &mut usize) {
    *count += packet.version;
    if let PacketType::Operator(_, ref v) = packet.packet_type {
        for p in v {
            count_ver(p, count);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"A0016C880162017C3686B18A3D4780"#;
        assert_eq!(part1(&parse(input)), 31);
    }

    #[test]
    fn part2_example() {
        let input = r#"9C0141080250320F1802104A08"#;
        assert_eq!(part2(&parse(input)), 1);
    }
}
