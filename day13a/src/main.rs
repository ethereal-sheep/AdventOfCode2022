#![feature(iter_intersperse)]
use std::fmt::Display;

enum PacketData {
    Value(i32),
    List(Vec<PacketData>),
}

impl PacketData {
    fn new(it: &mut impl Iterator<Item = char>) -> Self {
        let mut v = Vec::<PacketData>::new();
        let mut curr = String::new();
        while let Some(c) = it.next() {
            match c {
                '[' => v.push(PacketData::new(it)),
                ']' => break,
                ',' => {
                    curr.drain(0..)
                        .collect::<String>()
                        .parse::<i32>()
                        .ok()
                        .and_then(|i| {
                            v.push(PacketData::Value(i));
                            Some(i)
                        });
                }
                _ => curr.push(c),
            }
        }
        curr.parse::<i32>().ok().and_then(|i| {
            v.push(PacketData::Value(i));
            Some(i)
        });
        PacketData::List(v)
    }

    fn compare(&self, rhs: &Self) -> bool {
        false
    }
}

impl Display for PacketData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketData::Value(a) => write!(f, "{}", a),
            PacketData::List(v) => write!(
                f,
                "[{}]",
                v.iter()
                    .map(|d| d.to_string())
                    .intersperse(",".to_string())
                    .collect::<String>()
            ),
        }
    }
}

fn main() {
    let s = include_str!("../input.txt");
    let v = s
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|s| PacketData::new(&mut s.chars().into_iter().skip(1)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    v.iter()
        .for_each(|v| v.iter().for_each(|d| println!("{}", d)));
}
