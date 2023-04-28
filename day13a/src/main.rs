#![feature(iter_intersperse, let_chains)]
use std::fmt::Display;

#[derive(Debug, Clone, Eq)]
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
}

impl PartialEq for PacketData {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let PacketData::Value(i) = self && let PacketData::Value(j) = other {
            i.cmp(j)
        } else if let PacketData::List(v1) = self && let PacketData::List(v2) = other {
            
            let n1 = v1.len();
            let n2 = v2.len();
            let n = std::cmp::min(n1, n2);

            for i in 0..n {
                match v1[i].cmp(&v2[i]) {
                    std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                    std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                    _ => ()
                }
            }
            n1.cmp(&n2)
        } else if let PacketData::Value(_) = self {
            let promote = PacketData::List(vec![self.clone()]);
            promote.cmp(other)
            
        } else if let PacketData::Value(_) = other {
            let promote = PacketData::List(vec![other.clone()]);
            self.cmp(&promote)
        } else {
            std::cmp::Ordering::Equal
        }
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
    let ans: _ = s
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|s| PacketData::new(&mut s.chars().into_iter().skip(1)))
                .collect::<Vec<_>>()
        }).enumerate().filter_map(|(i, v)| if v[0] < v[1] {
            Some(i + 1)
        } else {
            None
        })
        .sum::<usize>();
    
    println!("{}", ans);
}
